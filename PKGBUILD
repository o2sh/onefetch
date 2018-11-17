# Maintainer: o2sh <ossama-hjaji@live.fr>
pkgname=onefetch
pkgver=1.0.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="Neofetch for your source code"
license=('MIT')

build() {
    return 0
}

package() {
    cd $srcdir
    cargo install --root="$pkgdir" --git=https://github.com/o2sh/onefetch
}
