# Maintainer: dev-dmitrii-g <gagarindmitrii56@gmail.com>
pkgname=CharTrace
pkgver=1.0.0
pkgrel=1
pkgdesc="A GTK4 app for character search in strings"
arch=('x86_64')
url="https://github.com/dev-dmitrii-g/CharTrace"
license=('MIT')
depends=('gtk4')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/dev-dmitrii-g/$pkgname/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --release
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 "$srcdir/../LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm644 "$srcdir/../README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
}