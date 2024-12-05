class Sadl < Formula
  desc "saltymedia downloader in rust"
  homepage "https://github.com/CooperDActor-bytes/sadl"
  url "https://github.com/CooperDActor-bytes/sadl/archive/v1.0.0.tar.gz"
  sha256 "your-archive-sha256-checksum"

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
