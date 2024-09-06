#!/bin/bash

# Set the project name (assuming the binary will be named after the project)
PROJECT_NAME="bimble"

# Delete the bin directory if it exists
if [ -d "./bin" ]; then
    echo "Removing existing bin directory..."
    rm -rf ./bin
fi

# Create a new bin directory
echo "Creating bin directory..."
mkdir -p ./bin

# Build for Linux (assuming you're running on Linux or using cross-compilation)
echo "Building for Linux..."
cargo build --release --target x86_64-unknown-linux-gnu
if [ $? -ne 0 ]; then
    echo "Linux build failed."
    exit 1
fi
cp target/x86_64-unknown-linux-gnu/release/$PROJECT_NAME ./bin/$PROJECT_NAME-linux

# Build for Windows
echo "Building for Windows..."
cargo build --release --target x86_64-pc-windows-gnu
if [ $? -ne 0 ]; then
    echo "Windows build failed."
    exit 1
fi
cp target/x86_64-pc-windows-gnu/release/$PROJECT_NAME.exe ./bin/$PROJECT_NAME-windows.exe
cp ./instructions.txt ./bin
# Build for macOS
##echo "Building for macOS..."
##cargo build --release --target x86_64-apple-darwin
##if [ $? -ne 0 ]; then
##    echo "macOS build failed."
##    exit 1
##fi
##cp target/x86_64-apple-darwin/release/$PROJECT_NAME ./bin/$PROJECT_NAME-macos

##echo "Build completed successfully. Binaries are in the bin/ directory."
