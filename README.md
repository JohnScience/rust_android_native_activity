# Example of Android native activity written in Rust

Example of approximately constant FPS animation (`frame_no mod 256` brightness) with [`minSdkVersion`](https://developer.android.com/guide/topics/manifest/uses-sdk-element#min) 10+. Tested on LG K61 with Android API level 30.

Source of inspiration: <https://github.com/rust-mobile/rust-android-examples/blob/95c47f206fd5063365ca6fdba5568b7663752fb5/na-mainloop/src/lib.rs>.

## Debugging on a physical phone (via USB or Wi-Fi pairing)

```
cargo apk r
```

Requires [cargo-apk](https://github.com/rust-mobile/cargo-apk).

Also see:

* [Android version vs API level](https://developer.android.com/studio/releases/platforms) on [developer.android.com].
* <https://apilevels.com/> - an overview of all Android versions and their corresponding identifiers for Android developers.
* [`NativeActivity`](https://developer.android.com/reference/android/app/NativeActivity) on [developer.android.com].
* ["Migrate from NativeActivity" (to GameActivity)](https://developer.android.com/games/agdk/game-activity/migrate-native-activity) on [developer.android.com].
* [github.com/rust-mobile/rust-android-examples/agdk-mainloop](https://github.com/rust-mobile/rust-android-examples/tree/95c47f206fd5063365ca6fdba5568b7663752fb5/agdk-mainloop) - example of a pure Rust app using [`GameActivity`](https://developer.android.com/games/agdk/game-activity).
* ["Proof of concept: iOS app written in pure Rust"](https://www.reddit.com/r/rust/comments/qpruup/proof_of_concept_ios_app_written_in_pure_rust/) on [reddit.com].
* <https://github.com/BrainiumLLC/cargo-mobile/> - cargo-mobile takes care of generating Xcode and Android Studio project files, building and running on device, generating project boilerplate, and a few other things. At the time of writing, building for Android is broken on NDK >= 23. Not tested by the author (yet).
* Open issue #302 "Mobile Support": <https://github.com/iced-rs/iced/issues/302>.
* [Unofficial Bevy Cheat Book](bevy-cheatbook.github.io) > [14. Bevy on Different Platforms](https://bevy-cheatbook.github.io/platforms.html) > [Mobile](https://bevy-cheatbook.github.io/platforms.html#mobile).

[developer.android.com]: developer.android.com
[reddit.com]: www.reddit.com
