{
  lib,
  pkgs,
  crane,
  ...
}:
let
  craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.stable.latest.default);
  src = craneLib.cleanCargoSource ../.;

  commonArgs = {
    inherit src;

    strictDeps = true;
    __structuredAttrs = true;
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  individualCrateArgs = commonArgs // {
    inherit cargoArtifacts;
    inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
    # no Checks
    doCheck = false;
  };

  fileSetForCrate =
    crate:
    lib.fileset.toSource {
      root = ../.;
      fileset = lib.fileset.unions [
        ../Cargo.toml
        ../Cargo.lock
        (craneLib.fileset.commonCargoSources ../common)
        (craneLib.fileset.commonCargoSources crate)
      ];
    };

  outPath = crate: "target/*/build/*/out";

  installCompletionsForCrate = crate: ''
    installShellCompletion --cmd ${crate} \
        --nushell ${outPath crate}/completions/${crate}.elv \
        --bash ${outPath crate}/completions/${crate}.bash \
        --fish ${outPath crate}/completions/${crate}.fish \
        --zsh  ${outPath crate}/completions/_${crate}
  '';

  installManPagesForCrate = crate: ''
    installManPage ${outPath crate}/man/*
  '';

  kymenu-dir = craneLib.buildPackage (
    individualCrateArgs
    // (
      let
        pname = "kymenu-dir";
      in
      {
        inherit pname;
        cargoExtraArgs = "-p ${pname}";
        src = fileSetForCrate ../kymenu-dir;
        meta.mainProgram = pname;

        nativeBuildInputs = [ pkgs.installShellFiles ];
        postInstall = toString [
          (installCompletionsForCrate pname)
          (installManPagesForCrate pname)
        ];
      }
    )
  );

  checks = {
    inherit kymenu-dir;

    kymenu-extras-clippy = craneLib.cargoClippy (
      commonArgs
      // {
        inherit cargoArtifacts;
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      }
    );
  };

  devShell = craneLib.devShell {
    checks = checks;
  };

  packages = {
    inherit kymenu-dir;
  };
in
{
  inherit checks packages devShell;
}
