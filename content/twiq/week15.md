+++
title = "This Week in Quads #15"
description = "2024-10-01"
date = 2024-10-01T20:10:42+00:00
updated = 2024-10-01T20:10:42+00:00
template = "blog/page.html"
+++

# servo-quad

Nothing to see there yet, but lately I was playing aroudn with the idea of stripping servo from heaviest dependencies and running it with miniquad.

Imagine servo, but super fast to compile, scriptable in languages other than JS and running anywhere miniquad can run!

![servo](/week15/servo.jpg)

*this very newsletter was supposed to prevent me from spending too much on private miniquad forks, so when first results are achieved, lets get back on track with the updates*

# miniquad: window icons on Linux/X11

![x11_icons](/week15/icon.jpg)

[PR with details](https://github.com/not-fl3/miniquad/pull/492)

# miniquad: Refactored internal texture representation

Miniquad used to store `raw: GLuint` as an opengl texture, and for multi sampled texture attachment miniquad used to save a renderbuffer id inside the very same raw field. Big mistake! Lead to a lot of confusion and misinterpreting the data. Now it is properly typed:

[PR with details](https://github.com/not-fl3/miniquad/pull/493)

# macroquad: render_texture_ex

Alongside `render_texture` and `render_texture_msaa`, macroquad now have a `render_texture_ex` function, with former being a shortcut to former. This will allow to add more configuration in the future, but immediate benefits: now it is possible to have depth buffer for render texture and configure sample count for msaa render textures.

[PR with details](https://github.com/not-fl3/macroquad/pull/835)
