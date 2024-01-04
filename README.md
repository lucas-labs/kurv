<p align="center"><img src=".github/icon-logo-h64.svg" height="128"></p>

<br>
<br>
<br>

<p align="center">𝐤𝐮𝐫𝐯 is a process manager, mainly for Node.js and Python applications. It's written in <code>Rust</code>. It daemonizes your apps so that they can run in the background. It also restarts them if they crash.
</p>

<p align="center"><img align="center" alt="Crates.io Version" src="https://img.shields.io/crates/v/kurv?style=flat-square&color=%2318181b&link=https%3A%2F%2Fcrates.io%2Fcrates%2Fkurv"></p>

<p align="center"><a href="https://kurv.lucode.ar" target="_blank">Docs</a> • <a href="https://crates.io/crates/kurv" target="_blank">Crate</a> • <a href="https://github.com/lucas-labs/kurv/tree/master?tab=readme-ov-file#readme" target="_blank">Readme</a></p>

<br>
<br>
<br>

> [!WARNING]  
> Heads up, this project is my Rust-learning playground and not production-ready yet:
> 
>   - I built this because my apps needed a process manager, and I had an itch to learn Rust. So, here it is... my first Rust project!
>   - No tests yet (oops!)
>   - Tested only on Windows 11
>   - Rust newbie alert! 🚨
>   - Using it for my own projects, but not on a grand scale


## Why 𝐤𝐮𝐫𝐯?



So, why the name 𝐤𝐮𝐫𝐯? Well, it means "basket" in many languages I don't speak, like Norwegian (but it sounded cool 😄). Think of 𝐤𝐮𝐫𝐯 as a basket for your apps. In kurv, we call each deployed app as an `egg`. So, let's go and collect some eggs 🥚 in your basket 🧺.


## Installation

> [!NOTE] 
> 𝐤𝐮𝐫𝐯 can run either as a server or as a CLI client, using the same binary. 
>
> The server is responsible for managing the eggs, while the client is used to interact with the server and tell it what to do or ask it for information.

### Download binaries

Download the latest release [from GitHub](https://github.com/lucas-labs/kurv/releases). 

### crates.io

You can also install it from [crates.io](https://crates.io/crates/kurv) using `cargo`:

```bash
cargo install kurv
```

## Usage 

![kurv usage](.github/kurv.gif)


### Start the server

To get the server rolling, type:

```bash
kurv server
```

> [!IMPORTANT]
> - 𝐤𝐮𝐫𝐯 will create a file called `.kurv` where it will store the current
> state of the server. The file will be created in the same directory where
> the binary is located or in the path specified by the `KURV_HOME_KEY`
> environment variable.
>
> - since 𝐤𝐮𝐫𝐯 can be used both as a server and as a client, if you want
> to run it as a server, you need to set the `KURV_SERVER` environment
> to `true`. This is just a safety measure to prevent you from running
> the server when you actually want to run the client.
> To bypass this, you can use the `--force` flag (`kurv server --force`)

### Collect some 🥚
To deploy/start/daemonize an app (collect an egg), do:

```bash
kurv collect <egg-cfg-path>
```

The path should point to a YAML file that contains the configuration for the egg. 

It should look something like this:

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

This will run the command `poetry run serve` in `/home/user/my-fastapi-app` with the environment variable `FASTAPI_PORT` set to `8080`.

If for some reason, the command/program crashes or exits, 𝐤𝐮𝐫𝐯 will revive it!

### Show me my eggs

If you want a summary of the current state of your eggs, run:

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

For details on a specific egg:

``` sh
$ kurv egg <egg:name|id|pid>
```

This will show you the egg's configuration, process details, etc.

### Stop an egg

To halt an egg without removing it:

``` sh
$ kurv stop <egg:name|id|pid>
```

This will stop the process but keep its configuration in the basket in case
you want to start it again later.

### Remove an egg

To actually remove an egg, run:

``` sh
$ kurv remove <egg:name|id|pid>
```

It will stop the process and remove the egg from the basket.

### Restart

If you need the process to be restarted, run:

``` sh
$ kurv restart <egg:name|id|pid>
```

### Inspiration

#### pm2
Inspired by the robust process manager, [pm2](https://pm2.keymetrics.io/), my goal with 𝐤𝐮𝐫𝐯 was to create a lightweight alternative. Not that pm2 is a resource hog, but I found myself in a server with extremely limited resources. Plus, I was itching for an excuse to dive into Rust, and voila, 𝐤𝐮𝐫𝐯 was born.

#### eggsecutor
Derived from [eggsecutor](https://github.com/lucas-labs/kurv), 𝐤𝐮𝐫𝐯 adopted the whimsical term "eggs" to represent deployed applications.

#### pueue
Insights from [pueue](https://github.com/Nukesor/pueue) were instrumental in helping me understand how to manage processes in Rust.


<br><br>

-------
With 🧉 from Argentina 🇦🇷