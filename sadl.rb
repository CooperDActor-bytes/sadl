class Sadl < Formula
  desc "saltymedia downloader - instagram"
  homepage "sadl.saltymedia.stream"
  url "https://github.com/yourusername/sadl/archive/v1.0.0.tar.gz"
  sha256 "your_tarball_sha256"
  license "MIT"

  depends_on "python@3.11"
  depends_on "ffmpeg"

  def install
    bin.install "sadl"
    system "pip3", "install", "yt-dlp"
  end

  test do
    (testpath/"insta.txt").write "https://www.instagram.com/reel/abc123/"
    system "#{bin}/sadl", "-I", "insta.txt"
  end
end

