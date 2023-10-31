{tracing-human-layer}: let
  inherit
    (tracing-human-layer)
    craneLib
    commonArgs
    ;
in
  craneLib.cargoDoc (commonArgs
    // {
      # The default `cargoDocExtraArgs` is `--no-deps`.
      cargoDocExtraArgs = "--all-features";
      RUSTDOCFLAGS = "-D warnings";
    })
