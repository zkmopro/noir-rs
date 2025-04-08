#!/bin/bash

# Check if a destination directory is provided
if [ $# -ne 1 ]; then
    echo "Usage: $0 <destination_directory>"
    echo "Example: $0 /path/to/destination"
    exit 1
fi

# Get the destination directory from the command line argument
DEST_DIR="$1"

# Define the source directory
SRC_DIR="$(dirname "$(dirname "$(dirname "$0")")")./cpp/src"

# Check if the source directory exists
if [ ! -d "$SRC_DIR" ]; then
    echo "Error: Source directory '$SRC_DIR' does not exist."
    exit 1
fi

# Create the destination directory if it doesn't exist
mkdir -p "$DEST_DIR"

# Find all .hpp and .tcc files in the source directory and copy them to the destination,
# preserving the directory structure
echo "Copying .hpp and .tcc files from $SRC_DIR to $DEST_DIR..."
find "$SRC_DIR" \( -name "*.hpp" -o -name "*.tcc" \) -type f | while read -r file; do
    # Get the relative path from the source directory
    rel_path="${file#$SRC_DIR/}"

    # Create the destination directory structure
    dest_file="$DEST_DIR/$rel_path"
    dest_dir="$(dirname "$dest_file")"
    mkdir -p "$dest_dir"

    # Copy the file
    cp "$file" "$dest_file"
    echo "Copied: $rel_path"
done

echo "Finished copying .hpp and .tcc files to $DEST_DIR"
