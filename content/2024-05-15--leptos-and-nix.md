---
title: Building Leptos apps with Nix, Fenix, and Crane
date: 2024-05-16
description: Let's walk through how to use Nix to build Leptos fullstack apps with nightly Rust, taking care of the server program, client WASM, and assets.
published: true
---

When I decided to setup this website, I decided to try building it in Leptos as a bit of an experiment. The Leptos web framework promises to bring the benefits of Rust onto the web, but without the bloated file sizes and slow startup times typically associated with WASM-based applications or `MY_LANG-to-JS` frameworks.

So building a little website in Rust is all well and good, but how and where to publish it? Of course I had to make things a little bit difficult for myself. I already run a little Proxmox 7 homelab with a cluster of Nix VMs, so why not package the app for Nix and just deploy it to a Nix VM? Surely someone has done this before, so it should be easy, right? Right???

Famous last words.

In the end, it's not that complicated, but like everything Nix, it's not that simple either and you're pretty much on your own to figure stuff out. Crane has a pretty good starter flake file, but Leptos has a bit of a non-standard setup: it uses an extra cargo binary `cargo-leptos` and has a bunch of assets that you may want to bundle.

### Step 1. Have a Leptos fullstack project.

I'm going to assume that you already know how to set up a Leptos app, or at least you can figure that part out on your own. The reason you're here is to find out how to add Nix to it. I gotchu.

### Step 2. Have Nix on your system.

Even if you don't plan on deploying to your local machine, or developing with Nix on your local machine, I really do recommend you set it up anyway because you'll have a much tighter feedback loop doing all of this locally. On MacOS you can set it up with a single command.

Again, I'm going to assume that you're here because you already have an interest in Nix and have already gotten as far as running it somewhere and even configuring a server with nginx where you could host your Leptos app.

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

We're going to copy/paste the "quick start" example from [here](https://crane.dev/examples/quick-start.html) into `YOUR_PROJECT/flake.nix`. If you're impatient like me and try running `nix build` now you'll find that it sorta works (maybe it builds) but doesn't actually do what we need. Don't worry! This file is actually not far off from what we need.

If you're not sure what a flake file is, it's basically a definition for how to make a Nix expression (or package) with verion-pinned dependencies that can be imported and used in other packages... it lists a bunch of inputs (dependencies), and then an output. Flakes are composable: you can use flakes as inputs to build other flakes.

In the flake file you'll see a line in the `outputs` block like this:

```nix
craneLib = crane.lib.${system};

# ...

craneLibLLvmTools = craneLib.overrideToolchain
  (fenix.packages.${system}.complete.withComponents [
```

This lets us override the nixpkgs that crane is going to use to build stuff. I've set mine to this:

```nix
rustToolchain = fenix.packages.${system}.fromToolchainFile {
  file = ./rust-toolchain.toml;
  sha256 = "paste the actual hash here";
};

craneLib = crane.lib.${system}.overrideToolchain rustToolchain;

# ...

craneLibLLvmTools = craneLib.overrideToolchain
  (rustToolchain.withComponents [
```

This tells crane to override the rust toolchain with the one we specify in `./rust-toolchain.toml`. You can usually just get the right hash value to use from the error message the first time you try to build. Because we need to build for WASM as well as the server, and because I use "nightly" features from Leptos, we should add the target in our toolchain file:

```toml
[toolchain]
channel = "nightly"
targets = ["wasm32-unknown-unknown"]
```

Next we need to add `cargo-leptos` so that we can run our Leptos commands like `cargo leptos watch` or `cargo leptos build --release`. Just below the craneLib there's a section of `buildInputs`. We're going to add a few packages here:

```nix
buildInputs = with pkgs; [
  cargo-leptos
  binaryen
  tailwindcss
```

Next we need to update the `src` of our flake because our build needs to include more than just the Rust source files. For this we can create a "path filter" and then use it.

```nix
srcFilter = path: type:
  (lib.hasSuffix "tailwind.config.js" path) ||
  (lib.hasInfix "/public/" path) ||
  (lib.hasInfix "/style/" path) ||
  (craneLib.filterCargoSources path type)
;
```

If there are any other file types or directories you need to include, you can put them here. For example to include `*.html` files you can add `(lib.hasSuffix "\.html" path) ||`.

Next we need to use this filter to update our package source:

```nix
my-crate = craneLib.buildPackage (commonArgs // {
inherit cargoArtifacts;
src = lib.cleanSourceWith {
  src = craneLib.path ./.; # the original src
  filter = srcFilter; # the filter we defined above
};
```

And add our extra binaries as build inputs so that they are available for our cargo build command:

```nix
nativeBuildInputs = with pkgs; [
  cargo-leptos
  tailwindcss
  binaryen
  makeWrapper
];
```

Now we finally come to the most "interesting" and relevant part. We can't just let Crane run the usual cargo build command, we want to run the leptos build, and then we want to collect all the outputs.

```nix
buildPhaseCargoCommand = "cargo leptos build --release -vv";
installPhaseCommand = ''
  mkdir -p $out/bin
  cp target/release/${name} $out/bin/
  cp -r target/site $out/bin/
  wrapProgram $out/bin/${name} \
    --set LEPTOS_SITE_ROOT $out/bin/site
'';
```

Finally you can also add the new dependencies for the dev shell, so that we can use `nix develop`:

```nix
# Extra inputs can be added here; cargo and rustc are provided by default.
packages = with pkgs; [
  cargo-leptos
  binaryen
  tailwindcss
  just
  cargo-watch
  treefmt
  nixpkgs-fmt
  rustfmt
];
```

### Step 4. Build!

If you've done everything right, then you should now be able to run `nix build` and wait a few minutes (it takes a while the first time) and if everything worked out you should end up with some output in `./result/`. You can try running your project at `./result/bin/my-crate`!

```nix
❯ ./result/bin/ck-dot-dev
listening on http://127.0.0.1:3000
```

And if you visit that link in your favourite browser then you should see your leptos app. You can also do `nix develep -c $SHELL` and then run commands like `just watch` to use watch mode! Hooray, we did it!

For reference here is my working [flake.nix](https://github.com/kahnclusions/ck-dot-dev/blob/main/flake.nix).

Next time, I'll show how to include this flake in your Nix system configuration to deploy and run it on a server.
