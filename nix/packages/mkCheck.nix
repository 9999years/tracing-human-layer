{
  stdenv,
  lib,
  rustPlatform,
  cargo,
  rustc,
}:
{
  name,
  checkPhase,
  checkInputs ? [ ],
  derivationArgs ? { },
}:
stdenv.mkDerivation (
  {
    name = "${name}-check";

    src = lib.cleanSource ../../.;

    inherit checkInputs;

    phases = [
      "unpackPhase"
      "checkPhase"
      "installPhase"
    ];

    inherit checkPhase;
    doCheck = true;

    installPhase = ''
      touch $out
    '';
  }
  // builtins.removeAttrs derivationArgs [
    "checkInputs"
  ]
)
