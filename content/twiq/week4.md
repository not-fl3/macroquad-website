+++
title = "This Week in Quads #4"
description = "2024-05-27"
date = 2024-06-03T20:10:42+00:00
updated = 2024-06-03T20:10:42+00:00
template = "blog/page.html"
+++

# blocking_event_loop on ios opengl/metal

[https://github.com/not-fl3/miniquad/pull/452](https://github.com/not-fl3/miniquad/pull/452).

Shout out to [birhburh](https://github.com/birhburh) for finishing a series of PRs porting `blocking_event_loop` on basically everything!

# Article on getrandom for *quad

[https://macroquad.rs/articles/getwasm/](https://macroquad.rs/articles/getwasm/)

Most rust crates just assume that everyone use a non-standart, third party binary, wasm-bindgen, to post-process .wasm files on web. `rand`/`ahash` a very popular crates, and one of the very few crates in the ecosystem that acknowledge that not everyone use wasm-bindgen.

Approach from the article demonstrates a completely different approach for wasm32 dependencies management: "getrandom" do not know about miniquad existence, do not depend on "miniquad" in any way, but it is still usable from a miniquad based project!

# html template

Android Chrome  on some phones render a page slightly zoomed by default. This results into some part of the *quad canvas being rendered behind the navigation bar. 

Adding `<meta name="viewport" content="width=device-width, initial-scale=1" />` fixed the problem, therefore the new recommended html template is:

```html
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>TITLE</title>
    <style>
        html,
        body,
        canvas {
            margin: 0px;
            padding: 0px;
            width: 100%;
            height: 100%;
            overflow: hidden;
            position: absolute;
            background: black;
            z-index: 0;
        }
    </style>
</head>

<body>
    <canvas id="glcanvas" tabindex='1'></canvas>
    <!-- Minified and statically hosted version of https://github.com/not-fl3/macroquad/blob/master/js/mq_js_bundle.js -->
    <script src="https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js"></script>
    <script>load("CRATENAME.wasm");</script> <!-- Your compiled wasm file -->
</body>

</html>

```

# Road to 3d macroquad

![race](/week4/race.gif)

*Old GIF for a 3d macroquad test project*


API compatibility was a big challange for a new, 3d-capable macroquad. Now we have a working prototype of a way to build an (almost)0.4 compatible, UB/statics free API:

```rust
use macroquad::compat::*;

async fn game(ctx: macroquad::Context) {
    init_compat_mode(ctx);

    loop {
        clear_background(LIGHTGRAY);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

fn main() {
    macroquad::start(Default::default(), |ctx| game(ctx));
}
```
[basic_shapes_compat.rs](https://github.com/not-fl3/macroquad/blob/0.5/examples/basic_shapes_compat.rs)


While it is possible to use a new, more fine-graded API:

[basic_shapes.rs](https://github.com/not-fl3/macroquad/blob/0.5/examples/basic_shapes.rs)
