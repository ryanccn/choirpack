#!/usr/bin/env bash
set -eo pipefail

os="$(uname -s | tr '[:upper:]' '[:lower:]')"
arch="$(uname -m)"

binary_name="choirpack-$os-$arch"

download_url="https://nightly.link/ryanccn/choirpack/workflows/build/main/choirpack-$os-$arch.zip"
download_path="$(mktemp -d)"

curl --location "$download_url" -o "$download_path/choirpack.zip"
unzip "$download_path/choirpack.zip" -d "$HOME/.local/bin"

chmod +x "$HOME/.local/bin/$binary_name"
mv "$HOME/.local/bin/$binary_name" "$HOME/.local/bin/choirpack"
rm -rf "$download_path"

echo "Installed Choirpack to ~/.local/bin!"
