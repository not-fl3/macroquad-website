+++
title = "This Week in Quads #14"
description = "2024-09-02"
date = 2024-09-12T20:10:42+00:00
updated = 2024-09-12T20:10:42+00:00
template = "blog/page.html"
+++

# miniquad: MSAA render textures on web

A second(after instancing) platform-dependent feature in miniquad: render passes resolve attachment. They are only supported on GL3+ and WebGl2, and they helps with anti-aliased render textures. While, tehcnically, they were implemented in miniquad already, now we got `features.resolve_attachments` and proper WebGL support.

[Examnple](https://github.com/not-fl3/miniquad/blob/master/examples/msaa_render_texture.rs)

[PR with details](https://github.com/not-fl3/miniquad/pull/487)

*Thanks Sokol, where the feature was gleaned from*

# macroquad: Configurable batch size

Macroquad performs automatic and static batching for each draw_* call. For each draw call, it pre-allocate a huge cpu/gpu buffer to add vertices to. When it exceeds the buffer, it allocates the new one, marking the new draw call.

Some examples when altering those values migh be convinient:
- for huge 3d models that do not fit into a single draw call, increasing
    the buffer size might be easier than splitting the model.
- when each draw_* call got its own material, buffer size might be reduced to save some memory

[PR with details](https://github.com/not-fl3/macroquad/pull/804)

# macroquad: A lot of Tiled related fixes

Shout out to [InnocentusLime](https://github.com/InnocentusLime) who fixed a ton of bugs in semi-abandoned macroquad-tiled!

[#792](https://github.com/not-fl3/macroquad/pull/792)
[#788](https://github.com/not-fl3/macroquad/pull/788)
[#803](https://github.com/not-fl3/macroquad/pull/803)
[#808](https://github.com/not-fl3/macroquad/pull/808)
[#801](https://github.com/not-fl3/macroquad/pull/801)

# Road to 3d macroquad

To battle test soundness of the new camera API, I ported fish-fight's follow camera. It does seem to work just fine, this might be a final interation on macroquad-0.5 2D camera API.

![camera](/week14/camera.gif)

[Example source](https://github.com/not-fl3/macroquad/blob/0.5/examples/follow_camera/main.rs)
