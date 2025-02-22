#!/bin/sh

if [[ $1 == "if_not_exists" ]]; then
  if [ -f ../android/app/src/main/jniLibs/android/arm64-v8a/libshared.so ] &&
     [ -f ../android/app/src/main/jniLibs/android/x86_64/libshared.so ]; then
    echo "libshared.so already exists, skipping Rust compilation"
    exit 0
  fi
fi

export PATH="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH"
export ANDROID_NDK=ANDROID_NDK_HOME
export ANDROID_NDK_ROOT=ANDROID_NDK_HOME

rm -rf target/uniffi
mkdir -p target/uniffi

cargo build --target aarch64-linux-android \
  --config "target.aarch64-linux-android.linker=\"${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android30-clang\""

cargo run --bin uniffi-bindgen generate --library target/aarch64-linux-android/debug/libshared.so --language kotlin --out-dir target/uniffi

cargo build --target x86_64-linux-android \
  --config "target.x86_64-linux-android.rustflags=\"-L${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/lib/clang/18/lib/linux/ -lstatic=clang_rt.builtins-x86_64-android -llog\"" \
  --config "target.x86_64-linux-android.linker=\"${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android30-clang\""

rm -rf ../android/app/src/main/java/com/lingolessons/shared/
mkdir -p ../android/app/src/main/java/com/lingolessons/shared/
cp -r target/uniffi/com/lingolessons/shared/* ../android/app/src/main/java/com/lingolessons/shared/

rm -rf ../android/app/src/main/jniLibs/android/
mkdir -p ../android/app/src/main/jniLibs/android/arm64-v8a
cp target/aarch64-linux-android/debug/libshared.so ../android/app/src/main/jniLibs/android/arm64-v8a/
mkdir -p ../android/app/src/main/jniLibs/android/x86_64
cp target/x86_64-linux-android/debug/libshared.so ../android/app/src/main/jniLibs/android/x86_64/
