+++
title = "This Week in Quads #10"
description = "2024-08-04"
date = 2024-08-04T20:10:42+00:00
updated = 2024-08-12T20:10:42+00:00
template = "blog/page.html"
+++

# An article on cross compilation from Linux to OSX 

[A new article on *quad cross compilation](https://macroquad.rs/articles/zigbuild-osx/)

Thanks [@https://github.com/birhburh](https://github.com/birhburh)!

# macroquad: More vertex attributes

Trading off a slightly little higher memory footprint to an ability to pass one more vertex attribute.

[PR with a description](https://github.com/not-fl3/macroquad/pull/779)

# macroquad: Default texture filter

There are two ways to set default texture filter:

```rust
fn window_conf() -> macroquad::conf::Conf {
    macroquad::conf::Conf {
        default_filter_mode: FilterMode::Nearest,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let texture: Texture2D = load_texture("pixel.png").await.unwrap();
    loop {
        clear_background(LIGHTGRAY);
        draw_texture_ex(&texture, 0., 0., WHITE);
        next_frame().await
    }
}
```

or

```rust
set_default_filter_mode(FilterMode::Linear);
// from now on all textures and fonts will be loaded with FilterMode::Linear
let texture: Texture2D = load_texture("pixel.png").await.unwrap();
```

[PR with description](https://github.com/not-fl3/macroquad/pull/772/)

# Road to 3d macroquad: embracing the Arc

Decision that took waaay to much time to make, but when it is done it feels so good. For the context, thats the problem:

```rust
{
    let texture = load_texture("ferris.png").await?;
    draw_texture(&texture);
    build_textures_atlas();
    draw_texture(&texture);
}
dbg!(telemetry::textures_count());
profiler::ui();
```

In macroquad `0.3` texture was a `Copy` handle. It would be never deleted unless explicit `delete` call. Macroquad had no problems of having internal texture references for building atlases, debugging drawcalls and whatnot.

`macroquad-0.4` tried to be smart and do a right thing - make `Texture` a statefull-ish object, that deletes itself following a normal rust lifetimes. It worked, on the surface the API looked reasonable, but it was super missleading: macroquad still needed internal texture references, atlases were still a thing etc. So on Drop texture was not really deleted, it was marked as "ready to delete", like some sort of a GC.

In `macroquad-0.5` all this will be gone. `load_texture` will return an `Arc<Texture2d>`. draw_texture will require an `Arc<Texture2D>`. It is clear from the API that `draw_texture` might store a copy of an arc for some deferred operations. It is clear that `Texture2D` is, indeed, a Texture, not a handle, and that Arc is an Arc. All this sounds super obvious, but, somehow, it took 2 major versions to implement...

[New texture example](https://github.com/not-fl3/macroquad/blob/0.5/examples/texture.rs)

[New basic_shapes example](https://github.com/not-fl3/macroquad/blob/0.5/examples/basic_shapes.rs)

[For the whole picture, new basic_shapes_compat example](https://github.com/not-fl3/macroquad/blob/0.5/examples/basic_shapes.rs)
