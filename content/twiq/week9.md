+++
title = "This Week in Quads #9"
description = "2024-07-08"
date = 2024-07-22T20:10:42+00:00
updated = 2024-07-22T20:10:42+00:00
template = "blog/page.html"
draft = true
+++

# Nanoserde

## 0.2.0 Release - Call for Testing

Nanoserde will be bumped to `v0.2.0` in the near future, with a pair of breaking change related to cargo features. Previously, there was no way to separate out the different formats (`json`, `binary`, etc.), with all of them compiled each time. After v0.2.0 is release, individual formats may be enabled/disabled through the use of the relevant cargo feature. The `no_std` feature has also been removed, and has been replaced by a `std` feature which gates both usage on stable rust and SerDe traits on `std::collections::{HashMap, HashSet}`. [See the documentation for details on both changes](https://github.com/not-fl3/nanoserde/blob/142d80f11f74041fa0c69d16659887d993184d9d/README.md?plain=1#L55).

These changes are merged in, but have not yet been released to `crates-io`. If you rely on `nanoserde`, version `0.2.0-beta.0` has been released to allow testing prior to the version bump. There should be no other breaking changes beyond the ones described above, but please file an issue if that is not the case. 
