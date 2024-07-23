+++
title = "This Week in Quads #8"
description = "2024-07-08"
date = 2024-07-15T20:10:42+00:00
updated = 2024-07-15T20:10:42+00:00
template = "blog/page.html"
+++

# Miniquad

## MacOS PR

A big rewrite of MacOsX implementation by [birhburh](https://github.com/birhburh) that fixed a bunch of old old-standing MacOs issues. 

[PR with the description](https://github.com/not-fl3/miniquad/pull/462)

# Macroquad

## Breaking change in `load_material`

Since `0.4.11`, macroquad's materials could handle uniform arrays. The signature change in `load_material` was supposed to be backwards compatible, but turned out that
- it was not quite backwards compatible
- the attempt to make it backward compatible introduce a lot of internal complications

This is the change required to the user code:

```diff
         MaterialParams {
-            uniforms: vec![("Center".to_owned(), UniformType::Float2)],
+            uniforms: vec![UniformDesc::new("Center", UniformType::Float2)],
             ..Default::default()
         },
```

[PR with explanation](https://github.com/not-fl3/macroquad/pull/766)

## Multiline draw_text

PR by [https://github.com/cyrgani](https://github.com/cyrgani), a highly requested feature, a single function for multiline text. 

![Multiline text](/week8/multiline.png)

[PR with description](https://github.com/not-fl3/macroquad/pull/751)

## Removed `bumpalo` dependency

`bumpalo` was responsible for bumping minimal supported rust version for macroquad quite a few times already. Turned out we only use two functions from `bumpalo`, and only in `experimental::scene` module, which is still there mostly for legacy reason - it should be a separate crate!

[PR with description](https://github.com/not-fl3/macroquad/pull/768)

## Revived Fish

FishGame might be the best big test case for `experimental::scene`. Turned out that with a few changes it still works with macroquad-0.4. Updated repo: [Fish](https://github.com/not-fl3/FishFight-The-Prequel)

![fish](/week8/fish.gif)

_Sorry, I forgot how this game works_
