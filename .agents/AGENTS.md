## Projects

kurv is a lightweight Rust process manager (pm2-inspired). The same binary is both a **server** (daemon that spawns/monitors child processes) and a **CLI client** (talks to the server over TCP). Managed processes are called **eggs** 🥚.

## Workspace layout

This repo is a **cargo workspace** (resolver 3, edition 2024). Root [Cargo.toml](Cargo.toml) owns `[workspace.package]` metadata and `[profile.*]` — member `Cargo.toml`s inherit via `field.workspace = true`. Profiles **only** apply from the root; any `[profile.*]` in a member crate is silently ignored.

- [app/kurv/](app/kurv/) — the main `kurv` binary (server + CLI client). All kurv-specific deps live here, **not** in `[workspace.dependencies]`.
- [plugins/](plugins/) — official plugin crates, auto-globbed via `members = ["plugins/*"]`. First example: [plugins/kurv-ui/](plugins/kurv-ui/) (hello-world placeholder).
- [crates/kurv-plugin-sdk/](crates/kurv-plugin-sdk/) — shared types and entrypoint for plugin crates (`publish = false`). Exposes `PluginConfig`, `KurvEnv`, and `start(configure, run_loop)` — see [crates/kurv-plugin-sdk/src/lib.rs](crates/kurv-plugin-sdk/src/lib.rs). Plugin `main`s should delegate to `start` rather than hand-roll arg parsing.

`[workspace.dependencies]` only holds deps genuinely shared across crates. Promote a dep upward the first time a **second** crate needs it — don't pre-promote.

## Commands

