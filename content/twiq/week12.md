+++
title = "This Week in Quads #12"
description = "2024-08-19"
date = 2024-08-19T20:10:42+00:00
updated = 2024-08-19T20:10:42+00:00
template = "blog/page.html"
+++

# miniquad: Anti aliased render targets

A change heavily inspired by sokol: a way to make render pass that automatically resolves multisampled render textures.

It is a breaking change, `new_render_pass_mrt` got an extra argument now. However,  it should not affect most people - `new_render_pass_mrt` got introduced a few weeks back, and, hopefully, most people still use `new_render_pass`.

[A new example](https://github.com/not-fl3/miniquad/blob/master/examples/msaa_render_texture.rs)
[PR with details](https://github.com/not-fl3/miniquad/pull/478)


for macroquad, this change allowed to implement a `render_target_msaa` function. It works just like a normal `render_target`, and `render_target.texture` is a normal macroquad's texture.

```diff
// Setup 'render_target', used to hold the rendering result so we can resize it
-let render_target = render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
+let render_target = render_target_msaa(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32, 4);
```

# road to 3d macroquad

The biggest promise of macroquad-0.5 is the same drawing API for macroquad-0.4 style static batching and for standalone 3d meshes. And it is about to actually happened!

``` rust
// direct equivalent of
// `draw_text("HELLO WORLD", 30, 300.0, 300.0, BLACK)`
// from macroquad 0.4
canvas1.draw(Text::new("HELLO WORLD", 30), vec2(300.0, 300.0), BLACK);

// using exactly the same `Text` to create a 3d model
let text = scene.model(Text::new("HELLO 3D", 30), DrawParams::default());
```

![text 3d](/week12/text3d.gif)

[Full example source](https://github.com/not-fl3/macroquad/blob/0.5/examples/gltf/main.rs)
