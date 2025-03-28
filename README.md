# magic-rs
Experimental Clash of Clans server emulator on top of libg.so (v8.67.8)
![Screenshot](screenshot.jpg)

# Why?
Supercell's games have their game logic implementation included in both client and server, for independent execution (the `Logic*` family of classes is shared). However, to implement a feature-complete server emulator, you should rewrite the logic entirely. This experimental way is to use logic that is retained in the game, basically turning client into a server.

# Current features
- Full home state emulation
- NPC attacks
- Player progress saving

# Implementation
Server side code is written in Rust. We provide idiomatic bindings for the structures/functions from `libg.so`. Player data is being saved inside application data in form of SQLite database (for sake of simplicity).

# Getting started
#### NOTE: you have to use a device with support of armeabi-v7a binaries
### a) Using pre-built apk files
Navigate to the [Releases](https://git.xeondev.com/Supercell/Magic/releases) page and download both Server and Client APK files (by default, they're targeted to the `127.0.0.1:9339` endpoint). Next, install both of them. Open the server application first (it should stay with black screen), then leave it running in the background. Open the client and play!
### b) Building from sources
#### Requirements:
- [Rust 1.85+](https://www.rust-lang.org/tools/install)
- [Android NDK](https://developer.android.com/ndk/downloads)
- [cargo-ndk](https://docs.rs/crate/cargo-ndk/3.5.4)

#### Preparing the toolchain
- Install android armv7 target via rustup:
```sh
rustup target add armv7-linux-androideabi
```
- Install cargo-ndk extension:
```sh
cargo install cargo-ndk
```

##### NOTE: make sure you have configured the `ANDROID_NDK_HOME` environment variable before invoking build command.

#### Compiling server library
```sh
git clone https://git.xeondev.com/Supercell/Magic
cd Magic
cargo ndk -t armeabi-v7a build --release
```

### Community
[Our Discord Server](https://discord.gg/reversedrooms) is open for everyone who's interested in our projects!

### Support
Your support for this project is greatly appreciated! If you'd like to contribute, feel free to send a tip [via Boosty](https://boosty.to/xeondev/donate)!
