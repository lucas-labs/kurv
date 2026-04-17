## Projects

kurv is a lightweight Rust process manager (pm2-inspired). The same binary is both a **server** (daemon that spawns/monitors child processes) and a **CLI client** (talks to the server over TCP). Managed processes are called **eggs** 🥚.

## Commands

The project uses [Taskfile](https://taskfile.dev) (`task ...`). Core workflows:

- `task dev` — run server with cargo-watch + `--force` (uses `+stable`)
- `task dev:nowatch` — run server once
- `task build` / `task build:release` — debug/release build; both run `python check_size.py` afterward to detect binary-size regressions against `.task/` cache
- `task check` — runs `fmt:check` + `lint:check` (what CI runs)
- `task fmt` — **requires `+nightly`** (unstable rustfmt options in `rustfmt.toml`: `imports_granularity`, `imports_indent`)
- `task lint` — `cargo clippy --workspace --all-targets --all-features --fix --allow-staged`
- `task local-ci-release` / `task local-ci-pub` — run CI workflows locally via [nektosact](https://nektosact.com)

**Tests:** plain cargo. Single test: `cargo test --test unit_tests <filter>`. Integration test entry `tests/unit_tests.rs` → `tests/unit/`. `tests/integration/` exists but is empty.

**Running the server manually:** set `KURV_SERVER=true` or pass `--force`, otherwise the binary refuses to start as a server (safety measure — same binary is also the client).

## Architecture

Three layers + shared utilities:

- [src/cli/](src/cli/) — arg parsing (`pico-args`), themed terminal output, subcommand dispatch in [src/cli/mod.rs](src/cli/mod.rs). Subcommands live in [src/cli/cmd/](src/cli/cmd/); each client subcommand calls the running server via the `Api` TCP client in [src/cli/cmd/api/mod.rs](src/cli/cmd/api/mod.rs).
- [src/kurv/](src/kurv/) — server core. [Kurv::run()](src/kurv/mod.rs) is the main loop (500ms tick): `spawn_all` → `check_running_eggs` → `check_stopped_eggs` → `check_removal_pending_eggs` → `check_unsynced_eggs`. Each returns an `unsynced` flag; state is only flushed to disk when something changed.
- [src/api/](src/api/) — runs on its own thread (spawned from [src/main.rs](src/main.rs)). Custom HTTP-like protocol over raw `TcpListener`; regex-routed in [src/api/mod.rs](src/api/mod.rs). No external HTTP framework.
- [src/common/](src/common/) — `Info` (paths/config), custom TCP request/response types in [src/common/tcp/mod.rs](src/common/tcp/mod.rs), logger, theme-aware `printth!` macro.

### Egg state machine

Defined in [src/kurv/egg/mod.rs](src/kurv/egg/mod.rs):

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

Executables prefixed `kurv-` dropped into `<KURV_HOME>/plugins/` are auto-discovered at server start. Each is invoked with `--kurv-cfg`; it must print its egg config as JSON on stdout. See [src/kurv/plugins/mod.rs](src/kurv/plugins/mod.rs). Plugins get `KURV_API_HOST`, `KURV_API_PORT`, `KURV_HOME`, `KURV_LOGS_DIR` injected into their env. They behave like eggs but can't be removed via the CLI (stop the server and delete the file instead).

### State persistence

`.kurv` file at `KURV_HOME` (defaults to the executable's parent dir). Written as JSON; loader in [src/kurv/state/mod.rs](src/kurv/state/mod.rs) still falls back to YAML for legacy files. IDs are reassigned on every load — don't rely on stable IDs across restarts.

### Process spawning

Uses the `command-group` crate (`.group_spawn()`) so child process groups are torn down with the parent on all platforms. Stdio is redirected to `<KURV_HOME>/task_logs/{egg-name}.{stdout,stderr}`.

## Conventions

- **Output:** use the `printth!` macro with theme tags (`<error>`, `<warn>`, `<info>`, `<white>`, `<green>`, `<b>`, `<head>`, etc.). Parser + styles in [src/cli/color/](src/cli/color/). Don't use `println!` for user-facing output.
- **Errors:** `anyhow` everywhere; no custom error types.
- **Imports:** single `use { ... }` block per file (enforced by `rustfmt.toml`'s `imports_granularity = "One"`). Requires nightly rustfmt.
- **Adding a CLI subcommand:** add an arm in [src/cli/mod.rs](src/cli/mod.rs) `dispatch_command()` and a module under [src/cli/cmd/](src/cli/cmd/).
- **Adding an API route:** append to the `Router::routes()` Vec in [src/api/mod.rs](src/api/mod.rs) — order matters, fallback `(".*", ".*", err::not_allowed)` stays last.
- **Extending the main loop:** new checks return a `bool` indicating whether state changed, then OR into `unsynced` — never write state unconditionally.

## Environment variables

- `KURV_SERVER=true` — required to start as server (or pass `--force`)
- `KURV_HOME` — overrides `.kurv` / `plugins/` / `task_logs/` parent dir
- `KURV_API_HOST` / `KURV_API_PORT` — API bind address (default `127.0.0.1:58787`)
- `KURV_LOGS_DIR` — override egg log dir (default `<KURV_HOME>/task_logs`)
- `KURV_VERSION_NAME` — compile-time release codename (read via `option_env!`)

## Notes

- Editing github issue threads: don't leave comments on issues unless asked.
- Don't create new markdown docs unless explicitly requested.
- Commit style: uses [coco](https://github.com/lucas-labs/coco) (`.cocorc` → `useEmoji: true`, `askScope: false`). Existing commits follow `type: 🔖 message` (e.g. `ci: 🔄 ...`, `docs: 📝 ...`).
