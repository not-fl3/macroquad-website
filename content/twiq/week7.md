+++
title = "This Week in Quads #7"
description = "2024-07-01"
date = 2024-07-01T20:10:42+00:00
updated = 2024-07-01T20:10:42+00:00
template = "blog/page.html"
+++

# macroquad's set_uniform arrays

Now it is possible to create materials with arrays in uniforms and to set those uniforms by normal macroquad code.

```rust
material.set_uniform("Color", color)
material.set_uniform_array("Lights", &light_positions[..]);
```

[example](https://github.com/not-fl3/macroquad/blob/master/examples/custom_material.rs)

[PR with description](https://github.com/not-fl3/macroquad/pull/754)

# egui-miniquad crates.io release

egui-miniquad compatible with the latest miniquad got crates.io release.

[Release link](https://github.com/not-fl3/egui-miniquad/releases/tag/0.15.0)

# Road to 3d macroquad

## Gizmos

First look onto the gizmos API - a special, no context, no parametrisation way to draw debug UI.

![gizmos](/week7/gizmo.gif)
![gizmos](/week7/gizmo2.gif)

*[A new gizmo example](https://github.com/not-fl3/macroquad/blob/reimagine/examples/gizmos/main.rs)*


## non-exclusive immidiate mode

Non-exclusive macroquad-0.4 comaptibility mode allows using the old API alongside the new one, porting old code to new API part by part.

```diff
async fn game(ctx: macroquad::Context) {
-    init_compat_mode(ctx);
+    init_compat_mode(&ctx);

    loop {
        clear_background(LIGHTGRAY);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

```
