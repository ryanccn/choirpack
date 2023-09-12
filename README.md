# Choirpack

Simple and powerful Node.js package manager utilities built around [Corepack](https://github.com/nodejs/corepack).

## Installation

Use the package/overlay from the Nix flake or download a binary from GitHub Releases.

## Usage

### `choirpack use`

Use a package manager for a Node.js project.

`--version` can be provided to pin a specific version; otherwise, the latest version if fetched from NPM.

`--no-install` prevents Choirpack from automatically reinstalling your dependencies with the new package manager.

### `choirpack update`

Infers your current package manager automatically from your `package.json` or lockfiles in the project folder and updates the version in the `packageManager` key.

## Bun support

[Bun](https://bun.sh/) can be a great package manager, but at the moment Corepack does not support it. Therefore, Choirpack does not work with Bun at the moment. You can track upstream progress at https://github.com/nodejs/corepack/issues/295!

## License

GPLv3
