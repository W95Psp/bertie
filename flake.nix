{
  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {flake-utils, nixpkgs, rust-overlay, ...}:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };
      in rec {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # openssl
            # pkgconfig
            # exa
            # fd
            pkgs.yq
            pkgs.clang
            pkgs.difftastic
            # rust-bin.beta.latest.default
            rust-analyzer
            # (rust-bin.nightly."2022-02-25".default.override {
            # (rust-bin.nightly."2021-11-15".default.override {
            # (rust-bin.stable."1.57.0".default.override {
            (rust-bin.nightly."2022-07-04".default.override {
              extensions = [
                "rustc-dev" "llvm-tools-preview"
                "rust-analysis"
                # "rust-analyzer-preview"
                # "rls"
                "rust-src"
                # cargo clippy clippy-preview llvm-tools-preview reproducible-artifacts rls rls-preview rust rust-analysis rust-docs rust-src rust-std rustc rustc-dev rustc-docs rustfmt rustfmt-preview

              ];
            })
          ];
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          # shellHook = ''
          # '';
        };
      }
    );
}
