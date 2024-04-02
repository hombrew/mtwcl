#!/bin/bash

# Function to detect the architecture
detect_architecture() {
    ARCH=$(uname -m)
    case $ARCH in
        x86_64)
            if [[ $(uname) == "Darwin" ]]; then
                echo "x86_64-apple-darwin"
            else
                echo "x86_64-unknown-linux-musl"
            fi
            ;;
        arm)
            echo "arm-unknown-linux-musleabihf"
            ;;
        armv7l)
            echo "armv7-unknown-linux-musleabihf"
            ;;
        aarch64)
            echo "aarch64-unknown-linux-musl"
            ;;
        *)
            echo "Unsupported architecture: $ARCH"
            exit 1
            ;;
    esac
}

# Function to detect the user's shell
detect_shell() { 
    SHELL=$(basename "$SHELL")
    echo "$SHELL"
}

# VERSION="v0.1.1"
VERSION="main"

# Define the base URL for downloading binaries
BASE_URL="https://github.com/hombrew/mtwcl/releases/download/$VERSION"

# Detect the architecture
ARCH=$(detect_architecture)

# Define the URL to download the binary
BINARY_URL="$BASE_URL/mtwcl-$ARCH"

# Define the destination directory where you want to install the binary
INSTALL_DIR="/usr/local/bin"

# Define the name of the binary file
BINARY_NAME="mtwcl"

# Download the binary using curl
echo "Downloading $BINARY_NAME for $ARCH..."
curl -L $BINARY_URL -o $BINARY_NAME

# Check if the download was successful
if [ $? -ne 0 ]; then
    echo "Failed to download $BINARY_NAME for $ARCH. Exiting."
    exit 1
fi

# Make the binary executable
chmod +x $BINARY_NAME

# Move the binary to the installation directory
echo "Installing $BINARY_NAME to $INSTALL_DIR..."
sudo mv $BINARY_NAME $INSTALL_DIR

# Check if the move was successful
if [ $? -ne 0 ]; then
    echo "Failed to install $BINARY_NAME. Exiting."
    exit 1
fi

# Detect the user's shell
SHELL_NAME=$(detect_shell)

# Add the installation directory to the PATH
if [[ "$SHELL_NAME" == "bash" ]]; then
    echo "Adding $INSTALL_DIR to PATH in ~/.bashrc..."
    echo "export PATH=\$PATH:$INSTALL_DIR" >> ~/.bashrc
elif [[ "$SHELL_NAME" == "zsh" ]]; then
    echo "Adding $INSTALL_DIR to PATH in ~/.zshrc..."
    echo "export PATH=\$PATH:$INSTALL_DIR" >> ~/.zshrc
else
    echo "Unsupported shell: $SHELL_NAME. Please add $INSTALL_DIR to your PATH manually."
fi

echo "Installation completed successfully."