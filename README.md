# Discord-Rust

An Android-first, native Rust chat client prototype inspired by familiar Discord mobile navigation.

## Project direction

The goal is a lightweight Android client with a familiar server/channel/chat layout and performance-conscious Rust code. The uploaded Discord APK is used only as a visual and behavioural reference; this repository does not embed, patch, decompile, or redistribute Discord's proprietary application code or assets.

## Current implementation

- Rust application shell using eframe/egui
- Android NativeActivity entry point
- ARM64 Android package metadata (com.discord.rust)
- Responsive mobile layout with server strip, channel drawer, chat view, and bottom navigation
- Desktop preview with server rail, channel list, chat, and member list
- Local preview messages and message composer (not connected to Discord)
- GitHub Actions release APK workflow

## Build

Desktop preview: cargo run

Android APK: cargo apk build --release --lib

The GitHub Actions workflow builds the ARM64 APK. See docs/ANDROID_BUILD.md.

## Discord connectivity

This project does not currently sign in to Discord or exchange live messages. Discord's supported OAuth2/API surface does not provide a general-purpose user-account chat API for third-party clients. OAuth2 identity scopes can support sign-in/profile features; bot and app functionality must use the documented bot/application APIs and permissions. Do not use personal account tokens, self-bot automation, or private undocumented endpoints.

## Status

The UI is an early native Rust foundation, not a complete Discord replacement. Networking, account authorization, persistent local state, attachments, voice, notifications, and real-time message sync remain unimplemented.
