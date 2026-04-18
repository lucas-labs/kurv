"""
interactive release-commit helper for the kurv workspace.

walks:
  1. discover workspace members via `cargo metadata`
  2. ask which scope to release (numbered prompt)
  3. show current version, prompt for next (default: patch bump)
  4. rewrite the [package].version line in the selected crate's Cargo.toml
  5. stage + commit as `release(<scope>): 🔖 v<version>`

the commit is NOT pushed — review it manually before `git push`.

no external deps — stdlib only.
"""

import json
import pathlib
import re
import subprocess
import sys


SEMVER_RE = re.compile(r'^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?$')
VERSION_LINE_RE = re.compile(r'^(\s*version\s*=\s*")[^"]+(".*)$')
SECTION_RE = re.compile(r'^\s*\[([^\]]+)\]\s*$')


def bail(msg):
    print(f'error: {msg}', file=sys.stderr)
    sys.exit(1)


def cancel():
    print('\nrelease cancelled.', file=sys.stderr)
    sys.exit(130)


def list_workspace_members():
    try:
        out = subprocess.check_output(
            ['cargo', 'metadata', '--format-version', '1', '--no-deps'],
            text=True,
        )
    except FileNotFoundError:
        bail('cargo not found on PATH')
    except subprocess.CalledProcessError as e:
        bail(f'cargo metadata failed: {e}')

    meta = json.loads(out)
    ws_members = set(meta.get('workspace_members', []))
    members = []
    for pkg in meta.get('packages', []):
        if pkg.get('id') not in ws_members:
            continue
        manifest = pkg.get('manifest_path')
        if not manifest:
            continue
        # a crate is releasable iff it ships a .release.yml next to its Cargo.toml.
        # this is the same file publish.yml reads to expand its build matrix, so
        # the script and CI agree on what "releasable" means. `publish = false`
        # is not a valid signal — plugins set it because they ship via GitHub
        # releases, not crates.io, but they still need to be released.
        if not (pathlib.Path(manifest).parent / '.release.yml').is_file():
            continue
        members.append({
            'name': pkg['name'],
            'version': pkg['version'],
            'manifest_path': manifest,
        })

    if not members:
        bail('no releasable workspace members found (looked for .release.yml next to each Cargo.toml)')

    members.sort(key=lambda m: m['name'])
    return members


def pick_scope(members):
    print('\nreleasable workspace members:\n')
    for i, m in enumerate(members, 1):
        print(f'  {i}) {m["name"]:<30} (current: {m["version"]})')

    print()
    raw = input('select scope [number]: ').strip()
    if not raw.isdigit():
        bail('expected a number')
    idx = int(raw)
    if idx < 1 or idx > len(members):
        bail(f'out of range: pick 1..{len(members)}')

    return members[idx - 1]


def bump_patch(version):
    core = version.split('-', 1)[0].split('+', 1)[0]
    parts = core.split('.')
    if len(parts) != 3 or not all(p.isdigit() for p in parts):
        return version
    parts[2] = str(int(parts[2]) + 1)
    return '.'.join(parts)


def ask_next_version(current):
    default = bump_patch(current)
    raw = input(f'next version [{default}]: ').strip() or default
    if not SEMVER_RE.match(raw):
        bail(f'not a valid semver: {raw}')
    return raw


def rewrite_cargo_toml(manifest_path, new_version):
    """rewrite version = "..." but only inside the [package] section."""
    path = pathlib.Path(manifest_path)
    src = path.read_text(encoding='utf-8')
    out = []
    in_package = False
    replaced = False

    for line in src.splitlines(keepends=True):
        sec = SECTION_RE.match(line)
        if sec:
            in_package = sec.group(1).strip() == 'package'
            out.append(line)
            continue

        if in_package and not replaced:
            m = VERSION_LINE_RE.match(line.rstrip('\r\n'))
            if m:
                eol = line[len(line.rstrip('\r\n')):]
                out.append(f'{m.group(1)}{new_version}{m.group(2)}{eol}')
                replaced = True
                continue

        out.append(line)

    if not replaced:
        bail(f'could not find a version line in [package] of {manifest_path}')

    path.write_text(''.join(out), encoding='utf-8')


def git_commit(manifest_path, scope, new_version):
    subprocess.check_call(['git', 'add', manifest_path])
    msg = f'release({scope}): 🔖 v{new_version}'
    subprocess.check_call(['git', 'commit', '-m', msg])
    print(f'\n✓ committed: {msg}')
    print('  review with `git show`, then `git push` when ready.')


def main():
    members = list_workspace_members()
    chosen = pick_scope(members)
    new_version = ask_next_version(chosen['version'])

    if new_version == chosen['version']:
        bail(f'version unchanged ({new_version}) — aborting')

    rewrite_cargo_toml(chosen['manifest_path'], new_version)
    git_commit(chosen['manifest_path'], chosen['name'], new_version)


if __name__ == '__main__':
    try:
        main()
    except KeyboardInterrupt:
        cancel()
