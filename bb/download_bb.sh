#!/bin/sh

# Exit on error
set -e

# OUT_DIR is specified by the rust build environment
if [ -z $OUT_DIR ]; then
    echo "OUT_DIR not specified"
    exit 1
fi
# TARGET is specified by the rust build environment
if [ -z $TARGET ]; then
    echo "TARGET not specified"
    exit 1
fi
BUILD_DIR=$OUT_DIR/bb
mkdir -p $BUILD_DIR

download_and_unzip() {
    local target="$1"
    local asset_name="bb_rs-$target.tar.gz"
    local zip_file="$BUILD_DIR/$asset_name"
    local bb_version="0.87.7"
    
    echo "Downloading $asset_name..."
    
    # Download file with error handling
    if ! curl -L -o "$zip_file" "https://github.com/zkmopro/aztec-packages/releases/download/$bb_version/$asset_name"; then
        echo "Failed to download $asset_name"
        return 1  # Return failure status
    fi
    
    echo "Unzipping $zip_file..."
    
    # Unzip with error handling
    if ! tar -xzf "$zip_file" -C "$BUILD_DIR"; then
        echo "Failed to unzip $zip_file"
        return 1
    fi
    
    echo "✅ Successfully downloaded and extracted $asset_name"
}

# Try downloading the full target
if ! download_and_unzip "$TARGET"; then
    echo "Retrying with local architecture..."
    
    local_arch=$(echo "$TARGET" | cut -d'-' -f1)
    
    if ! download_and_unzip "$local_arch"; then
        echo "Download failed for both bb_rs-$TARGET.tar.gz and bb_rs-$local_arch.tar.gz"
        exit 1  # Exit the script with failure
    fi
fi