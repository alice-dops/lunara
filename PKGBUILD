# Maintainer:
pkgname=lunara-git
pkgver=0.2.0
pkgrel=1
pkgdesc="Home TV menue"
arch=('x86_64' 'aarch64')
url="https://gitlab.malice.home/malice/lunara"
license=('MIT')
depends=('cairo' 'desktop-file-utils' 'gdk-pixbuf2' 'glib2' 'gtk3' 'hicolor-icon-theme' 'libsoup' 'pango' 'webkit2gtk-4.1' 'xdotool')
makedepends=('git' 'openssl' 'appmenu-gtk-module' 'libappindicator-gtk3' 'librsvg' 'cargo' 'npm' 'nodejs')
provides=('lunara')
# source=("git+${url}.git")
# sha256sums=('SKIP')

# pkgver() {
#   cd lunara
#   ( set -o pipefail
#     git describe --long --abbrev=7 2>/dev/null | sed 's/\([^-]*-g\)/r\1/;s/-/./g' ||
#     printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short=7 HEAD)"
#   )
# }
#
prepare() {
  pwd
  # cd lunara
  npm install
}

build() {
  # cd lunara
  npm run tauri build -- -b deb
}

package() {
  cd ..
  cp -a src-tauri/target/release/bundle/deb/lunara_${pkgver}_*/data/* "${pkgdir}"
}
