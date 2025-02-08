# ck-dot-dev

This repository hosts my website available at [0x75.net](https://0x75.net).

The source code is open source as an example for how you could write Leptos apps and build them with Nix.

## Development

```bash
nix develop -c $SHELL
just watch
```

As a shortcut in Fish shell I put the following function at `~/.config/fish/functions/develop.fish`:

```fish
function develop --wraps='nix develop'
  env ANY_NIX_SHELL_PKGS=(basename (pwd))"#"(git describe --tags --dirty) (type -P nix) develop --command fish
end
```

and then you can enter Nix develop shell with:

```fish
develop
just watch
```

## Build and run

You can build and run in any environment with:

```fish
nix build
./result/bin/ck-dot-dev
```

I am working on an example for including the Nix flake as a service in your server config.
