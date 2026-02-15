#!/bin/bash

# Configuration
APP_NAME="WPMeter"
APP_ID="com.jithin.wpmeter"
TARGET_DIR="target/release"
BUNDLE_DIR="$TARGET_DIR/os_bundle"
INSTALL_DIR="$BUNDLE_DIR/$APP_NAME"
BIN_DIR="$INSTALL_DIR/usr/bin"
DESKTOP_DIR="$INSTALL_DIR/usr/share/applications"
AUTOSTART_DIR="$INSTALL_DIR/etc/xdg/autostart"

# Ensure clean state
rm -rf "$BUNDLE_DIR"
mkdir -p "$BIN_DIR"
mkdir -p "$DESKTOP_DIR"
mkdir -p "$AUTOSTART_DIR"

# Build the project
echo "Building release binary..."
cargo build --release

# Copy binary
echo "Bundling application..."
cp "$TARGET_DIR/$APP_NAME" "$BIN_DIR/"

# Create .desktop file (application launcher entry)
cat > "$DESKTOP_DIR/$APP_ID.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=$APP_NAME
Comment=A minimal WPM typing speed meter for the system tray
Exec=$APP_NAME
Terminal=false
Categories=Utility;
StartupNotify=false
EOF

# Create autostart entry (so it launches on login, like LSUIElement on macOS)
cat > "$AUTOSTART_DIR/$APP_ID.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=$APP_NAME
Comment=A minimal WPM typing speed meter for the system tray
Exec=$APP_NAME
Terminal=false
X-GNOME-Autostart-enabled=true
StartupNotify=false
EOF

echo ""
echo "Done! Linux bundle created at $INSTALL_DIR"
echo ""
echo "To install system-wide, run:"
echo "  sudo cp $BIN_DIR/$APP_NAME /usr/bin/"
echo "  sudo cp $DESKTOP_DIR/$APP_ID.desktop /usr/share/applications/"
echo "  sudo cp $AUTOSTART_DIR/$APP_ID.desktop /etc/xdg/autostart/"
echo ""
echo "Or to install for current user only:"
echo "  cp $BIN_DIR/$APP_NAME ~/.local/bin/"
echo "  cp $DESKTOP_DIR/$APP_ID.desktop ~/.local/share/applications/"
echo "  mkdir -p ~/.config/autostart"
echo "  cp $AUTOSTART_DIR/$APP_ID.desktop ~/.config/autostart/"
