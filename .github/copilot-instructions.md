# Kurv - Process Manager Copilot Instructions

## Project Overview

Kurv is a lightweight Rust process manager inspired by pm2, designed to daemonize and monitor
applications (called "eggs" 🥚). The same binary functions as both server and CLI client,
communicating via a REST-like TCP protocol.

**Key Concepts:**

-   **Egg**: A managed process/application with configuration (command, args, cwd, env vars)
-   **Server mode**: Long-running daemon that spawns and monitors eggs
-   **Client mode**: CLI that sends commands to the server via TCP (default: 127.0.0.1:58787)
-   **State persistence**: `.kurv` file (JSON) stores all egg configurations and states

## Architecture

### Three-Layer Structure

1. **CLI Layer** (`src/cli/`): Command parsing, user output, themed terminal formatting
2. **Core Layer** (`src/kurv/`): Server logic, egg lifecycle management, process spawning
3. **API Layer** (`src/api/`): TCP server with custom HTTP-like protocol for client-server communication


> Note: And also, **Common Utilities** (`src/common/`): shared helpers for logging, TCP, theming,
> etc, used across all layers.

### Key Workflows

**Server Startup:**

```rust
// main.rs checks KURV_SERVER=true or --force flag before running server
Kurv::collect() -> loads .kurv state, marks running eggs as Pending
-> spawns API server thread
-> runs main loop (500ms tick) checking egg states
```

**Main Loop** (`src/kurv/mod.rs:run()`):

-   `spawn_all()`: Start eggs with status `Pending` or `Errored`
-   `check_running_eggs()`: Detect crashed processes, update status
-   `check_stopped_eggs()`: Kill eggs marked as `Stopped`
-   `check_removal_pending_eggs()`: Remove eggs with `PendingRemoval` status
-   Writes state to disk only when changes detected

**Egg Lifecycle States** (`src/kurv/egg/mod.rs`):

```
Pending -> Running -> (crash) -> Errored -> Pending (retry)
         ↓ (user stop)
         Stopped -> (user start) -> Pending
         ↓ (user remove)
         PendingRemoval -> (removed from state)
```

## Code Conventions

### Custom Macros and Patterns

**`printth!` macro**: Theme-aware terminal output with HTML-like tags

```rust
// src/cli/color/theme.rs - custom HTML parser applies crossterm styles
printth!("<error>Failed</error> to start <white>{}</white>", egg_name);
// Renders: styled "Failed" in red+bold, egg_name in white
```

**Theme tags** defined in `src/cli/color/mod.rs:get_theme()`:

-   `<error>`, `<warn>`, `<info>`, `<debug>`, `<trace>` - log levels
-   `<white>`, `<green>`, `<yellow>`, `<magenta>`, `<blue>` - colors
-   `<b>` - bold, `<dim>` - dimmed, `<head>` - section headers

### State Management

**Thread-safe shared state:**

```rust
type InfoMtx = Arc<Mutex<Info>>;      // App metadata, paths, PID
type KurvStateMtx = Arc<Mutex<KurvState>>; // Eggs collection
```

**Egg state updates** use partial update pattern (`EggStateUpsert` in `src/kurv/egg/mod.rs:update_state()`):

```rust
egg.update_state(EggStateUpsert {
    status: Some(EggStatus::Running),
    pid: Some(12345),
    start_time: Some(Local::now()),
    ..Default::default() // leaves other fields unchanged
});
```

### Process Management

**Workers pool** (`src/kurv/workers.rs`): BTreeMap tracking spawned processes

```rust
// Indexed by: group -> worker_id -> (egg_id, GroupChild)
// Default group: "default_kurv"
workers.add_child(None, egg.name.clone(), egg.id.unwrap(), child);
```

**Spawning** uses `command-group` crate for cross-platform process group management:

```rust
// src/kurv/spawn.rs - handles stdio redirection to log files
let stdout = create_log_file_handles(&egg, &info)?;
Command::new(&egg.command)
    .group_spawn() // kills child processes when parent dies
```

### API/TCP Protocol

**Custom HTTP-like protocol** (`src/common/tcp/mod.rs`):

-   Request parsing with path/query params
-   Regex-based routing: `("POST", "/eggs/(?P<egg_id>.*)/stop", eggs::stop)`
-   JSON responses with `json()` helper
-   Client communicates via form-encoded body over TCP

## Development Practices

### Building and Testing

**Task runner**: Uses Taskfile (`task build` or `task build:release`)

```yaml
# Runs cargo build + check_size.py to monitor executable bloat
task build:release
```

**Size tracking**: `check_size.py` compares build sizes against `.task/` cache to detect regressions

**Formatting**: `rustfmt.toml` configures: max_width=100, imports_granularity=One

### Environment Variables

-   `KURV_SERVER=true`: Required to run server (safety check, bypass with `--force`)
-   `KURV_HOME`: Override `.kurv` file location (default: executable parent dir)
-   `KURV_API_HOST`: Server host (default: 127.0.0.1)
-   `KURV_API_PORT`: Server port (default: 58787)

### Common Patterns

**Client command dispatch** (`src/cli/mod.rs:dispatch_command()`):

```rust
// Commands return DispatchResult::Dispatched or ::Server
// Server variant triggers server mode in main.rs
```

**Error handling**: Uses `anyhow` throughout, no custom error types

**Logging**: Custom logger (`src/common/log.rs`) wraps `log` crate, integrates with theme system

## File Locations

-   **State file**: `.kurv` (JSON, previously YAML) at `KURV_HOME` or exe dir
-   **Egg logs**: `task_logs/{egg-name}.{stdout|stderr}` at `KURV_HOME`
-   **Egg configs**: User-provided `*.egg` (or any other name really) YAML files with name, command, args, cwd, env

## Making Changes

When modifying:

-   **Egg lifecycle**: Update state machine in `src/kurv/mod.rs` main loop checks
-   **CLI commands**: Add route in `src/cli/mod.rs:dispatch_command()` match arms
-   **API endpoints**: Add to `src/api/mod.rs:Router::routes()` array
-   **State schema**: Start from `src/kurv/state/mod.rs:load()` and follow existing patterns
-   **Terminal output**: Use `printth!` with theme tags, define new tags in `src/cli/color/mod.rs`

## Working on Issues

When addressing github issues, avoid creating comments in the issue thread.

## Documentation

Avoid creating new markdown documentation files, unless explicitly requested.