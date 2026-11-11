# Discord-Rust

An Android-first Rust client prototype using egui/eframe.

## Current status

- Rust UI prototype with a Discord-inspired dark theme
- Desktop layout with server rail, channel list, chat and member panel
- Narrow-screen Android layout with compact server and channel navigation
- Android NativeActivity entry point
- APK metadata for com.discord.rust
- GitHub Actions workflow for an arm64 Android APK

## Build desktop preview

    cargo run

## Build Android APK

See docs/ANDROID_BUILD.md. The quickest option is the Android APK workflow in the Actions tab.

## Package

- Display name: Discord Rust
- Application ID: com.discord.rust
- ABI: arm64-v8a

## Important

This repository contains an independent implementation, not Discord's proprietary source code or APK. It is not affiliated with Discord. The current app is a visual prototype and does not yet connect to Discord accounts or live chat. Future connectivity must use supported Discord OAuth2/API features, not personal user tokens or self-bot automation.
