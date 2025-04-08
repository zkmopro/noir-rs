# bb.rs

Rust bindings for Barretenberg C++ codebase.

## Build

```
# Build on your own machine
cargo build -vvvv

# Cross-compile for iOS
cargo build -vvvv --target aarch64-apple-ios

# Cross-compile for Android
cargo build -vvvv --target aarch64-linux-android
```

## Known issues

### Missing `sys/random.h`

random.h is not available in the iOS SDK includes but it is available in the MacOS SDK includes. So you can copy it from `/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys` and paste it in `/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk/usr/include/sys`. This will work, no compability issues, it's just not there for some reason.

You can also run `scripts/patcher.sh` to do this (you may need to run it as `sudo`).



