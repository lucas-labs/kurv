<p align="center"><img src=".github/icon-logo-h64.svg" height="128"></p>

<br>
<br>
<br>


<p align="center">${\normalsize \textbf{kurv}}$ is a process manager, mainly for Node.js and Python applications. It's written in <code>Rust</code>. It daemonizes your apps so that they can run in the background. It also restarts them if they crash.</p>

<br>
<br>
<br>

> [!WARNING]  
> This project is not ready for production use:
>   - I created this project because I needed a process manager for my apps and because I wanted to learn Rust. This is my first Rust project.
>   - No tests (yet)
>   - Only tested on Windows 11
>   - I'm not a Rust expert!
>   - I'm using it for my own projects, but it hasn't been tested on a large scale


## Why `kurv`?
`kurv` means "basket" in many languages. For example, in Norwegian (I'm not Norwegian, but I liked the word 😄). Think of `kurv` as a basket for your apps. In `kurv`, we call each deployed app an `egg`. So, let's go and collect some eggs 🥚 in your basket 🧺.

## Installation

Download the latest release from GitHub. 

> [!NOTE] 
> `kurv` can run either as a server or as a CLI client, using the same binary. 

## Start the server

To start the server, run:

```bash
kurv server
```

> [!IMPORTANT]
> - `kurv` will create a file called `.kurv` where it will store the current
> state of the server. The file will be created in the same directory where
> the binary is located or in the path specified by the `KURV_HOME_KEY`
> environment variable.
>
> - since `kurv` can be used both as a server and as a client, if you want
> to run it as a server, you need to set the `KURV_SERVER` environment
> to `true`. This is just a safety measure to prevent you from running
> the server when you actually want to run the client.
> To bypass this, you can use the `--force` flag (`kurv server --force`)

## Collect some 🥚
To collect an egg (deploy/start/deamonize an app), run:

```bash
kurv collect <egg-cfg-path>
```

The path should point to a YAML file that contains the configuration for the egg. The configuration file should look like this:

```yaml title="myegg.kurv"
name: fastapi # the name of the egg / should be unique
command: poetry # the command/program to run
args: # the arguments to pass to the command
  - run
  - serve
cwd: /home/user/my-fastapi-app # the working directory in which the command will be run
env: # the environment variables to pass to the command
  FASTAPI_PORT: 8080
```

This will run the command `poetry run serve` in the directory `/home/user/my-fastapi-app` with the environment variable `FASTAPI_PORT` set to `8080`.

If for some reason, the command/program exits, `kurv` will try to restart it.

## Show me my eggs

If you want a summary of your eggs, their state, etc., run:

```zsh
$ kurv list

🥚 eggs snapshot

╭───┬───────┬───────────┬─────────┬───┬────────╮
│ # │ pid   │ name      │ status  │ ↺ │ uptime │
├───┼───────┼───────────┼─────────┼───┼────────┤
│ 1 │ 35824 │ fastapi   │ running │ 0 │   1s   │
│ 2 │ 0     │ fastapi-2 │ stopped │ 0 │   -    │
╰───┴───────┴───────────┴─────────┴───┴────────╯
```

If you want to see more details about a specific egg, run:

``` sh
$ kurv egg <egg:name|id|pid>
```

This will show you the egg's configuration, the process details, etc.

## Stop an egg

If you want to stop an egg, run:

``` sh
$ kurv stop <egg:name|id|pid>
```

This will not remove the egg; it will just stop the process.

## Remove an egg

To actually remove an egg, run:

``` sh
$ kurv remove <egg:name|id|pid>
```

It will stop the process and remove the egg from the basket.

## Restart

If you need the process to be restarted, run:

``` sh
$ kurv restart <egg:name|id|pid>
```
