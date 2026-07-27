#!/bin/bash
set -e

[[ $# -eq 1 ]] || { echo "Provide package version (MUST START WITH v)"; exit 1; }
[[ "$1" == v* ]] || { echo "Version must start with 'v'"; exit 1; }

PACKAGE="oiff-$1-x86_64-linux"

echo "Installing dependencies..."
rustup update
cargo update
cargo fmt
cargo install cargo-deb cargo-generate-rpm
cargo build --release

echo "Packaging .deb package..."
cargo deb

echo "Packaging .rpm package..."
cargo generate-rpm

echo "Packaging binary..."
mkdir -p "$PACKAGE"
cp target/release/oiff "$PACKAGE/$PACKAGE"

echo "Packaging .zip package..."
mkdir -p "$PACKAGE/usr/bin"
mkdir -p "$PACKAGE/usr/share/applications"
mkdir -p "$PACKAGE/usr/share/icons/hicolor/scalable/apps"
mkdir -p "$PACKAGE/usr/share/mime/packages"
mkdir -p "$PACKAGE/usr/share/thumbnailers"

cp target/release/oiff "$PACKAGE/usr/bin/oiff"
cp oiff-viewer/oiff-viewer.desktop "$PACKAGE/usr/share/applications/oiff-viewer.desktop"
cp oiff-viewer/oiff-viewer.svg "$PACKAGE/usr/share/icons/hicolor/scalable/apps/oiff-viewer.svg"
cp thumbnailer/oiff.thumbnailer "$PACKAGE/usr/share/thumbnailers/oiff.thumbnailer"
cp thumbnailer/oiff.xml "$PACKAGE/usr/share/mime/packages/oiff.xml"

cat << 'EOF' > "$PACKAGE/install.sh"
#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "Press \"enter\"/\"return\" to install oiff (ROOT privileges required)"
read -r

echo "Copying files to system directories..."

echo copying oiff...
sudo cp "$SCRIPT_DIR/usr/bin/oiff" /usr/local/bin/oiff

echo copying oiff viewer...
sudo cp "$SCRIPT_DIR/usr/share/applications/oiff-viewer.desktop" /usr/share/applications/oiff-viewer.desktop
sudo cp "$SCRIPT_DIR/usr/share/icons/hicolor/scalable/apps/oiff-viewer.svg" /usr/share/icons/hicolor/scalable/apps/oiff-viewer.svg

echo copying thumbnailer...
sudo cp "$SCRIPT_DIR/usr/share/thumbnailers/oiff.thumbnailer" /usr/share/thumbnailers/oiff.thumbnailer
sudo cp "$SCRIPT_DIR/usr/share/mime/packages/oiff.xml" /usr/share/mime/packages/oiff.xml

echo "Updating system databases..."
rm -rf ~/.cache/thumbnails/*
sudo update-mime-database /usr/share/mime
sudo update-desktop-database /usr/share/applications

echo "Installation complete!"
EOF

cat << 'EOF' > "$PACKAGE/uninstall.sh"
#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "Press \"enter\"/\"return\" to uninstall oiff (ROOT privileges required)"
read -r

echo "Removing files from system directories..."

echo removing oiff...
sudo rm -f /usr/local/bin/oiff

echo removing oiff viewer...
sudo rm -f /usr/share/applications/oiff-viewer.desktop
sudo rm -f /usr/share/icons/hicolor/scalable/apps/oiff-viewer.svg

echo removing thumbnailer...
sudo rm -f /usr/share/thumbnailers/oiff.thumbnailer
sudo rm -f /usr/share/mime/packages/oiff.xml

echo "Updating system databases..."
rm -rf ~/.cache/thumbnails/*
sudo update-mime-database /usr/share/mime
sudo update-desktop-database /usr/share/applications

echo "Uninstallation complete!"
EOF

chmod +x "$PACKAGE/install.sh" "$PACKAGE/uninstall.sh"

zip -r "$PACKAGE.zip" "$PACKAGE"
rm -rf "$PACKAGE"

echo -e "Copying install packages to \"${HOME}/Downloads/${PACKAGE}\""
DOWNLOADS_DIR="$HOME/Downloads/$PACKAGE"
mkdir -p "$DOWNLOADS_DIR"

mv "$PACKAGE.zip" "$DOWNLOADS_DIR/"

cp target/release/oiff "$DOWNLOADS_DIR/$PACKAGE"

mv target/debian/*.deb "$DOWNLOADS_DIR/$PACKAGE.deb"
mv target/generate-rpm/*.rpm "$DOWNLOADS_DIR/$PACKAGE.rpm"

echo Done
