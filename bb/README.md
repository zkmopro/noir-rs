# bb.rs

Rust bindings for Barretenberg C++ codebase.

## Build

Before you build, make sure the toolchain for your target platform is installed already.

```sh
# Show which toolchain will be used in the current directory
rustup show
```

If the toolchain is not installed, you can add it using `rustup target add <target platform>`.

```sh
# Example: Add `aarch64-apple-ios-sim` target
rustup target add aarch64-apple-ios-sim
```

See [here](https://github.com/zkmopro/noir-rs/tree/main?tab=readme-ov-file#platform-support) for the list of supported platforms.

#### Build Commands

```sh
# Build on your own machine
cargo build -vvvv

# Cross-compile for iOS
cargo build -vvvv --target <target iOS platform>

# Cross-compile for Android
cargo build -vvvv --target <target Android platform>
```

## Known issues

### Missing `sys/random.h`

random.h is not available in the iOS SDK includes but it is available in the MacOS SDK includes. So you can copy it from `/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys` and paste it in `/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk/usr/include/sys`. This will work, no compability issues, it's just not there for some reason.

You can also run `scripts/patcher.sh` to do this (you may need to run it as `sudo`).
