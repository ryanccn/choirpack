{
  description = "easily switch between nodejs package managers";

  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    version = builtins.substring 0 8 self.lastModifiedDate;

    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    packageFn = pkgs: let
      inherit (pkgs.lib) licenses maintainers;
    in {
      choirpack = pkgs.buildGoModule rec {
        pname = "choirpack";
        inherit version;

        src = builtins.path {
          name = "${pname}-src";
          path = ./.;
        };

        vendorHash = "sha256-U4/729PMaKBCVPEtiCp1G85qbwnMibqzstIza0gSM7Q=";

        meta = {
          description = "easily switch between nodejs package managers";
          homepage = "https://github.com/ryanccn/${pname}";
          license = licenses.agpl3;
          maintainers = [maintainers.getchoo];
        };
      };
    };

    forAllSystems = nixpkgs.lib.genAttrs systems;
    nixpkgsFor = forAllSystems (system: import nixpkgs {inherit system;});
  in {
    devShells = forAllSystems (s: let
      pkgs = nixpkgsFor.${s};
      inherit (pkgs) mkShell;
    in {
      default = mkShell {
        packages = [pkgs.go];
      };
    });

    packages = forAllSystems (s: let
      p = packageFn nixpkgsFor.${s};
    in
      p // {default = p.choirpack;});

    overlays.default = _: prev: (packageFn prev);
  };
}
