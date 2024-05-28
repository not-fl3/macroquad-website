+++
title = "This Week in Quads #2"
description = "2024-05-12"
date = 2024-05-20T20:10:42+00:00
updated = 2024-05-20T20:10:42+00:00
draft = false
template = "blog/page.html"
+++

## Option to reduce CPU usage to zero when waiting for an incoming event

Now its possible to halt rendering when waiting for the event.
In other words, with `blocking_event_loop: true` the following code will not print `Frame update` all the time, while the button will be responsive and the "animation" will still be smooth.


```rust
...

#[macroquad::main(window_conf)]
async fn main() {
    let mut timer_frames = 0;
    loop {
        info!("Frame updated");
        clear_background(LIGHTGRAY);
        if ui::root_ui().button(None, "Test") {
            info!("Button pressed");
            timer_frames = 50;
        }
        if timer_frames != 0 {
            timer_frames -= 1;
            draw_rectangle(0.0, 100.0, timer_frames as f32 * 20.0, 60.0, GREEN);
            macroquad::miniquad::window::schedule_update();
        }
        next_frame().await
    }
}
```

[PR with details](https://github.com/not-fl3/miniquad/pull/437)

## JS version versioning rework

Since miniquad 0.3 pretty much all *quad based app started their life with this error.
![error](/week2/error.png)  
*Version mismatch: gl.js version...*

There was no trivial fix due to the versioning scheme miniquad used to have: each js file was suppose to correspond to the exact crate version.
Now rust code can explicitly say which JS version it needs, which accomodates a lot better a real world scneario when rust crates updates a lot more frequently than JS.

Migration to a new scheme example:
```diff
--- a/js/audio.js
+++ b/js/audio.js
@@ -189,4 +189,4 @@ function register_plugin(importObject) {
     importObject.env.audio_playback_set_volume = audio_playback_set_volume;
 }
 
-miniquad_add_plugin({ register_plugin, version: "0.1.0", name: "macroquad_audio" });
+miniquad_add_plugin({ register_plugin, version: 1, name: "macroquad_audio" });
diff --git a/src/web_snd.rs b/src/web_snd.rs
index 357ec30..8c61541 100644
--- a/src/web_snd.rs
+++ b/src/web_snd.rs
@@ -14,11 +14,7 @@ extern "C" {
 #[no_mangle]
 pub extern "C" fn macroquad_audio_crate_version() -> u32 {
-    let major = 0;
-    let minor = 1;
-    let patch = 0;
-
-    (major << 24) + (minor << 16) + patch
+    1
 }
```

[PR with details](https://github.com/not-fl3/miniquad/pull/436)

# Atlases fix

If `build_textures_atlas` sometimes leaded to weird visual artifacts - it might be fixed now.

```rust
let texture: Texture2D = load_texture("examples/ferris.png").await.unwrap();
texture::build_textures_atlas();
loop {
    clear_background(LIGHTGRAY);
    draw_texture(&texture, 0., 0., WHITE);
    next_frame().await
}
```

Before  
![2](/week2/ferris2.png)

After  
![1](/week2/ferris1.png)

# Road to 3d macroquad

Main new macroquad promise: we are not limited to static batching anymore, it should be possible to explicitly batch geometry, draw directly or use instanced rendering.

This is an attempt to reproduce [pixijs's bunnymark](https://www.goodboydigital.com/pixijs/bunnymark/) with new macroquad's instanced rendering.

[Web build](https://not-fl3.github.io/miniquad-samples/bunnies/index.html)

[Source](https://github.com/not-fl3/quad-gl/blob/v0.4/examples/bunnymark.rs)

![basic-shapes](/week2/bunnies2.png)

On my t480 it gives ~20fps on 400k bunnies, while old, static-only macroquad was capable of 15fps at 20k bunnies.

Next goal - make it actually usable, and add instanced rendering [normal shapes example](https://github.com/not-fl3/quad-gl/blob/v0.4/examples/basic_shapes.rs).
