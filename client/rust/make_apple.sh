#!/bin/zsh

set -e

rm -rf target/headers && mkdir -p target/headers
rm -rf shared.xcframework

#ios-arm64
cargo build --target aarch64-apple-ios
cargo run --bin uniffi-bindgen generate --library target/aarch64-apple-ios/debug/libshared.a --language swift --out-dir target/headers

#ios-arm64_x86_65-simulator
#cargo build --target aarch64-apple-ios-sim
#cargo build --target x86_64-apple-ios
#lipo -create target/aarch64-apple-ios-sim/debug/libshared.a target/x86_64-apple-ios/debug/libshared.a -output target/aarch64-apple-ios-sim/debug/universal.a

#macos-arm64_x86_65
#cargo build --target aarch64-apple-darwin
#cargo build --target x86_64-apple-darwin
#lipo -create target/aarch64-apple-darwin/debug/libshared.a target/x86_64-apple-darwin/debug/libshared.a -output target/aarch64-apple-darwin/debug/universal.a

rm -f target/headers/module.modulemap
mv target/headers/sharedFFI.modulemap target/headers/module.modulemap
mv -f target/headers/shared.swift ../apple/App/App/shared.swift

xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/debug/libshared.a -headers target/headers \
  -library target/aarch64-apple-ios-sim/debug/universal.a -headers target/headers \
  -library target/aarch64-apple-darwin/debug/universal.a -headers target/headers \
  -output shared.xcframework
