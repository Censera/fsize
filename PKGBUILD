# Maintainer: Censera Censera@proton.me
pkgname=fsize
pkgver=0.1.0
pkgrel=1
pkgdesc="Display file/directory sizes with human‑friendly output"
arch=('x86_64' 'aarch64')
url="https://github.com/censera/fsize"
license=('MIT')
depends=('glibc')
makedepends=('cargo' 'rust')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --release --locked
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    install -Dm755 target/release/fsize "$pkgdir/usr/bin/fsize"
    ln -s fsize "$pkgdir/usr/bin/file"
    ln -s fsize "$pkgdir/usr/bin/filesize"
    ln -s fsize "$pkgdir/usr/bin/fs"
}
