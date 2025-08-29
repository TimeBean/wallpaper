# Maintainer: Your Name <alexanderklush8@gmail.com>
pkgname=wallpaper
pkgver=0.1.0
pkgrel=1
pkgdesc="CLI tool for setting wallpapers and generating color palettes (swww → wallust → matugen)"
arch=('x86_64')
url="https://github.com/TimeBean/wallpaper"
license=('GPL3')
depends=('swww' 'wallust' 'matugen')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/heads/main.tar.gz")
sha256sums=('a9d534ac0b70b2525b3fd86053708268c88a5ddf4d63709ed6fcbafe208615ba')


build() {
  if [ -d "$srcdir/$pkgname-$pkgver" ]; then
    cd "$srcdir/$pkgname-$pkgver"
  else
    cd "$srcdir/$pkgname-main"
  fi
  cargo build --release --locked
}



package() {
  if [ -d "$srcdir/$pkgname-$pkgver" ]; then
    cd "$srcdir/$pkgname-$pkgver"
  else
    cd "$srcdir/$pkgname-main"
  fi
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}

