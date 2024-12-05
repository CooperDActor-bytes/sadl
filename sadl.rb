class Sadl < Formula
  desc "saltymedia downloader in rust"
  homepage "https://github.com/CooperDActor-bytes/sadl"
  url "https://github.com/CooperDActor-bytes/sadl/archive/v1.0.1.tar.gz"
  sha256 "a1140e2117549c19fb063cf2d0dbf26677145d8fff83731051b9b1bee140270f"

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
