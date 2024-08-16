+++
title = "Macroquad on iOS"
description = "Step by step quide in setting up development environment and building a game for iOS."
date = 2022-10-31T09:19:42+00:00
updated = 2022-10-31T10:19:42+00:00
draft = false
template = "blog/page.html"
+++

![zemeroth](ios_zemeroth.png)

# Setting up a macroquad project

This article assume that all the commands are invoked in the root folder of any mini/macroquad based project. For simplicity, lets assumet that this project was created like this:

```sh
> cargo init mygame
> cd mygame
> cargo add macroquad
> cat > src/main.rs << EOF
use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(RED);
        draw_circle(200.0, 200.0, 60.0, YELLOW);
        next_frame().await
    }
}
EOF
```

# Building for the simulator

iOS application is just a normal folder, named like "MyGame.app". This folder contains the binary, a file with a metadata and all the resources.  
The binary is a normal, cargo-produced, binary and resources are normal files, just like on any other platform. There is no third-party post-processors, resource compilators or anything like this.  
Create a folder, copy your binary and resources and it is a valid iOS application!

```sh
mkdir MyGame.app

cargo build --target x86_64-apple-ios --release

cp target/x86_64-apple-ios/release/mygame MyGame.app

cat > MyGame.app/Info.plist << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
<key>CFBundleExecutable</key>
<string>mygame</string>
<key>CFBundleIdentifier</key>
<string>com.mygame</string>
<key>CFBundleName</key>
<string>mygame</string>
<key>CFBundleVersion</key>
<string>1</string>
<key>CFBundleShortVersionString</key>
<string>1.0</string>
</dict>
</plist>
EOF

# only once, to get the emulator running
xcrun simctl list
xcrun simctl boot LONG_HEX_ID_OF_REQUIRED_IPHONE_FROM_SIMCTL_LIST

# also just once, to show the simulator UI
open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app/

# on each build, to run the game
xcrun simctl install booted MyGame.app/
xcrun simctl launch booted com.mygame
```

**note** on resources. All the files inside "MyGame.app/assets" are accesible to `miniquad::fs` just like on any other platform.

```sh
├── MyGame.app
│   ├── mygame
│   ├── Info.plist
│   ├── assets
│   │   ├── texture.png
│   │   ├── someronfile.ron
```

`load_texture("texture.png")` or `load_file("someronfile.ron")` will work just fine.

# Simulator logs

```sh
xcrun simctl spawn booted log stream --predicate 'processImagePath endswith "mygame"'
```

# Deploying on the real device with

The real device use exactly the same "application bundle" format - its a "Name.app" folder with a binary, Info.plist and resources.

But real device use aarch64-apple-ios instead of x86_64-apple-ios and it requires the bundle to be signed.

Which means - to install and run the app, following conditions should be met:

- the bundle(Name.app folder) should contain a "embedded.mobileprovision" file
- in iphone's settings, in General -> "VPN & Device Management" should be a record with your team name

## Getting provisioning profile files

This is ridicoulosly painful proces. Brace yourself!
Good news - its only a one time thing. With all the certificates being available installing the app to an iphone works just as smooth as to the simulator!

A lot of credits goes to this article: This article helped A TON: https://medium.com/@vojtastavik/building-an-ios-app-without-xcodes-build-system-d3e5ca86d30d
It is a bit outdated, some CLI arguments are a bit different now, but, in general, it was incredibly instrumental to get all this done!

### Prerequisites

