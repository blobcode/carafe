# 🍶 carafe

A simple, single-binary rust webserver with support for multiple paths and easy configuration!

## Getting Started <a name = "getting_started"></a>

To get started, you will need cargo & rust installed

### Installing

To install, simply clone the repo,

```
git clone https://github.com/blobcode/carafe
```

`cd` into it

```
cd carafe
```
and run it

```
cargo run --release
```
## Usage <a name = "usage"></a>

You can start carafe with the `carafe` command (or `cargo run` in development). In this mode, it takes in a port and directory, for example

```
carafe 8080 ./static
```

If no arguments are passed, it will by default look for a `carafe.toml` in the current directory and use that for config. If there is none, it will default to starting on port 8080 serving the current working directory. You can also pass carafe a path to a `carafe.toml` using the `-c` flag, for example

```
carafe -c ./config/carafe.toml
```

Please note that this will overide all cli arguments.