## Projects

kurv = lightweight Rust process mgr (pm2-ish). Same binary = **server** (daemon spawn/monitor children) + **CLI client** (TCP). Managed procs called **eggs** 🥚.

## Workspace layout

**cargo workspace** (resolver 3, edition 2024). Root [Cargo.toml](Cargo.toml) owns `[workspace.package]` + `[profile.*]` — members inherit via `field.workspace = true`. Profiles **only** apply from root; member `[profile.*]` silently ignored.

- [app/kurv/](app/kurv/) — main `kurv` binary (server + CLI). All kurv deps live here, **not** in `[workspace.dependencies]`.
- [plugins/](plugins/) — official plugin crates, auto-globbed `members = ["plugins/*"]`. First: [plugins/kurv-ui/](plugins/kurv-ui/) (hello-world placeholder).
- [crates/kurv-plugin-sdk/](crates/kurv-plugin-sdk/) — shared types + entrypoint for plugins (`publish = false`). Exposes `PluginConfig`, `KurvEnv`, `start(configure, run_loop)` — see [crates/kurv-plugin-sdk/src/lib.rs](crates/kurv-plugin-sdk/src/lib.rs). Plugin `main`s delegate to `start`, no hand-rolled args.

`[workspace.dependencies]` = only genuinely shared deps. Promote upward when **second** crate needs it — no pre-promote.

## Commands

