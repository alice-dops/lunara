# Maintainer:
pkgname=lunara-git
pkgver=0.0.1
pkgrel=1
pkgdesc="Home TV menue"
arch=('x86_64' 'aarch64')
url="https://github.com/<user>/<project>"
license=('MIT')
depends=('cairo' 'desktop-file-utils' 'gdk-pixbuf2' 'glib2' 'gtk3' 'hicolor-icon-theme' 'libsoup' 'pango' 'webkit2gtk-4.1')
makedepends=('git' 'openssl' 'appmenu-gtk-module' 'libappindicator-gtk3' 'librsvg' 'cargo' 'pnpm' 'nodejs')
provides=('<pkgname>')
conflicts=('<binname>' '<pkgname>')
source=("git+${url}.git")
sha256sums=('SKIP')

pkgver() {
  cd <project>
  ( set -o pipefail
    git describe --long --abbrev=7 2>/dev/null | sed 's/\([^-]*-g\)/r\1/;s/-/./g' ||
    printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short=7 HEAD)"
  )
}

prepare() {
  cd <project>
  pnpm install
}

build() {
  cd <project>
  pnpm tauri build -b deb
}

package() {
  cp -a <project>/src-tauri/target/release/bundle/deb/<project>_${pkgver}_*/data/* "${pkgdir}"
}
