# Rust移行 Phase2 開始ログ（2024-06-27）

## 目的
- Rust製ゲームエンジン（WASM）とNext.js（React）フロントエンドのハイブリッド統合を開始する。
- コアなゲームロジックをRustに移管し、パフォーマンス・保守性・拡張性を向上させる。

## 初期方針
- Rustエンジンを`wasm-pack`でWASMビルド対応にする
- JS⇔Rust間のAPI設計・データ構造設計を進める
- Next.js側でWASMを組み込み、テトリミノ操作などの最小APIを動作させる

## 直近の作業内容
- `doc/files/phase2.md`にPhase2のファイル仕様・設計方針をまとめる
- `rust-game-engine/`にwasm-bindgen/wasm-pack依存を追加予定
- `lib.rs`にWASMエクスポート用関数の雛形を追加予定
- Next.js側でWASMバイナリの組み込み方法を検討

---

以降、各作業・設計・実装の詳細はこの`log/rust-migration/`配下に随時記録する。 