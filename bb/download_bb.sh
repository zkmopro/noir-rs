#!/bin/sh

# Exit on error
set -e

# OUT_DIR is specified by the rust build environment
if [ -z $OUT_DIR ]; then
    echo "OUT_DIR not specified"
    exit 1
fi
# TARGET is specified by the rust build environment
if [ -z $CARGO_CFG_TARGET_OS ]; then
    echo "CARGO_CFG_TARGET_OS not specified"
    exit 1
fi
BUILD_DIR=$OUT_DIR/bb
mkdir -p $BUILD_DIR

download_and_unzip() {
    local target="$1"
    local zip_file="$BUILD_DIR/$target.zip"
    
    echo "Downloading $target..."
    
    # Download file with error handling
    if ! curl -L -o "$zip_file" "https://bb.zkmopro.org/bb-$target.tar.gz"; then
        echo "Failed to download $target.zip"
        return 1  # Return failure status
    fi
    
    echo "Unzipping $zip_file..."
    
    # Unzip with error handling
    if ! tar -xzf "$zip_file" -C "$BUILD_DIR"; then
        echo "Failed to unzip $zip_file"
        return 1
    fi
    
    echo "✅ Successfully downloaded and extracted $target.zip"
}

# Try downloading the full target
if ! download_and_unzip "$CARGO_CFG_TARGET_OS"; then
    
    echo "Download failed for $CARGO_CFG_TARGET_OS"
    exit 1  # Exit the script with failure
    
fi