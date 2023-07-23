#!/bin/bash

install_dir="/usr/local/bin"

if [ "$(id -u)" -ne 0 ]; then
    echo "Please run this script with sudo or as root."
    exit 1
fi

# Build the binary
echo "Building the binary..."
cargo build --release

# Installing on your system
echo "Installing on your system..."
cp ./target/release/pm "$install_dir"

# Check if the copy was successful
if [ $? -eq 0 ]; then
    echo "'pm' has been installed to $install_dir."
else
    echo "Failed to install 'my_project'."
    exit 1
fi

# Add executable permissions to the installed binary
chmod +x "$install_dir/pm"

echo "Installation completed successfully!"