+++
title = "Cross compilation of *quads project from Linux to OSX"
description = "Instruction for cross compilation using zigbuild."
date = 2024-08-02T09:19:42+00:00
updated = 2024-08-02T09:19:42+00:00
draft = false
template = "blog/page.html"
+++

Initially I thought it would be enough to just add rust target, but it expects for linker to have support of `-framework` flag and needs MacOS SDK. `cargo-zigbuild` comes to the rescue.

## Add target:
```sh
rustup target add x86_64-apple-darwin
```

## Download necessary sdk
Luckly there is [this repo](https://github.com/roblabla/MacOSX-SDKs) with all macos sdks versions. Use needed version. Replace it in the lines below.
```sh
curl -L https://github.com/roblabla/MacOSX-SDKs/releases/download/13.1/MacOSX13.1.sdk.tar.xz | tar xJ
```

## Install Zig language compiler
Using [this guilde](https://github.com/ziglang/zig/wiki/Install-Zig-from-a-Package-Manager) for your distribution.
For Ubuntu 20.04 it was:
```sh
snap install zig --classic --beta
```

## Install zigbuild
Zig provides C cross compiler with stdlib headers and linkers for every platform, so zigbuild empowers rust with ability to link binaries for other platforms almost without restrictions.
```sh
cargo install --locked cargo-zigbuild
```

## Set SDKROOT for zigbuild to find frameworks
Add this to .bashrc if needed (replace `$(pwd)` of course)
### For OSX:
```sh
export SDKROOT=$(pwd)/MacOSX13.1.sdk/
```

## Cross-compile using zig as linker
```sh
cargo zigbuild --release --target x86_64-apple-darwin
```

## Example
![example app](/images/zigbuild_osx.png)
