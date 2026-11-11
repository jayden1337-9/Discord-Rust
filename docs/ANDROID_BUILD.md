# Android build

The project now has an Android NativeActivity entry point and APK metadata.

## App identity

- Application ID: com.discord.rust
- Display name: Discord Rust
- ABI: arm64-v8a
- Minimum Android: API 23
- Target Android: API 35

## Build with cargo-apk

Install Rust, the Android SDK and NDK, then install the Android Rust target and APK builder:

    rustup target add aarch64-linux-android
    cargo install cargo-apk

Set ANDROID_HOME and NDK_HOME to your installed SDK and NDK directories. Then run from the repository root:

    cargo apk build --release --lib

The generated APK is placed under target/android-artifacts/release/apk/.

## Build from GitHub Actions

The Android APK workflow builds the arm64 APK on GitHub and uploads it as a workflow artifact. Open the repository's Actions tab, select the latest successful run, and download the discord-rust-arm64-apk artifact.

## Termux note

The Rust source is Android-ready, but a local Termux build still needs a compatible Android NDK toolchain and SDK setup. The GitHub Actions build is the easiest first APK route and avoids installing the full Android toolchain on the phone.

## Current limitations

This is still a UI prototype. It does not sign users in, connect to Discord Gateway, or send real messages. Use Discord-supported OAuth2 and API access; never put account tokens or signing secrets in source control.
