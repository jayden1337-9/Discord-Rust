# Android build

## App identity

- Application ID: com.discord.rust
- Display name: Discord Rust
- ABI: arm64-v8a
- Minimum Android: API 23
- Target Android: API 35

## Build with cargo-apk

Install Rust, the Android SDK and NDK, then install the Android target and APK builder:

    rustup target add aarch64-linux-android
    cargo install cargo-apk

Set ANDROID_HOME and NDK_HOME to the installed SDK and NDK directories. From the repository root:

    cargo apk build --release --lib

The workflow currently verifies the output at:

    target/release/apk/discord-rust.apk

## GitHub Actions

The workflow in .github/workflows/android.yml builds the ARM64 APK on GitHub and uploads it as the discord-rust-arm64-build artifact. It restores the existing release keystore from the ANDROID_KEYSTORE_BASE64 and ANDROID_KEYSTORE_PASSWORD repository secrets. Keep both secrets private and preserve the keystore for future updates.

## Current limitations

The application launches a Rust/egui interface with local preview messages. It is not yet connected to Discord. The official APK is not a source project and cannot be converted wholesale into Rust; this repository implements its own client. Use supported Discord OAuth2 and application/bot API features only. Discord does not expose a general-purpose OAuth user-account endpoint for reading and sending ordinary user messages.