The project uses [Taskfile](https://taskfile.dev) (`task ...`). Core workflows:

- `task dev` — run server with cargo-watch + `--force` (uses `+stable`, scoped to `-p kurv`)
- `task dev:nowatch` — run server once
- `task build` / `task build:release` — debug/release build of **kurv only** (`-p kurv`); both run `python check_size.py` afterward to detect binary-size regressions against `.task/` cache. Plugins are not built in the size-sensitive path.
- `task check` — runs `fmt:check` + `lint:check` (what CI runs)
- `task fmt` — **requires `+nightly`** (unstable rustfmt options in `rustfmt.toml`: `imports_granularity`, `imports_indent`)
- `task lint` — `cargo clippy --workspace --all-targets --all-features --fix --allow-staged`
- `task release` — runs `python scripts/release.py` (see [Release flow](#release-flow))
- `task local-ci-release` / `task local-ci-pub` — run CI workflows locally via [nektosact](https://nektosact.com)

**Tests:** plain cargo. Single test: `cargo test -p kurv --test unit_tests <filter>`. 

Note: Do not add inline `#[cfg(test)]` modules in source files; keep tests in crate-local `tests/` folders so business logic files stay focused.

**Running the server manually:** set `KURV_SERVER=true` or pass `--force`, otherwise the binary refuses to start as a server (safety measure — same binary is also the client).

## Architecture

Three layers + shared utilities inside the `kurv` crate:

- [app/kurv/src/cli/](app/kurv/src/cli/) — arg parsing (`pico-args`), themed terminal output, subcommand dispatch in [app/kurv/src/cli/mod.rs](app/kurv/src/cli/mod.rs). Subcommands live in [app/kurv/src/cli/cmd/](app/kurv/src/cli/cmd/); each client subcommand calls the running server via the `Api` TCP client in [app/kurv/src/cli/cmd/api/mod.rs](app/kurv/src/cli/cmd/api/mod.rs).
- [app/kurv/src/kurv/](app/kurv/src/kurv/) — server core. [Kurv::run()](app/kurv/src/kurv/mod.rs) is the main loop (500ms tick): `spawn_all` → `check_running_eggs` → `check_stopped_eggs` → `check_removal_pending_eggs` → `check_unsynced_eggs`. Each returns an `unsynced` flag; state is only flushed to disk when something changed.
- [app/kurv/src/api/](app/kurv/src/api/) — runs on its own thread (spawned from [app/kurv/src/main.rs](app/kurv/src/main.rs)). Custom HTTP-like protocol over raw `TcpListener`; regex-routed in [app/kurv/src/api/mod.rs](app/kurv/src/api/mod.rs). No external HTTP framework.
- [app/kurv/src/common/](app/kurv/src/common/) — `Info` (paths/config), custom TCP request/response types in [app/kurv/src/common/tcp/mod.rs](app/kurv/src/common/tcp/mod.rs), logger, theme-aware `printth!` macro.

### Egg state machine

Defined in [app/kurv/src/kurv/egg/mod.rs](app/kurv/src/kurv/egg/mod.rs):

```
Pending → Running → (crash) → Errored → Pending (auto-retry)
                 ↓ (user stop)
                 Stopped → (user start) → Pending
                 ↓ (user remove)
                 PendingRemoval → removed from state
```

On boot, any egg persisted as `Running` is reset to `Pending` so the main loop re-spawns it.

### Shared state

```rust
type InfoMtx = Arc<Mutex<Info>>;
type KurvStateMtx = Arc<Mutex<KurvState>>;
```

Both are cloned into the API thread. Partial egg updates go through `EggStateUpsert` — only fields set to `Some(_)` overwrite.

### Plugins

Executables prefixed `kurv-` dropped into `<KURV_HOME>/plugins/` are auto-discovered at server start. Each is invoked with `--kurv-cfg`; it must print its egg config as JSON on stdout. See [app/kurv/src/kurv/plugins/mod.rs](app/kurv/src/kurv/plugins/mod.rs). Plugins get `KURV_API_HOST`, `KURV_API_PORT`, `KURV_HOME`, `KURV_LOGS_DIR` injected into their env. They behave like eggs but can't be removed via the CLI (stop the server and delete the file instead).

Official plugins live as crates under `plugins/*` in this workspace and build on top of [`kurv-plugin-sdk`](crates/kurv-plugin-sdk/) — each plugin `main` is a one-liner delegating to `kurv_plugin_sdk::start(configure, run_loop)`. The reference implementation is [plugins/kurv-ui/src/main.rs](plugins/kurv-ui/src/main.rs).

### State persistence

`.kurv` file at `KURV_HOME` (defaults to the executable's parent dir). Written as JSON; loader in [app/kurv/src/kurv/state/mod.rs](app/kurv/src/kurv/state/mod.rs) still falls back to YAML for legacy files. IDs are reassigned on every load — don't rely on stable IDs across restarts.

### Process spawning

Uses the `command-group` crate (`.group_spawn()`) so child process groups are torn down with the parent on all platforms. Stdio is redirected to `<KURV_HOME>/task_logs/{egg-name}.{stdout,stderr}`.

## Release flow

Releases are **per-package** and **scope-driven**. One commit releases one crate.

- **Commit format:** `release(<scope>): 🔖 v<version>` — scope is **required**; unscoped `release:` commits are rejected by CI. `<scope>` must match a workspace member name (e.g. `kurv`, `kurv-ui`).
- **Tag format:** `<scope>-v<semver>` (e.g. `kurv-v0.2.0`, `kurv-ui-v0.0.2`).
- **Helper:** [scripts/release.py](scripts/release.py) (also exposed as `task release`) — interactive, stdlib-only. Discovers workspace members via `cargo metadata`, prompts for scope + new version, rewrites the `[package].version` line in that crate's `Cargo.toml` (section-aware — won't touch `version = ...` entries under `[dependencies]`), then commits. Does **not** push.
- **Per-crate release config:** each releasable crate ships a `.release.yml` alongside its `Cargo.toml` declaring `crates_io: bool` and a build matrix. See [app/kurv/.release.yml](app/kurv/.release.yml) (`crates_io: true`) and [plugins/kurv-ui/.release.yml](plugins/kurv-ui/.release.yml) (`crates_io: false` — plugins ship via GitHub releases only). The `{scope}` placeholder in archive names is substituted at CI time.
- **CI pipeline:** [.github/workflows/ci.yml](.github/workflows/ci.yml) gates on `startsWith(head_commit.message, 'release(')` → delegates to [.github/workflows/draft-release.yml](.github/workflows/draft-release.yml), which resolves scope → crate path via `cargo metadata`, uses `git describe --match '<scope>-v*'` for scoped changelog, and creates a draft release tagged `<scope>-v<version>`. When published, [.github/workflows/publish.yml](.github/workflows/publish.yml) reads that crate's `.release.yml`, expands the matrix dynamically, builds artifacts, and conditionally publishes to crates.io when `crates_io: true`.

## Conventions

- **Output:** use the `printth!` macro with theme tags (`<error>`, `<warn>`, `<info>`, `<white>`, `<green>`, `<b>`, `<head>`, etc.). Parser + styles in [app/kurv/src/cli/color/](app/kurv/src/cli/color/). Don't use `println!` for user-facing output in the kurv binary.
- **Errors:** `anyhow` everywhere; no custom error types.
- **Imports:** single `use { ... }` block per file (enforced by `rustfmt.toml`'s `imports_granularity = "One"`). Requires nightly rustfmt.
- **Adding a CLI subcommand:** add an arm in [app/kurv/src/cli/mod.rs](app/kurv/src/cli/mod.rs) `dispatch_command()` and a module under [app/kurv/src/cli/cmd/](app/kurv/src/cli/cmd/).
- **Adding an API route:** append to the `Router::routes()` Vec in [app/kurv/src/api/mod.rs](app/kurv/src/api/mod.rs) — order matters, fallback `(".*", ".*", err::not_allowed)` stays last.
- **Extending the main loop:** new checks return a `bool` indicating whether state changed, then OR into `unsynced` — never write state unconditionally.
- **Adding a new plugin crate:** create `plugins/<name>/` (must start with `kurv-` so the loader picks up the binary). Depend on `kurv-plugin-sdk = { workspace = true }` and wire `main` via `kurv_plugin_sdk::start`. Add a `.release.yml` next to its `Cargo.toml` (that file — not `publish = false` — is what marks the crate as releasable; `scripts/release.py` filters on it). Keep plugin-only deps in the plugin's `Cargo.toml`, not the workspace root.

## Environment variables

- `KURV_SERVER=true` — required to start as server (or pass `--force`)
- `KURV_HOME` — overrides `.kurv` / `plugins/` / `task_logs/` parent dir
- `KURV_API_HOST` / `KURV_API_PORT` — API bind address (default `127.0.0.1:58787`)
- `KURV_LOGS_DIR` — override egg log dir (default `<KURV_HOME>/task_logs`)
- `KURV_VERSION_NAME` — compile-time release codename (read via `option_env!`)

## Notes

- Editing github issue threads: don't leave comments on issues unless asked.
- Don't create new markdown docs unless explicitly requested.
- Commit style: uses [coco](https://github.com/lucas-labs/coco) (`.cocorc` → `useEmoji: true`, `askScope: false`). Existing commits follow `type: 🔖 message` (e.g. `ci: 🔄 ...`, `docs: 📝 ...`). Release commits are the exception — they **require** a scope and are produced by `scripts/release.py`, not coco.
