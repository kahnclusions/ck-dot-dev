---
title: How to add cache-busting asset hashes to your Leptos app
date: 2024-05-14
description: Because when you deploy changes to styles or images, it's nice if your visitors' browsers use the latest versions of files.
published: true
---

I'm surprised this isn't better documented because it should be an essential step for anyone deploying a Leptos app into production: adding some cache-busting capability to ensure that browsers always load the right version of assets after you deploy changes to your site.

For example, the default configuration adds assets like this:

```html
<link id="leptos" href="/pkg/ck-dot-dev.css" rel="stylesheet">
<script id="leptos-link-2" src="/js/prism.js"></script>
<link rel="modulepreload" href="/pkg/ck-dot-dev.js">
<link rel="preload" href="/pkg/ck-dot-dev.wasm" as="fetch" type="application/wasm" crossorigin="">
```

It means that if you deploy a new version of, say, `/pkg/ck-dot-dev.css`, browsers will happily continue loading the old version they've cached, which means your site may look broken until those browsers decide to refetch the file.

Hash-based cache-busting means hashing your asset files and adding that hash to the file names, so that when the file has changed, it's filename also changes, and browsers are forced to fetch the new version (which they will then cache). Like this:

```html
<link id="leptos" href="/pkg/ck-dot-dev.-f_O12vPNjgpVANP75DJ0Q.css" rel="stylesheet">
```

If you dig through the docs on `cargo-leptos` you'll see that there's an option you can add to your `Cargo.toml`:

```toml
[package.metadata.leptos]
hash-files = true
```

So I added this and tried to deploy and run again, but, no dice. I did see a hash file now generated at `./target/release/hash.txt` but the link and script tags at runtime were still referencing the hashless filenames.

The solution is that you need both the `hash-files = true` option at compile-time, the `hash.txt` to be in the same directory as the binary file (in case you copy or package it somewhere), _and_ the `LEPTOS_HASH_FILES=true` environment variable at runtime.

With all of those requirements satisfied, you should get a head like this:

```html
<link id="leptos" href="/pkg/ck-dot-dev.-f_O12vPNjgpVANP75DJ0Q.css" rel="stylesheet">
<script id="leptos-link-2" src="/js/prism.js"></script>
<link rel="modulepreload" href="/pkg/ck-dot-dev.OM9NZwWBie7UkQxNQWbv3A.js">
<link rel="preload" href="/pkg/ck-dot-dev.NwV5azkWdHtdoquxlzfJkw.wasm" as="fetch" type="application/wasm" crossorigin="">
```

Success!
