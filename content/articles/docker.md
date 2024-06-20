+++
title = "Docker cheatsheat"
description = "Docker is the easiest way to build *quad project for Android. This is a little cheatsheat on how to simplify life with docker."
date = 2024-06-19T09:19:42+00:00
updated = 2024-06-19T10:19:42+00:00
draft = false
template = "blog/page.html"
+++

Android SDK is huge and have a lot of dependencies, all of them requires manual installation. Docker allows to make it once and basically clone a linux computer with [everything android-related](/articles/android/#a-manual-way) already pre-installed.  

This article is aimed for folks who never seen docker before. It is not a comprehensive docker tutorial, just a list of things that is possible with `cargo-apk` image.  

# Adding persistent cache folder

 ` -v /tmp/registry\":/usr/local/cargo/registry\"`

Full command:
`docker run  --rm -v /tmp/registry:/usr/local/cargo/registry -v $(pwd):/root/src -w /root/src notfl3/cargo-apk cargo quad-apk build --release`

 This will tell docker to use `/tmp/registry` on the host machine for cargo's registry, therefore docker will not download all the dependencies on each build.

# Passing environment variables

`-e RUST_BACKTRACE=1`

`docker run --rm -e RUST_BACKTRACE=1 -v /tmp/registry:/usr/local/cargo/registry -v $(pwd):/root/src -w /root/src notfl3/cargo-apk cargo quad-apk build --release`

Useful when `cargo-quad-apk` does not behave.

# Interactive mode

`-it .. /bin/bash`

```bash
> docker run 
  --rm 
  -v $(pwd):/root/src 
  -w /root/src 
  -it notfl3/cargo-apk /bin/bash

[root@5aafd507681b src]#
```

This will run docker in an interactive mode: it will give a shell inside the container. Running `cargo quad-apk build --release` from that shell is the fastest way to rebuild the .apk.

# Modifying the container

```bash
> docker run
  --rm
  -v $(pwd):/root/src
  -w /root/src
  -it notfl3/cargo-apk /bin/bash

> [root@5aafd507681b src]# rustup update

> Ctrl-D

> docker ps -a

CONTAINER ID   IMAGE              COMMAND                  CREATED          STATUS    PORTS     NAMES
9470f79a3f61   notfl3/cargo-apk   "cargo quad-apk buil…"   34 seconds ago   Created             determined_satoshi

> docker commit 9470f79a3f61 better-cargo-apk

> docker run -it better-cargo-apk /bin/bash

```

Run container in interactive mode, do all the required modifications, stop the container. Than "commit" changes to the container with a new image tag. Than use the new image tag in all docker-related commands. Note: it is possible to use `docker commit 9470f79a3f61 notfl3/cargo-apk`, new tag is an option to roll back into original image.

# apksigner

Tools from [Signing the APK](/articles/android/#4-signing-the-apk) section of android tutorial are all available in the `not-fl3/cargo-apk` container.

`-v(/home/USER/.android):/root/.android_secrets` 

`docker run --rm -v(/home/USER/.android):/root/.android_secrets -v (pwd):/root/src -w /root/src -it notfl3/cargo-apk /bin/bash`

will get a shell session with an access to .android and to the project root. 

```
[root@960582514a32 src]# apksigner 
USAGE: apksigner <command> [options]
       apksigner --version
       apksigner --help

EXAMPLE:
       apksigner sign --ks release.jks app.apk
       apksigner verify --verbose app.apk

```
