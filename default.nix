{
  lib,
  stdenv,
  pkg-config,
  rustPlatform,
  installShellFiles,
  CoreFoundation,
  Security,
  IOKit,
  libiconv,
  version,
  self,
  lto ? true,
  optimizeSize ? true,
}: let
  filter = path: type: let
    path' = toString path;
    base = baseNameOf path';

    matches = lib.any (suffix: lib.hasSuffix suffix base) [".rs" ".toml"];
    isLock = base == "Cargo.lock";
  in
    type == "directory" || matches || isLock;

  filterSource = src:
    lib.cleanSourceWith {
      src = lib.cleanSource src;
      inherit filter;
    };
in
  rustPlatform.buildRustPackage {
    pname = "choirpack";
    inherit version;

    src = filterSource self;
    cargoLock.lockFile = ./Cargo.lock;

    RUSTFLAGS =
      lib.optionalString lto " -C lto=fat -C embed-bitcode=yes"
      + lib.optionalString optimizeSize " -C codegen-units=1 -C strip=symbols -C opt-level=z";

    buildNoDefaultFeatures = true;
    buildFeatures = [];

    buildInputs = lib.optionals stdenv.isDarwin [
      CoreFoundation
      Security
      IOKit
      libiconv
    ];

    nativeBuildInputs = [
      pkg-config
      installShellFiles
    ];

    postInstall = ''
      installShellCompletion --cmd choirpack \
        --bash <("$out/bin/choirpack" completions bash) \
        --zsh <("$out/bin/choirpack" completions zsh) \
        --fish <("$out/bin/choirpack" completions fish)
    '';

    meta = with lib; {
      description = "Simple and powerful Node.js package manager utilities built around Corepack";
      maintainers = with maintainers; [ryanccn];
      license = licenses.gpl3Only;
      mainProgram = "choirpack";
    };
  }
