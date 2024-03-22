{ pkgs ? import <nixpkgs> {} }:

pkgs.stdenv.mkDerivation rec {
  pname = "Calcifer";
  version = "1.0";
  src = pkgs.fetchFromGitHub {
    owner = "WanderingPenwing";
    repo = "Calcifer";
    rev = "v${version}";
    sha256 = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
  };

  buildInputs = [ pkgs.rustc pkgs.cargo ];

  nativeBuildInputs = [ pkgs.pkg-config ];

  cargoSha256 = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

  installPhase = ''
    cargo install --path . --root $out
  '';

  meta = with pkgs.stdenv.lib; {
    description = "MyApp - A simple desktop application written in Rust";
    homepage = "https://github.com/your-github-username/myapp";
    license = licenses.mit;
    maintainers = [ maintainers.your-name ];s
  };
}