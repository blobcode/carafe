# 🍶 carafe

A simple, single-binary rust webserver with support for multiple paths and easy configuration!

## Getting Started <a name = "getting_started"></a>

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

What things you need to install the software and how to install them.

```
Give examples
```

### Installing

A step by step series of examples that tell you how to get a development env running.

Say what the step will be

```
Give the example
```

And repeat

```
until finished
```

End with an example of getting some data out of the system or using it for a little demo.

## Usage <a name = "usage"></a>

You can start carafe with the `carafe` command. In this mode, it takes in a port and directory, for example

```
carafe 8080 ./static
```

If no arguments are passed, it will by default look for a `carafe.toml` in the current directory and use that for config. If there is none, it will default to starting on port 8080 serving the current working directory. You can also pass carafe a path to a `carafe.toml` using the `-c` flag, for example

```
carafe -c ./config/carafe.toml
```

Please note that this will overide all cli arguments.