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
sha256sums=('4feae8a8fbbd579a9145cffd1645ef17ea5f98311a04e60b491279ae21f2107e')


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

