#!/bin/bash

# Exit on any error
set -e

# Define the binary name and path
binary_name="shell-tip"
binary_path="/usr/local/bin/$binary_name"

# Build the binary using Cargo
cargo build --release

# Install thebinary
sudo cp "target/release/$binary_name" "$binary_path"

# Add the directory to the PATH if it does not already exist
if [[ ":$PATH:" != *":/usr/local/bin:"* ]]; then
  echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
  echo "Added /usr/local/bin to PATH"
fi

echo "Installed $binary_name to $binary_path"