Uses [Taskfile](https://taskfile.dev) (`task ...`). Core:

- `task dev` — server w/ cargo-watch + `--force` (`+stable`, `-p kurv`)
- `task dev:nowatch` — server once
- `task build` / `task build:release` — debug/release **kurv only** (`-p kurv`); both run `python check_size.py` after to catch binary-size regressions vs `.task/` cache. Plugins skipped.
- `task check` — `fmt:check` + `lint:check` (CI runs this)
- `task fmt` — **needs `+nightly`** (unstable rustfmt opts in `rustfmt.toml`: `imports_granularity`, `imports_indent`)
- `task lint` — `cargo clippy --workspace --all-targets --all-features --fix --allow-staged`
- `task release` — runs `python scripts/release.py` (see [Release flow](#release-flow))
- `task local-ci-release` / `task local-ci-pub` — CI locally via [nektosact](https://nektosact.com)

**Tests:** plain cargo. Single: `cargo test -p kurv --test unit_tests <filter>`.

Note: no inline `#[cfg(test)]` modules in source; keep tests in crate-local `tests/` folders.

**Manual server:** set `KURV_SERVER=true` or pass `--force`, else binary refuses (safety — same binary is also client).

## Architecture

Three layers + shared utils inside `kurv` crate:

- [app/kurv/src/cli/](app/kurv/src/cli/) — arg parsing (`pico-args`), themed terminal output, dispatch in [app/kurv/src/cli/mod.rs](app/kurv/src/cli/mod.rs). Subcmds in [app/kurv/src/cli/cmd/](app/kurv/src/cli/cmd/); each calls server via `Api` TCP client in [app/kurv/src/cli/cmd/api/mod.rs](app/kurv/src/cli/cmd/api/mod.rs).
- [app/kurv/src/kurv/](app/kurv/src/kurv/) — server core. [Kurv::run()](app/kurv/src/kurv/mod.rs) = main loop (500ms tick): `spawn_all` → `check_running_eggs` → `check_stopped_eggs` → `check_removal_pending_eggs` → `check_unsynced_eggs`. Each returns `unsynced` flag; state flushed only when changed.
- [app/kurv/src/api/](app/kurv/src/api/) — own thread (spawned from [app/kurv/src/main.rs](app/kurv/src/main.rs)). Custom HTTP-ish over raw `TcpListener`; regex-routed in [app/kurv/src/api/mod.rs](app/kurv/src/api/mod.rs). No HTTP framework.
- [app/kurv/src/common/](app/kurv/src/common/) — `Info` (paths/config), TCP req/resp types in [app/kurv/src/common/tcp/mod.rs](app/kurv/src/common/tcp/mod.rs), logger, `printth!` macro.

### Egg state machine

In [app/kurv/src/kurv/egg/mod.rs](app/kurv/src/kurv/egg/mod.rs):

```
Pending → Running → (crash) → Errored → Pending (auto-retry)
                 ↓ (user stop)
                 Stopped → (user start) → Pending
                 ↓ (user remove)
                 PendingRemoval → removed from state
```

Boot: any egg persisted as `Running` reset to `Pending` so loop respawns.

### Shared state

```rust
type InfoMtx = Arc<Mutex<Info>>;
type KurvStateMtx = Arc<Mutex<KurvState>>;
```

Both cloned into API thread. Partial egg updates via `EggStateUpsert` — only `Some(_)` fields overwrite.

### Plugins

Executables prefixed `kurv-` dropped in `<KURV_HOME>/plugins/` = auto-discovered at server start. Invoked w/ `--kurv-cfg`; must print egg config as JSON to stdout. See [app/kurv/src/kurv/plugins/mod.rs](app/kurv/src/kurv/plugins/mod.rs). Plugins get `KURV_API_HOST`, `KURV_API_PORT`, `KURV_HOME`, `KURV_LOGS_DIR` in env. Behave like eggs but can't be removed via CLI (stop server + delete file).

Official plugins = crates under `plugins/*` in workspace, built on [`kurv-plugin-sdk`](crates/kurv-plugin-sdk/) — each `main` = one-liner delegating to `kurv_plugin_sdk::start(configure, run_loop)`. Reference: [plugins/kurv-ui/src/main.rs](plugins/kurv-ui/src/main.rs).

### State persistence

`.kurv` file at `KURV_HOME` (defaults to exe parent dir). Written as JSON; loader in [app/kurv/src/kurv/state/mod.rs](app/kurv/src/kurv/state/mod.rs) falls back to YAML for legacy. IDs reassigned every load — don't rely on stable IDs across restarts.

### Process spawning

Uses `command-group` crate (`.group_spawn()`) so child process groups die w/ parent on all platforms. Stdio → `<KURV_HOME>/task_logs/{egg-name}.{stdout,stderr}`.

## Release flow

Releases = **per-package** + **scope-driven**. One commit = one crate.

- **Commit format:** `release(<scope>): 🔖 v<version>` — scope **required**; unscoped `release:` rejected by CI. `<scope>` matches workspace member name (e.g. `kurv`, `kurv-ui`).
- **Tag format:** `<scope>-v<semver>` (e.g. `kurv-v0.2.0`, `kurv-ui-v0.0.2`).
- **Helper:** [scripts/release.py](scripts/release.py) (also `task release`) — interactive, stdlib-only. Discovers members via `cargo metadata`, prompts scope + version, rewrites `[package].version` in that crate's `Cargo.toml` (section-aware — won't touch `version = ...` under `[dependencies]`), then commits. Does **not** push.
- **Per-crate config:** each releasable crate ships `.release.yml` next to `Cargo.toml` declaring `crates_io: bool` + build matrix. See [app/kurv/.release.yml](app/kurv/.release.yml) (`crates_io: true`) + [plugins/kurv-ui/.release.yml](plugins/kurv-ui/.release.yml) (`crates_io: false` — plugins ship via GH releases only). `{scope}` placeholder in archive names = substituted at CI time.
- **CI pipeline:** [.github/workflows/ci.yml](.github/workflows/ci.yml) gates on `startsWith(head_commit.message, 'release(')` → delegates to [.github/workflows/draft-release.yml](.github/workflows/draft-release.yml), which resolves scope → crate path via `cargo metadata`, uses `git describe --match '<scope>-v*'` for scoped changelog, creates draft tagged `<scope>-v<version>`. When published, [.github/workflows/publish.yml](.github/workflows/publish.yml) reads crate's `.release.yml`, expands matrix dynamically, builds artifacts, conditionally publishes to crates.io when `crates_io: true`.

## Conventions

- **Output:** use `printth!` w/ theme tags (`<error>`, `<warn>`, `<info>`, `<white>`, `<green>`, `<b>`, `<head>`, etc.). Parser + styles in [app/kurv/src/cli/color/](app/kurv/src/cli/color/). No `println!` for user-facing output in kurv binary.
- **Errors:** `anyhow` everywhere; no custom error types.
- **Imports:** single `use { ... }` block per file (enforced by `rustfmt.toml` `imports_granularity = "One"`). Needs nightly rustfmt.
- **New CLI subcmd:** add arm in [app/kurv/src/cli/mod.rs](app/kurv/src/cli/mod.rs) `dispatch_command()` + module under [app/kurv/src/cli/cmd/](app/kurv/src/cli/cmd/).
- **New API route:** append to `Router::routes()` Vec in [app/kurv/src/api/mod.rs](app/kurv/src/api/mod.rs) — order matters, fallback `(".*", ".*", err::not_allowed)` stays last.
- **Extending main loop:** new checks return `bool` for state-changed, OR into `unsynced` — never write state unconditionally.
- **New plugin crate:** create `plugins/<name>/` (must start `kurv-` so loader picks up binary). Depend on `kurv-plugin-sdk = { workspace = true }`, wire `main` via `kurv_plugin_sdk::start`. Add `.release.yml` next to `Cargo.toml` (that file — not `publish = false` — marks crate releasable; `scripts/release.py` filters on it). Plugin-only deps stay in plugin's `Cargo.toml`, not workspace root.

## Environment variables

- `KURV_SERVER=true` — required to start as server (or `--force`)
- `KURV_HOME` — overrides `.kurv` / `plugins/` / `task_logs/` parent dir
- `KURV_API_HOST` / `KURV_API_PORT` — API bind addr (default `127.0.0.1:58787`)
- `KURV_LOGS_DIR` — override egg log dir (default `<KURV_HOME>/task_logs`)
- `KURV_VERSION_NAME` — compile-time release codename (via `option_env!`)

## Notes

- GH issue threads: no comments unless asked.
- No new markdown docs unless asked.
- Commit style: [coco](https://github.com/lucas-labs/coco) (`.cocorc` → `useEmoji: true`, `askScope: false`). Existing follow `type: 🔖 message` (e.g. `ci: 🔄 ...`, `feat: ✨ ...`, `docs: 📝 ...`). Release commits = exception — **require** scope, produced by `scripts/release.py`, not coco.