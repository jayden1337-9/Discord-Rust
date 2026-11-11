# Android build notes

This starter currently provides a desktop egui preview. Android packaging is not
wired up yet. The Android integration must be selected and tested before claiming
that this repository produces a working APK.

For Android, verify:
- target ABI (likely aarch64-linux-android for a 64-bit ARM phone)
- Android NDK version and API level
- Android activity/entry-point integration supported by the chosen egui stack
- APK packaging and signing
- runtime permissions and network security configuration

Do not put signing keys or passwords in Git.
