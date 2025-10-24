# binman

binary/executables manager, for downloading and updating executables.

## How it works

1. Read from `toml` files, include all configurations for which binary and executables needs to be updated and installed.
2. Create bash shell script according to `toml` file.
3. Execute it.

## Dependencies

bash

## Directory Structure

```
.
├── Cargo.toml
├── LICENSE
├── README.md
└── src/
└── example/
```

## Quick Start

Below are optional argument you can pass to `binman`.

binman only accept one argument, `toml` file path.
If no argument provided, it will look for filename `binman.toml` in this order.
2. `~/.binman.toml`
3. `$XDG_CONFIG_HOME/binman.toml`  if `$XDG_CONFIG_HOME` is defined
4. `~/.config/binman.toml`
