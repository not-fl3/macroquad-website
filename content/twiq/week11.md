+++
title = "This Week in Quads #11"
description = "2024-08-12"
date = 2024-08-12T20:10:42+00:00
updated = 2024-08-12T20:10:42+00:00
template = "blog/page.html"
+++

# Website rehaul

A big [PR](https://github.com/not-fl3/macroquad-website/pull/17) landed on macroquad-website, making this very website a lot better, specifically on mobiles.

Great work, [brettchalupa](https://github.com/brettchalupa)!

# Road to 3D macroquad

A great "builder API dilemma" was finally solved. For those who follow macroquad-0.5 branch, it went through quite a few different takes on a builder API for passing arguments to draw_* functions. I want to believe that this week was the last iteration and this is the final version of a drawing API.

```rust
// macroquad-0.4
draw_circle(100.0, 100.0, 10.0, RED);

// macroquad-0.5
canvas.draw(Circle::new(10.0), vec2(100.0, 100.0), RED);
```

Pros:
- Simple draw calls still look compact.
- Last argument is an `Into<DrawParams>`. No more `draw_circle_lines` - all the drawing styles and material is in the `DrawParams`.
- It is possible to use `Circle` without canvas and draw it without any batching.
- It is possible to use `Circle` to create a 3d mesh.

Cons:
- It might be tempting to persist the `Circle` and it is not super clear that it is just a lightweight builder pattern thing.
- API discoverability suffer a little.

This week macroquad-0.4 drawing functions got ported to the new API.

[New texture example](https://github.com/not-fl3/macroquad/blob/0.5/examples/texture.rs)

[New basic_shapes example](https://github.com/not-fl3/macroquad/blob/0.5/examples/basic_shapes.rs)

This change might not look like such a big deal, but it was the last big roadblock for publishing macroquad-0.5-alpha and it took crazy amount of iterations.
