+++
title = "This Week in Quads #6"
description = "2024-06-24"
date = 2024-06-24T20:10:42+00:00
updated = 2024-06-24T20:10:42+00:00
template = "blog/page.html"
+++

## Better miniquad error handling on linux:

Before:

```
thread 'main' panicked at /home/fl3/fun/miniquad/src/lib.rs:346:50:
X11 backend failed
```

After:
```
thread 'main' panicked at /home/fl3/fun/miniquad/src/lib.rs:346:50:
X11 backend failed: LibraryNotFound(DlOpenError("libxkbcommon.so.0"))
```

[Link to PR](https://github.com/not-fl3/miniquad/pull/465)

## MacOs exit hotkey

Thanks to [markmurphydev](https://github.com/markmurphydev) MacOs miniquad window now closes with Cmd-Q and have all the Mac standart application menus.

[Link to PR](https://github.com/not-fl3/miniquad/pull/464)

# Road to 3d macroquad

A [new example](https://github.com/not-fl3/macroquad/blob/reimagine/examples/materials/main.rs) with a first take on a (very WIP) material system with customizable uniforms and shaders.

![helmet](/week6/helmet.gif)

# Community projects

_Feel free to post your own projects here by opening a PR against [next week blogpost](https://github.com/not-fl3/macroquad-website/blob/source/content/twiq/week7.md)._

## miniquad on KaiOS

![](/week6/kaios.jpg)

[https://github.com/birhburh/miniquad_kaios_tests](https://github.com/birhburh/miniquad_kaios_tests)

_(from [discord](https://discord.com/invite/WfEp6ut) #showcase)_

## Game development in Rust with Macroquad

`A complete guide on how to develop a classic 2D shoot 'em up game using the game library Macroquad and the Rust programming language. It covers everything from a simple Hello World Macroquad application to adding graphics, audio, a shader, a graphical menu, and how to release the game on multiple platforms.`

_(from [discord](https://discord.com/invite/WfEp6ut) #showcase)_

[https://mq.agical.se/](https://mq.agical.se/)

![](/week6/mqagical.png)
