{
  description = "Shared Harness Nexus events and state transitions.";
  inputs.base.url = "github:LiGoldragon/build-base";
  inputs.ethos.url = "github:LiGoldragon/ethos-zero";
  outputs = { self, base, ethos }: base.lib.workspaceOutputs {
    cargoNix = ./Cargo.nix;
    extraChecks = { pkgs, toolchain, ... }: {
      fmt = pkgs.runCommand "harness-nexus-library-fmt" { nativeBuildInputs = [ toolchain ]; } ''
        rustfmt --edition 2024 --check ${./src}/lib.rs ${./src}/identify.rs ${./src}/transition.rs
        touch "$out"
      '';
      generated = pkgs.runCommand "harness-nexus-library-generated" {
        nativeBuildInputs = [ ethos.packages.${pkgs.stdenv.hostPlatform.system}.ethos-zero ];
      } ''
        ethos-zero "Generate.{ ${./ethos}/library.ethos $PWD/generated }"
        diff -u ${./src/generated/library.rs} generated/library.rs
        touch "$out"
      '';
    };
    memberChecks = { check, ... }: {
      harness-nexus-library = check "harness-nexus-library" [ "datom" ];
    };
  };
}
