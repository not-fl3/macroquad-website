+++
title = "Macroquad 0.4 changelog"
description = "Macroquad with Metal on IOS and other updates."
date = 2023-07-12T09:19:42+00:00
updated = 2023-07-12T09:19:42+00:00
draft = false
template = "blog/page.html"
+++

# Miniquad update

Miniquad update did not affected userspace macroquad at all, all the examples and demos were completely intact. Except `use macroquad::miniquad::*` case.

Just for the `use macroquad::miniquad::*` case, here is a short recap of a miniquad changelog:

No more "Context" for event handlers.
RenderingBackend is now owned by the client code, and for window manipulation miniquad now provide a static "window" module.

For events, it looks like:
```diff
impl EventHandler for Stage {
-    fn update(&mut self, _ctx: &mut Context) {
+    fn update(&mut self) {
```

For functions that used to be part of the context, it became:
```diff
- ctx.screen_size();
+ window::screen_size()
```

And for rendering functions, now its 
```rust
let gl_context = GlContext::new();

gl_context.begin_default_pass();
```

In multiple places miniquad used to receive arguments like this: 
```
pub fn immutable<T>(ctx: &mut Context, buffer_type: BufferType, data: &[T]) -> Buffer;
```
With rendering backend being a trait object, this is no longer possible. 
Now it looks like this: 
```rust
fn new_buffer_immutable(&mut self, buffer_type: BufferType, data: BufferSource) -> BufferId;
```

On the call site, the change required is:
```diff
- .. &indices);
+ BufferSource::slice(&indices));
```

# Deprecated ex-0.2 function got removed

`draw_texture_rec`  
`widgets::InputField`  
`ui::input_field`  

# Thread safety

Macroquad always assumed WASM as a primary target and was very thread-unsafe. Before 0.4 using macroquad from different thread was basically an UB. 0.4 is still mostly single-threaded, but, at least, wrong usage will lead into a panic, not into a UB.

# Camera consistency

Small change, but there was a lot of issues with that - now render target camera and a normal camera have the Y axis.

# Default features

Cargo features are additive: let's say there is a crate A, depending on crate B and C.

If crate B depends on "Dependency" with feature "Foo" and crate C depends on the same "Dependency" with feature "Bar", cargo will add just one crate into a build process, "Dependency", with both features "Foo" and "Bar" on.

In macroquad 0.3 a library using macroquad like this: `macroquad = "0.3"` was silently introducing "audio" feature in the dependency tree. It was very counterintuitive and required complicated workarounds.

After 0.4 macroquad introduce new, no-default, features policy. It will make life of libraries authors a lot easier: `macroquad = "0.4"` in a sub-dependencies Cargo.toml will never bring new, unwanted features.

# Resources management

in 0.3, all resources was `Copy`, which prevented them to have proper destructors.

```rust
loop {
  let texture = load_texture("examples/ferris.png").await.unwrap();
  next_frame().await;
}
```
this snipped used to be a memory leak. While it almost never a good idea to load new resources mid-frame, it was still very un-idiomatic.
0.4 fixes this. All resources are just `Clone`, not `Copy` and they all acts like smart-pointers: `texture.clone()` is really really cheap, its okay to do a `.clone()` of any resource multiple times a frame: its just a pointer clone, not actual texture clone.

```rust
    let texture: Texture2D = load_texture("examples/ferris.png").await.unwrap();

    // will print 1 cause there is only 1 texture loaded
    println!("{}", telemetry::textures_count());

    {
        let texture1: Texture2D = load_texture("examples/ferris.png").await.unwrap();
        // will naturally print 2 cause there is only 1 texture loaded
        println!("{}", telemetry::textures_count());

        let texture2 = texture.clone();
        // will still print 2, texture.clone() is a cheap texture handle clone, not
        // a real "deep" clone, it does not create a new texture
        println!("{}", telemetry::textures_count());
    }

    // will print 1 again, texture1 and 2 got deleted automatically
    println!("{}", telemetry::textures_count());
```

As an example of a cheap `.clone`: 

```
    loop {
        // drawing to the texture

        // 0..100, 0..100 camera
        set_camera(&Camera2D {
            zoom: vec2(0.01, 0.01),
            target: vec2(0.0, 0.0),
            render_target: Some(render_target.clone()), // this .clone is fine, it will only clone a reference
            ..Default::default()
        });
        ...
    }
```

# Error handling

The last panicking functions were eliminated. Now everything returns `Result<_, macroquad::Error>`.

# Metal shaders

macroquad do not (yet) have any shader's cross compilers, so now all custom shaders requires `ShaderSource` struct with glsl and metal shader sources instead of just glsl.
With metal being opt-in feature this should be fine, at least until macroquad will get a shader cross-compiler.

```diff
             load_material(
-                DEFAULT_VERTEX_SHADER,
-                DEFAULT_FRAGMENT_SHADER,
+                ShaderSource {
+                    glsl_vertex: Some(DEFAULT_VERTEX_SHADER),
+                    glsl_fragment: Some(DEFAULT_FRAGMENT_SHADER),
+                    metal_shader: None,
+                },
                 mq::material::MaterialParams::default(),
             )
```

