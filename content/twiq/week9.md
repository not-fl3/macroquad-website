+++
title = "This Week in Quads #9"
description = "2024-07-22"
date = 2024-07-22T20:10:42+00:00
updated = 2024-07-22T20:10:42+00:00
template = "blog/page.html"
+++

# Nanoserde

## 0.2.0 Release - Call for Testing

Nanoserde will be bumped to `v0.2.0` in the near future, with a pair of breaking change related to cargo features. Previously, there was no way to separate out the different formats (`json`, `binary`, etc.), with all of them compiled each time. After v0.2.0 is release, individual formats may be enabled/disabled through the use of the relevant cargo feature. The `no_std` feature has also been removed, and has been replaced by a `std` feature which gates both usage on stable rust and SerDe traits on `std::collections::{HashMap, HashSet}`. [See the documentation for details on both changes](https://github.com/not-fl3/nanoserde/blob/142d80f11f74041fa0c69d16659887d993184d9d/README.md?plain=1#L55).

These changes are merged in, but have not yet been released to `crates-io`. If you rely on `nanoserde`, version `0.2.0-beta.0` has been released to allow testing prior to the version bump. There should be no other breaking changes beyond the ones described above, but please file an issue if that is not the case. 

# [Sloop](https://github.com/not-fl3/sloop)

![sloop](/week9/sloop.gif)

[Sloop](https://github.com/not-fl3/sloop) is a (very) experimental build system. It was an experiment on how hard would it be to build a fairly complex 3d game without cargo. Somehow it worked, and results were too interesting to keep it private.

While it is not very practical(yet), `sloop` might be a good case study on building a fully custom build system on top of raw `rustc`, with no `cargo` involved.

[A long mastodon thread on how it works](https://mastodon.gamedev.place/@fedor/112873297227332511).

[A few example projects](https://github.com/not-fl3/sloop-example-projects)

```rust
fn main() -> Result<(), ()> {
    let libc = sloop::DependencyBuilder::new("deps/libc")
        .build()?;
    let miniquad = sloop::DependencyBuilder::new("deps/miniquad")
        .with_dependency(&libc)
        .build()?;

    sloop::Builder::new()
        .binary()
        .name("TriangleOnTheSloop")
        .entrypoint("src/triangle.rs")
        .with_dependency(&libc)
        .with_dependency(&miniquad)
        .build()
}
```

```bash
> ls
build.rs  deps/  src/

> sloop
Building libc
Building miniquad
Building TriangleOnTheSloop src/triangle.rs
Done!

> ls
build.rs  deps/  src/  TriangleOnTheSloop*
```

