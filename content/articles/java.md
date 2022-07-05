+++
title = "Java interop with Miniquad"
description = "How to Android runs binaries and how to work with Java libraries from *quads"
date = 2022-07-04T20:10:42+00:00
updated = 2022-07-04T20:10:42+00:00
draft = false
template = "blog/page.html"

[taxonomies]
authors = ["Fedor"]
+++

Miniquad allows seamless integration of Java code, intagrating Java compilation into a rust build pipeline. This allows *quad project to get access to any Android APIs and an option to integrate any third-party Java libraries.

The article use a native file dialog as an example.

Code is available here: 

[https://github.com/not-fl3/example-android-fileopen/](https://github.com/not-fl3/example-android-fileopen/)

More comprehensive example: 

[https://github.com/not-fl3/example-android-bluetooth/](https://github.com/not-fl3/example-android-bluetooth/)

{{ video(src = "https://user-images.githubusercontent.com/910977/177233789-fefbf9c9-6c55-4151-804a-22b85dfa82de.mp4" )}}
*Bluetooth example asking for permissions. Surpisingly, its A LOT of java!*

## How android runs things

First a brief introduction on how applications works on android. A bird's eye view on how apps works on android, to find out where it is possible to insert some calls to get the fial dialog (or any native api, actually) to appear. 

Each android package, .apk, is basically a zip archive with:
- classes.dex file - all the compiled java files
- .so files - all the compiled binary code
- some .xml's with metadata, resources and assets

Android will initialize java virtual machine and run the app inside this virtual machine. All the interactions with the OS goes through the virtual machine. 

There are two ways to start android application, described in one of the .xml's of the apk:
- skip classes.dex completely and tell android to load .so and call a few symbols to let the app initialize.
- find a java class in classes.dex file and let it run.

Miniquad is using the second option - it have MainActivity.java that is responsible for initializing the app and receiving input events. And inside this MainActivity.java .so with user code is being loaded and some native functions from that .so are being called.

Why miniquad is not built around NativeActivity, the first option? Well, it used to be built around NativeActivity until 0.3, actually.
The problem here - some of the android API's are really, really hard to use through java native interface. So it really helps when there is an option to make a wrapper for something in Java and provide an easy to use function for a native code.

*A little remark on apks* Now instead of raw .apk developers will be forced to use .aab - a new format for applications on android. Each .aab is a bunch of signed .apk's, so the idea is still the ssame.

### Writing some java

A short recap from previous section:
- miniquad has MainActivity.java
- it creates an OS window and load .so with user code
- it pass all the events to this .so through calling native functions from .so

The way miniquad's native plugins are built are very similar to UE4 android plugin's idea(just in case the reader is familiar with UE4/Android interop).

We can ask the build system to insert some code right to the MainActivity.java. And we can as the build system to add compile some .java files to get the classes into classes.dex.

Adding code inside MainActivity works in a quite literal way - there is a MainActivity.java template: [MainActivity.java](https://github.com/not-fl3/miniquad/blob/master/java/MainActivity.java). Each crate based on miniquad can ask the build system to add some lines into `//% MAIN_ACTIVITY_ON_RESUME` or `//% whatever`.

Fortunately Java is not against recurring lines in imprts declaration,
```java
import android.app.Activity;
import android.app.Activity;
import android.app.Activity;
```

is totally legit for Java. So each plugin can be independent on each other and add its logic right into the MainActivity, with its own imports and its own initialisation code in Activity OnCreate and so on.

**The plan of building a file dialog crate**

- Make a few .java files with classes helpful for dealing with files  
- Ask to insert code into MainActivity.java to make some initialization on java side
- Call some functions through JNI from our plugin rust code to get the data back

## How to open a file dialog

This part might be generalized into "How to use Intents on Android". 

[An intent is an abstract description of an operation to be performed. ... An Intent provides a facility for performing late runtime binding between the code in different applications.](https://developer.android.com/reference/android/content/Intent)

MainActivity has a method, `startActivityForResult`. This will replace the application's activity with a new activity for the intent and later will send the result back into MainActivity.

To simplify getting data from a callback (hmm), the docs [suggests](https://developer.android.com/training/basics/intents/result) to include 88Mb library with 499 Java files. While this is totally possible (check the bluetooth example, it goes this way), let's try to avoid this for a little dialog.

`startActivityForResult` will send the result into a MainActivity's as a MainActivity's virtual function. Here ability to patch MainActivity comes in handy: [add code into MainActivity](https://github.com/not-fl3/example-android-fileopen/blob/main/java/MainActivity.java).

## Wrapping this into a crate

The goal here - make a crate that provides a `find_file` function. This function opens a dialog and returns a bytes of a file content.

To tell cargo where are the java files: 
[quad.toml](https://github.com/not-fl3/example-android-fileopen/blob/main/quad.toml):

```toml
main_activity_inject = "java/MainActivity.java"
java_files = [
    "java/fileopen/FileOpen.java",
]
```

Java class responsible for the dialog: [FileOpen.java](https://github.com/not-fl3/example-android-fileopen/blob/main/java/fileopen/FileOpen.java#L16)

Most java calls look like 

```
let env = android::attach_jni_env();
ndk_utils::call_void_method!(env, "OpenFileDialog", "()V");
```

Useful links on Rust<->Java interop:

[miniquad's ndk_utils](https://github.com/not-fl3/miniquad/blob/master/src/native/android/ndk_utils.rs)
[JNI functions(available in the "env")](https://docs.oracle.com/javase/7/docs/technotes/guides/jni/spec/functions.html)

## Using the crate 

This part is simple. 
Just include the crate in Cargo.toml: 

```toml
[dependencies]
fileopen = ".."
```

and use it from any miniquad/macroquad project:

```
if root_ui().button(None, "Open file"){
    fileopen::find_file(file_data.clone());
}
```

## Post-scriptum

*Wait is a second, patching java code, calling javac, isn't it all horrible hacks?*

*Yes it is indeed. Would be so happy to throw it all away! But so far its the easiest way to deal with java I ever seen (from a library user perspective, writing java is horrible). No need to download gigabytes of android studio, no build pipeline complications.*