- free appstore account
- macos 10.15+ (probably 10.14 works as well)
- xcode 11+
- [ios-deploy](https://github.com/ios-control/ios-deploy) tool. (`brew install ios-deploy`)
- any ios device

### Connecting phone with xcode

On the first connection, both xcode and macos might complain about versions incompatibility.
MacOs complains may be just ignored, but to make XCode recognise the device:

- double check that there is a folder with Iphone's ios version here: `/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/DeviceSupport`
- if no - download the required file somewhere from the internet(there are a lot of repos on github with such a data).

The goal of this step - to see a new device available in `Xcode -> Window -> Devices and Simulators`

### On DeviceSupport files

Any(11+ at least) xcode could install provision files on any IOS device!

On pre 16 iOS, the only requirement is a little file in "/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/DeviceSupport"

Post iOS 16, iPhone should be in a "Developer mode" to be able to receive the provisions. However, the "Developer mode" toggle is hidden in the settings until device seen a mac with xcode 14 installed at least once. Yes, you need to physically connect your iPhone to any Mac with XCode 14 to get a toggle in the menu!

There are, however, scatchy third-party tools("iCareFone 2" did it for me) to get this very toggle. With this toggle on, XCode11 with "16.2" in "DeviceSupport" folder can successefully install the provisions on the iphone.

### .mobileprovision file

Purpose of this step - create an empty xcode project runnable on the iphone. This project will be never used for building anything, just to make xcode to download provision files from apple developer portal.

First, login to the app store account. (it might be a free account, no need for the developer one). `xcode->preferences->account login create team`

Than create a new dummy project with the desired bundle indentifier (`com.mygame`).

Run it on the device.

If xcode complains with "-402620375" (and no other explanations) - add `--generate-entitlement-der` to `Project -> Build Settings -> Other Code Signing Flags`.

This will create a runnable app on the phone and make two provision files in `~/Library/MobileDevice/Provisioning\ Profiles/`.
One for "com.mygame" and one for "com.mygameUITests". To find which one is which `cat ~/Library/MobileDevice/Provisioning\ Profiles/lotsofhex.mobileprovision | grep -a UITests`. The one that doesnt have UITests in it is the one!

Copy it to MyApp.app/embedded.mobileprovision

**Note** it seems XCode (at least version 15.1) is a bit arbitrary about which `.mobileprovisions` file it places in the **Provisioning Profiles** folder. If your developer account has several profiles, you may have to clean the folder, forcing XCode to redownload the provisioning file, until you get one that matches your bundle identifier. **Also note** at least with XCode 15.1 the wildcard provisioning profile has proven to work well. (The wildcard file does not mention your bundle identifier at all, and just uses `"*"` for that entry).

**Note** for the paid apple developer account - with the developer account this step may be skipped alltogether, provision profiles could be downloaded from the iOS developer portal.

### .scent file (maybe)

**Note** on MacOS 14 (Sonoma), `codesign` does not support `.scent` files and you should provide the `application-identifier` some other way. You can check if this option is available by running `codesign --sent`. If the error says "unrecognized option `--sent'" then see below for how to provide an **Entitlements** file instead.

`.scent` file contains all the metadata for signing the bundle.

First, get a team id (the team is a thing created in xcode in the previous step).

```sh
> cat mygame_provisionsfetch.xcodeproj/project.pbxproj | grep DEVELOPMENT_TEAM
DEVELOPMENT_TEAM = YOURID;
```

_mygame_provisionsfetch.xcodeproj_ is our dummy provisions-fetching project.

Than create a `.scent` file next to the `MyGame.app` folder:

```sh
cat > MyGame.scent << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>application-identifier</key>
	<string>MYTEAMID.com.mygame</string>
	<key>com.apple.developer.team-identifier</key>
	<string>MYTEAMID</string>
	<key>get-task-allow</key>
	<true/>
	<key>keychain-access-groups</key>
	<array>
		<string>MYTEAMID.com.mygame</string>
	</array>
</dict>
</plist>
EOF
```

```
├── MyGame.app
│   ├── embedded.mobileprovision
│   ├── mygame
│   ├── Info.plist
│   ├── assets
│   │   ├── texture.png
│   │   ├── someronfile.ron
├── MyGame.scent
```

One way to find what to use for `MYTEAMID` is in the provisioning profile file. Look for `TeamIdentifier` in the output of this command:

```sh
security cms -D -i MyGame.app/embedded.mobileprovision
```

(For cleaner output than a plain `cat` will give you.)

### Entitlements file (unless `.scent` file worked)

For `codesign` to be successful, it needs to know about the `application-identifier`. If you are using macOS 14+ (Sonoma) you probably should use an Entitlements file instead of a `.scent` one.

The `application-identifier` is a concatenation of your team identifier (`MYTEAMID` above) and your bundle identifier. Create an XML file somewhere (e.g. in the crate root named `MyGame.entitlements.xml`) with this content:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>application-identifier</key>
  <string>MYTEAMID.com.mygame</string>
</dict>
</plist>
```

### sign it

**Before** macOS 14 (Sonoma):

```sh

# this should show the certificate previously created in xcode
# copy the very long hex id from the certificate
> security find-identity -v -p codesigning

# and use it here to sign the binary
> codesign --force --timestamp=none --sign LONGHEXID mygame

# and sign the bundle itself
> codesign --force --timestamp=none --sign LONGHEXID --scent MyGame.scent --generate-entitlement-der MyGame.app
```

For Sonoma finding the identity (`LONGHEXID`) is the same as above. But you only need to sign the bundle, and you should use the entitlements file instead of the scent one:

```sh
codesign --force --timestamp=none --sign LONGHEXID --entitlements MyGame.entitlements.xml MyGame.app
```

### deploy it

```sh
> ios-deploy -c
> ios-deploy -i HEXDEVICEID -b MyGame.app
```

# Build scripts

When all the groundwork is done those scripts will deploy run the game on the simulator/real device.

## Emulator

```sh
cargo build --target x86_64-apple-ios
# if assets are updated all the time - copy them on each build
cp -r assets MyGame.app/
# note debug/release
cp target/x86_64-apple-ios/debug/mygame MyGame.app/mygame
xcrun simctl install booted MyGame.app/
xcrun simctl launch booted com.mygame
```

## Real device

```sh
cargo build --target aarch64-apple-ios --release
# if assets are updated all the time - copy them on each build
cp -r assets MyGame.app/
# note debug/release
cp target/aarch64-apple-ios/release/mygame MyGame.app/mygame
codesign --force --timestamp=none --sign VERYLONGHEXID MyGame.app/mygame
codesign --force --timestamp=none --sign VERYLONGHEXID --scent MyGame.scent --generate-entitlement-der MyGame.app
ios-deploy -i HEXDEVICEID -b MyGame.app
```

**Note** on macOS 14 (Sonoma), replace both `codesign` invokations with:

```sh
codesign --force --timestamp=none --sign VERYLONGHEXID --entitlements MyGame.entitlements.xml MyGame.app
```

