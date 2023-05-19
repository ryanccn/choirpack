# Choirpack

Choirpack is a CLI for switching package managers in Node.js, built in Go.

It supports npm, yarn, and pnpm, and it interfaces with corepack (hence the name).

## Installation

### Global install

```bash
$ curl -fsSL https://raw.githubusercontent.com/ryanccn/choirpack/main/install.sh | sh
```

Alternatively, you can manually download the artifact corresponding to your platform from the latest commit.

### Nix

```bash
$ nix profile install .
$ nix-env -iA packages.<system>.default
```

```nix
{pkgs, choirpack}: {
  nixpkgs.overlays = [choirpack.overlays.default];
  environment.systemPackages = [pkgs.choirpack];
}
```

### Go (not recommended)

If you want, you can also install Choirpack with Go, although this is not recommended:

```bash
$ go install github.com/ryanccn/choirpack
```

## Usage

In a Node.js project, simply run `choirpack use <package manager>`.

It will configure everything for you.

## Roadmap

- [ ] Support different Yarn versions
