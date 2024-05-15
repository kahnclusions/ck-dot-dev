---
title: Building Leptos apps with Nix, Fenix, and Crane
date: 2023-09-30
description: Let's walk through how to use Nix to build Leptos fullstack apps with nightly Rust, taking care of the server program, client WASM, and assets.
---

When I decided to setup this website, I decided to try building it in Leptos as a bit of an experiment. One the one hand I could use off the shelf static site generators, because this site is going to be mostly static content after all, but on the other hand I wanted to try something interesting. The Leptos web framework promises to bring the benefits of Rust onto the web, but without the bloated file sizes typically associated with WASM-based applications or other MY_FAVOURITE_LANG to JS frameworks.

So building a little website in Rust is all well and good, but how and where to publish it? Of course I had to make things a little bit difficult for myself. I already run a little Proxmox 7 homelab with a cluster of Nix VMs, so why not package the app for Nix and just deploy it to a Nix VM? Surely someone has done this before, so it should be easy, right? Right???

Famous last words.

In the end, it's not that complicated, but like everything Nix, it's not that simple either and you're pretty much on your own to figure stuff out. Crane has a pretty good starter flake file, but Leptos has a bit of a non-standard setup: it uses an extra cargo binary `cargo-leptos` and has a bunch of assets that you may want to bundle.

### Step 1. Have a Leptos fullstack project.

I'm going to assume that you already know how to set up a Leptos app, or at least you can figure that part out on your own. The reason you're here is to find out how to add Nix to it. I gotchu.

### Step 2. Have Nix on your system.

Even if you don't plan on deploying to your local machine, or developing with Nix on your local machine, I really do recommend you set it up anyway because you'll have a much tighter feedback loop doing all of this locally. On MacOS this is now as simple as running a simple command.

If you've just installed Nix and haven't used flakes before, you can make life easier by adding this config:

```fish
mkdir -p ~/.config/nix
echo "experimental-features = nix-command flakes" > ~/.config/nix/nix.conf
```

And if you're like me and use Fish shell then you can add a wrapper around `nix develop` to use Fish at `~/.config/fish/functions/develop.fish`:

```fish
function develop --wraps='nix develop'
  env ANY_NIX_SHELL_PKGS=(basename (pwd))"#"(git describe --tags --dirty) (type -P nix) develop --command fish
end
```

### Step 3. Draw the rest of the fucking owl. Aka, add a `flake.nix` file.

Specifically, add the "Quick start" `flake.nix` from `crane`. You can find it [here](https://crane.dev/examples/quick-start.html). Just create the empty file and paste all of that in. If you're impatient like me and try running `nix build` now you'll find that it sorta works but doesn't. Don't worry! This file is actually pretty close to what you need.

I want to take you through each part of the file and explain what it's for. This way it'll make more sense when we make modifications to it later. A flake file is like a definition for how to make a Nix module... it lists a bunch of inputs (dependencies), and then a bunch of outputs.
 
```nix
inputs = {
  nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  crane = {
    url = "github:ipetkov/crane";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  fenix = {
    url = "github:nix-community/fenix";
    inputs.nixpkgs.follows = "nixpkgs";
    inputs.rust-analyzer-src.follows = "";
  };

  flake-utils.url = "github:numtide/flake-utils";

  advisory-db = {
    url = "github:rustsec/advisory-db";
    flake = false;
  };
};
```

It already contains the necessary dependencies. In particular, `crane` is a Rust build tool for Nix, and `fenix` is doing the job that `rustup` usually does--allows to specify the Rust versions, toolchain, etc, to use for building. 

```nix
outputs = { self, nixpkgs, crane, fenix, flake-utils, advisory-db, ... }:
  flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      # ...blah blah more stuff...
    in
    {
      foo = bar
      # ...blah blah more stuff...
    }
```

Outputs is basically a function that has each of the named inputs as parameters. Our "return" value is to call `flake-utils.lib.eachDefaultSystem (system: ...)` to create a definition for each type of system. Finally the `let ... in ...` blocks defines a bunch of variables that can then be used in the resulting object.
