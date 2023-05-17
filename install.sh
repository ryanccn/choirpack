#!/usr/bin/env bash
set -eo pipefail

os="$(uname -s | tr '[:upper:]' '[:lower:]')"
arch="$(uname -m)"

install_dir="${CHOIRPACK_INSTALL_DIR:-"$HOME/.local/bin"}"
binary_name="choirpack-$os-$arch"

download_url="https://nightly.link/ryanccn/choirpack/workflows/build/main/choirpack-$os-$arch.zip"
download_path="$(mktemp -d)"

curl --location "$download_url" -o "$download_path/choirpack.zip"
unzip "$download_path/choirpack.zip" -d "$install_dir" &> /dev/null

chmod +x "$install_dir/$binary_name"
mv "$install_dir/$binary_name" "$install_dir/choirpack"
rm -rf "$download_path"

echo "Installed Choirpack to $install_dir!"
