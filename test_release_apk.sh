# Build in release mode and adb install to android device

set -e

cargo tauri android build --target aarch64

$ANDROID_BUILD_TOOLS/zipalign -p -f 4 \
  src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk app-aligned.apk

# Sign apk
$ANDROID_BUILD_TOOLS/apksigner sign \
  --ks ~/.android/debug.keystore \
  --ks-key-alias androiddebugkey \
  --ks-pass pass:android \
  --key-pass pass:android \
  --out app-release-signed.apk \
  app-aligned.apk

$ANDROID_BUILD_TOOLS/apksigner verify --print-certs app-release-signed.apk

# Install to device
adb install -r --no-incremental app-release-signed.apk

echo "Launching app..."
$ANDROID_HOME/platform-tools/adb shell am start -n uk.co.joebutton.synthmob/.MainActivity
