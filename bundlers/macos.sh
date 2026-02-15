#!/bin/bash

# Configuration
APP_NAME="WPMeter"
TARGET_DIR="target/release"
BUNDLE_DIR="$TARGET_DIR/os_bundle"
APP_DIR="$BUNDLE_DIR/$APP_NAME.app"
MACOS_DIR="$APP_DIR/Contents/MacOS"
RESOURCES_DIR="$APP_DIR/Contents/Resources"

# Ensure clean state
rm -rf "$BUNDLE_DIR"
mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

# Build the project
echo "Building release binary..."
cargo build --release

# Copy binary
echo "Bundling application..."
cp "$TARGET_DIR/$APP_NAME" "$MACOS_DIR/"

# Create Info.plist
cat > "$APP_DIR/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>$APP_NAME</string>
    <key>CFBundleIdentifier</key>
    <string>com.jithin.$APP_NAME</string>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>LSUIElement</key>
    <true/>
</dict>
</plist>
EOF

echo "Done! App bundle created at $APP_DIR"
