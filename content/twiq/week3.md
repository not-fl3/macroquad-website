+++
title = "This Week in Quads #3"
description = "2024-05-27"
date = 2024-05-27T20:10:42+00:00
updated = 2024-05-27T20:10:42+00:00
draft = true
template = "blog/page.html"
+++

# blocking_event_loop on mac

A PR by [birhburh](https://github.com/birhburh).

[https://github.com/not-fl3/miniquad/pull/443](https://github.com/not-fl3/miniquad/pull/443).

# macroquad configuration

Before, in the following example, `config` was supposed to return miniquad's `Conf` struct.
Now it can return either miniquad's `Conf` directly and get macroquad's default launch arguments, or return macroquad's `Conf` and configure macroquad-specific things.

As the first macroquad-specifc argument, it is now possible to specify which event will wake macroquad up from `blocking_event_loop`.

In other words, this code will print "tick" only after "Space" press.

```rust
fn config() -> macroquad::conf::Conf {
    let mut conf: miniquad::conf::Conf = Default::default();
    conf.platform.blocking_event_loop = true;
    Conf {
        miniquad_conf: conf,
        update_on: Some(UpdateTrigger {
            specific_key: Some(vec![KeyCode::Space])
        })
    }
}
#[macroquad::main(config)]
async fn main() {
    loop {
        println!("tick");
        next_frame().await;
    }
}
```

# Optional WebGL2 support on wasm

Miniquad always used webgl1 as the most commonly supported web graphics API.
Now it is possible to optionally create webgl2 context and use `version 300 es` shaders within a standart miniquad.

[PR with description](https://github.com/not-fl3/miniquad/pull/444)

# macroquad test suite

`quadtest` is a little project that collects *quad related regressions in a single binary. This week it got a few atlas related tests.

![quadtest](/week3/quadtest.gif)
