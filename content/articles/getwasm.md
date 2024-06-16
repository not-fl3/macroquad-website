+++
title = "getrandom with *quad on WASM"
description = "Guide For those who want to use getrandom (mainly for rand or for ahash) with *quad."
date = 2024-06-16T09:19:42+00:00
updated = 2024-07-16T09:19:42+00:00
draft = false
template = "blog/page.html"
+++

*Article originally posted on [the discord](https://discord.com/invite/WfEp6ut) by [juh9870](https://discordapp.com/users/145509639562199040)*


Guide For those who want to use getrandom (mainly for rand or for ahash) with Macroquad.  
This method utilizes crypto.getRandomValues for getting cryptographically strong random values, which are good to seed your PRNG (including quad_rand)  

## Step 1

Add dependencies on getrandom crate with custom feature, and on sapp-jsutil crate for JS bridging

Example:
```
[target.'cfg(target_arch = "wasm32")'.dependencies]
getrandom = { version = "0.2", features = ["custom"] }
sapp-jsutils = { version = "0.1" }
```

## Step 2

Create a file somewhere in your project, and make it wasm-only.

```rust
#[cfg(target_arch = "wasm32")]
mod getrandom_on_web;
```

And use the following code
```rust
use sapp_jsutils::JsObject;

extern "C" {
    fn macroquad_js_get_random_buffer(length: usize) -> JsObject;
}

/// Required by `getrandom` crate.
fn getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    let obj = unsafe { macroquad_js_get_random_buffer(buf.len()) };
    let mut bytes = Vec::with_capacity(buf.len());
    obj.to_byte_buffer(&mut bytes);

    for (target, data) in buf.iter_mut().zip(bytes) {
        *target = data;
    }
    Ok(())
}
getrandom::register_custom_getrandom!(getrandom);
```

## Step 3

Add the JS code to your project to define the function on JS side

```js
// Plugin registration, see https://macroquad.rs/articles/wasm/ for more info
getrandom_plugin = function (importObject) {
    // make macroquad_js_get_random_buffer() function available to call from rust
    importObject.env.macroquad_js_get_random_buffer = macroquad_js_get_random_buffer;
}

miniquad_add_plugin({getrandom_plugin});

function macroquad_js_get_random_buffer(length) {
    const myArray = new Uint8Array(length);
    crypto.getRandomValues(myArray);
    return js_object(myArray);
}
```
