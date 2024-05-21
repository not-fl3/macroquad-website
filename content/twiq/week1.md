+++
title = "This Week in Quads #1"
description = "2024-05-12"
date = 2024-05-12T20:10:42+00:00
updated = 2024-05-12T20:10:42+00:00
draft = false
template = "blog/page.html"
+++

## Post #1!

Now we have an [RSS feed](http://macroquad.rs/atom.xml) with a small, low effort, weekly progress summary.

## cargo webquad

[cargo-webquad](https://github.com/not-fl3/cargo-webquad/)

One single command to "run" quad based project on web. Handy for quick dev builds.

```
> cargo quadweb serve --example basic_shapes --assets examples
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
Copying examples to "****/target/web-artifacts"
addr: http://localhost:8080
```

## Embracing HashMaps

Fairly big change from [eloraiby](https://github.com/eloraiby): [miniquad/pull/428](https://github.com/not-fl3/miniquad/pull/428)

Instead of evergrowing plain vectors, miniquad resources now lives in HashMaps. No API changes, no measurable performance regressions and no constant memory leaks.

## Road to 3d macroquad

Sneak-peak into graphics part of future macroquad is now available as "v0.4" branch of quad-gl. Super unstable and highly experimental!

Gradual batching control with very macroquad-like API:
[basic-shapes](https://github.com/not-fl3/quad-gl/blob/v0.4/examples/basic_shapes.rs)
![basic-shapes](/week1/basic_shapes.gif)

And some basic 3d rendering:
[gltf](https://github.com/not-fl3/quad-gl/blob/v0.4/examples/gltf.rs)
![helmet](/week1/h.gif)
