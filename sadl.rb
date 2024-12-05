class Sadl < Formula
  desc "saltymedia downloader in rust"
  homepage "https://github.com/CooperDActor-bytes/sadl"
  url "https://github.com/CooperDActor-bytes/sadl/archive/v1.0.0.tar.gz"
  sha256 "6537b176396f154580ebe55d20b340e67a9079f50531ef2c28778d06958c7621"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release"
    bin.install "target/release/sadl"
    man1.install "man/sadl.1"
  end

  test do
    system "#{bin}/sadl", "--version"
  end
end
