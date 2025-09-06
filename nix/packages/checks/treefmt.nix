{
  mkCheck,
  # See: https://github.com/NixOS/nixpkgs/pull/437251
  pkgs,
  nixfmt,
  cargo,
  rustfmt,
  actionlint,
}:

mkCheck {
  name = "treefmt";

  checkInputs = [
    pkgs.treefmt
    cargo
    nixfmt
    rustfmt
    actionlint
  ];

  checkPhase = ''
    treefmt --ci
  '';
}
