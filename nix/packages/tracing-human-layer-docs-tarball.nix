{
  stdenv,
  tracing-human-layer-docs,
}: let
  inherit (tracing-human-layer-docs) version;
in
  stdenv.mkDerivation {
    pname = "tracing-human-layer-docs-tarball";
    inherit version;

    src = tracing-human-layer-docs;

    dontConfigure = true;
    dontBuild = true;

    installPhase = ''
      dir=tracing-human-layer-docs-${version}
      mv share/doc \
        "$dir"

      mkdir $out
      tar --create \
        --file $out/tracing-human-layer-docs-${version}.tar.gz \
        --auto-compress \
        --verbose \
        "$dir"
    '';
  }
