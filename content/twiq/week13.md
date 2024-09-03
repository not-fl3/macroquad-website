+++
title = "This Week in Quads #13"
description = "2024-08-26"
date = 2024-08-26T20:10:42+00:00
updated = 2024-08-26T20:10:42+00:00
template = "blog/page.html"
+++

# macroquad: UTF-8 in input fields

All macroquad's UI input fields with text now fully support UTF-8.

![utf](/week13/utf.jpg)

[PR with description](https://github.com/not-fl3/macroquad/pull/786/)

# Road to 3d macroquad

## Camera2D rework

![ferris](/week13/ferris.gif)

"How to make an aspect-ratio respecting camera" is, by far, a single most commonly asked question in macroquad's discord. Everything about macroquad's cameras was very frustrating and super non-intuitive.

This week macroquad might have got a solution: just remove the camera alltogether! Over all camera fixing proposals macroquad ever had, this "camera" using code looks by far the best. Next step - support an old, macroquad-0.4 Camera2D in a compatibility layer and make sure that the new style of handling viewports got feature-parity with the old camera.


```rust
    // This will guarantee that no matter what window size,
    // the top to bottom distance will be 100 in world space.
    canvas.set_viewport_bound(ViewportBound::Horizontal(100.0));
    loop {
        ctx.clear_screen(WHITE);
        crab_position += crab_direction(&ctx);
        // crab_position will be exactly in the middle of the window
        canvas.set_viewport_center(crab_position);
        ...
        canvas.draw(Sprite::new(&crab), crab_position, WHITE);
        ...
    }
```

[Full example source](https://github.com/not-fl3/macroquad/blob/reimagine/examples/camera.rs)

## Rounded rectangle

Follow up on last weeks "BuilderAPI" thing. All the drawing functions migrated to the new API, the last ported one being a `draw_rounded_rectangle.`.
Now it looks like

```rust
canvas1.draw(
    Rectangle::new(100.0, 100.0).rounded_corners(RoundedCorners {..}), 
    vec2(0.0, 0.0),
    RED
);
```


