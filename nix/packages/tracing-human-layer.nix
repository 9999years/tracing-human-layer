{
  pkgs,
  pkgsStatic,
  lib,
  stdenv,
  inputs,
  rustPlatform,
  rust-analyzer,
  cargo-release,
  cargo-criterion,
}: let
  inherit (inputs) crane advisory-db;
  craneLib = crane.mkLib pkgs;

  commonArgs' = {
    src = craneLib.cleanCargoSource (craneLib.path ../../.);

    nativeBuildInputs = lib.optionals stdenv.isDarwin [
      # Additional darwin specific inputs can be set here
      pkgsStatic.libiconv
    ];
  };

  # Build *just* the cargo dependencies, so we can reuse
  # all of that work (e.g. via cachix) when running in CI
  cargoArtifacts = craneLib.buildDepsOnly commonArgs';

  commonArgs =
    commonArgs'
    // {
      inherit cargoArtifacts;
    };

  checks = {
    tracing-human-layer-tests = craneLib.cargoNextest (commonArgs
      // {
        NEXTEST_HIDE_PROGRESS_BAR = "true";
      });
    tracing-human-layer-clippy = craneLib.cargoClippy (commonArgs
      // {
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      });
    tracing-human-layer-rustdoc = craneLib.cargoDoc (commonArgs
      // {
        cargoDocExtraArgs = "--document-private-items";
        RUSTDOCFLAGS = "-D warnings";
      });
    tracing-human-layer-fmt = craneLib.cargoFmt commonArgs;
    tracing-human-layer-audit = craneLib.cargoAudit (commonArgs
      // {
        inherit advisory-db;
      });
  };

  devShell = craneLib.devShell {
    inherit checks;

    # Make rust-analyzer work
    RUST_SRC_PATH = rustPlatform.rustLibSrc;

    # Extra development tools (cargo and rustc are included by default).
    packages = [
      rust-analyzer
      cargo-release
      cargo-criterion
    ];
  };
in
  # Build the actual crate itself, reusing the dependency
  # artifacts from above.
  craneLib.buildPackage (commonArgs
    // {
      # Don't run tests; we'll do that in a separate derivation.
      # This will allow people to install and depend on `tracing-human-layer`
      # without downloading a half dozen different versions of GHC.
      doCheck = false;

      # Only build `tracing-human-layer`, not the test macros.
      cargoBuildCommand = "cargoWithProfile build";

      passthru = {
        inherit
          checks
          devShell
          commonArgs
          craneLib
          ;
      };
    })
