# Discord-Rust

An Android-first Rust client project. The current code is an **interface prototype**:
it recreates a familiar Discord-style layout using Rust and egui, but it is not yet
connected to Discord accounts or live chat.

## Current features

- Dark chat layout with server rail, channel list, message timeline and member panel
- Channel switching and local message composer prototype
- Modular Rust UI code

## Build a desktop preview

Install Rust, then run:

```sh
cargo run
```

## Android status

This first commit is a desktop egui preview, not a finished Android APK. Android
entry-point integration, NDK packaging, signing, and device testing are still
required. See [Android build notes](docs/ANDROID_BUILD.md).

## Discord connectivity

Use Discord's documented OAuth2 and bot-supported API functionality. Do not use
personal user tokens or automate a normal user account as a self-bot. Discord's
API does not provide a supported way for an independent third-party client to
reproduce every feature of the official mobile app.

## Package target

Planned Android application ID: `com.discord.rust`.

## Layout

```text
src/
  main.rs
  app.rs
  theme.rs
  ui/
    mod.rs
    servers.rs
    channels.rs
    chat.rs
    members.rs
```

The repository contains no Discord APK, proprietary source code, or copied
Discord artwork. Use original assets or assets you have permission to use.
