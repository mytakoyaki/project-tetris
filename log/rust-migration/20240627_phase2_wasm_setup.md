# Rust移行 Phase2 WASMセットアップログ（2024-06-27）

## 概要
- Rustゲームエンジン（rust-game-engine）をWebAssembly（WASM）ビルド対応にセットアップ。
- wasm-bindgen/wasm-pack依存追加、lib.rsにWASMエクスポートAPI雛形を実装、READMEにNext.js連携手順を追記。

---

## 変更内容
- Cargo.toml: wasm-bindgen, serde, serde_json依存追加、crate-typeをcdylibに設定、リリースビルド最適化
- src/lib.rs: wasm_bindgenセットアップ、Field/Tetromino構造体とinit_field/spawn_tetromino等のAPI雛形を追加
- README.md: WASMビルド・Next.js連携手順・API例を追記

---

## 次の作業予定
- wasm-packでWASMビルドを実行し、pkg/配下の生成物を確認
- Next.js側でWASMバイナリの読み込みテスト
- Rust⇔JS間のデータ構造・API設計の詳細化 