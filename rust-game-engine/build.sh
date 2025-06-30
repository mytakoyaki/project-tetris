#!/bin/bash

# Rust Tetris Game Engine WebAssembly ビルドスクリプト

echo "🚀 Building Rust Tetris Game Engine for WebAssembly..."

# 開発ビルド
echo "📦 Building development version..."
cargo build

if [ $? -eq 0 ]; then
    echo "✅ Development build successful"
else
    echo "❌ Development build failed"
    exit 1
fi

# WebAssembly ビルド
echo "🌐 Building WebAssembly version..."
wasm-pack build --target web

if [ $? -eq 0 ]; then
    echo "✅ WebAssembly build successful"
else
    echo "❌ WebAssembly build failed"
    exit 1
fi

# リリースビルド
echo "🎯 Building release version..."
wasm-pack build --target web --release

if [ $? -eq 0 ]; then
    echo "✅ Release build successful"
else
    echo "❌ Release build failed"
    exit 1
fi

echo "🎉 All builds completed successfully!"
echo "📁 Output files:"
echo "   - Development: target/debug/"
echo "   - WebAssembly: pkg/"
echo "   - Release: pkg/ (optimized)" 