# 🔄 ハイブリッド構成移行計画

## 📋 **作業全体の流れ**

### **Phase 1: 基盤構築（完了済み）**
- ✅ **rust-game-engineプロジェクト作成**
- ✅ **WebAssembly対応設定**
- ✅ **基本的なゲームエンジン構造**
- ✅ **実績システム基盤**

### **Phase 2: 既存コード移行（進行中）**
- 🔄 **TypeScript → Rust 移行**
- ⏳ **WebAssembly統合**
- ⏳ **Redux統合修正**

### **Phase 3: 統合・テスト（予定）**
- ⏳ **Next.js統合**
- ⏳ **パフォーマンス最適化**
- ⏳ **エラーハンドリング**

### **Phase 4: デプロイ・運用（予定）**
- ⏳ **GitHub Pagesデプロイ**
- ⏳ **継続的改善**

---

## 🎯 **詳細移行計画**

### **Phase 2A: ゲームロジック移行**

#### **Step 1: テトロミノシステム**
```typescript
// 移行元: modern-tetris/src/features/game/utils/tetromino.ts
// 移行先: rust-game-engine/src/game/tetromino.rs

// 実装内容:
// - テトロミノ形状定義
// - 回転ロジック
// - 色情報管理
```

#### **Step 2: 衝突検出システム**
```typescript
// 移行元: modern-tetris/src/features/game/utils/collision.ts
// 移行先: rust-game-engine/src/game/collision.rs

// 実装内容:
// - 境界チェック
// - ブロック衝突判定
// - 重複検出
```

#### **Step 3: SRSシステム**
```typescript
// 移行元: modern-tetris/src/features/game/utils/srs.ts
// 移行先: rust-game-engine/src/game/srs.rs

// 実装内容:
// - キックテーブル
// - 回転判定
// - 壁キック処理
```

#### **Step 4: スコアリングシステム**
```typescript
// 移行元: modern-tetris/src/features/game/utils/pointsSystem.ts
// 移行先: rust-game-engine/src/game/scoring.rs

// 実装内容:
// - ポイント計算
// - コンボシステム
// - レベル計算
```

#### **Step 5: T-Spin検出**
```typescript
// 移行元: modern-tetris/src/features/game/utils/spinDetection.ts
// 移行先: rust-game-engine/src/game/spin_detection.rs

// 実装内容:
// - T-Spin判定
// - Mini T-Spin判定
// - スピン条件チェック
```

### **Phase 2B: 実績システム移行**

#### **Step 1: 実績データ移行**
```typescript
// 移行元: modern-tetris/src/store/slices/fullAchievements.ts
// 移行先: rust-game-engine/src/achievement/data.rs

// 実装内容:
// - 255個の実績定義
// - カテゴリ分類
// - 条件設定
```

#### **Step 2: 実績管理ロジック**
```typescript
// 移行元: modern-tetris/src/store/slices/achievementSlice.ts
// 移行先: rust-game-engine/src/achievement/manager.rs

// 実装内容:
// - 実績判定ロジック
// - 進捗追跡
// - 解除処理
```

### **Phase 2C: WebAssembly統合**

#### **Step 1: APIラッパー作成**
```typescript
// 新規作成: modern-tetris/src/wasm/game_engine.ts
// 新規作成: modern-tetris/src/wasm/achievement_manager.ts

// 実装内容:
// - Rust関数のJavaScriptラッパー
// - 型定義
// - エラーハンドリング
```

#### **Step 2: Redux統合修正**
```typescript
// 修正: modern-tetris/src/store/slices/gameSlice.ts
// 修正: modern-tetris/src/store/slices/achievementSlice.ts

// 修正内容:
// - Rust API呼び出し
// - 状態同期
// - エラー処理
```

#### **Step 3: Next.js設定修正**
```typescript
// 修正: modern-tetris/next.config.ts

// 修正内容:
// - WebAssembly対応
// - ビルド設定
// - アセット最適化
```

---

## 📊 **移行進捗管理**

### **完了済み（✅）**
- [x] rust-game-engineプロジェクト作成
- [x] 基本的なゲームエンジン構造
- [x] WebAssembly対応設定
- [x] 実績システム基盤
- [x] ビルドスクリプト作成

### **進行中（🔄）**
- [ ] テトロミノシステム移行
- [ ] 衝突検出システム移行
- [ ] SRSシステム移行
- [ ] スコアリングシステム移行

### **未着手（⏳）**
- [ ] T-Spin検出移行
- [ ] 実績データ移行
- [ ] 実績管理ロジック移行
- [ ] WebAssembly APIラッパー作成
- [ ] Redux統合修正
- [ ] Next.js設定修正
- [ ] 統合テスト
- [ ] パフォーマンス最適化
- [ ] エラーハンドリング実装

---

## 🛠️ **開発環境セットアップ**

### **Rust環境**
```bash
# Rust インストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# wasm-pack インストール
cargo install wasm-pack

# プロジェクトディレクトリ移動
cd rust-game-engine

# 依存関係インストール
cargo build
```

### **Next.js環境**
```bash
# プロジェクトディレクトリ移動
cd modern-tetris

# 依存関係インストール
npm install

# 開発サーバー起動
npm run dev
```

### **統合テスト**
```bash
# Rust テスト
cd rust-game-engine
cargo test

# WebAssembly テスト
wasm-pack test --headless --firefox

# Next.js テスト
cd ../modern-tetris
npm test
```

---

## 🚀 **パフォーマンス目標**

### **移行前（TypeScript）**
- **FPS**: 30-45 FPS
- **メモリ使用量**: 100-150MB
- **初期化時間**: 3-5秒
- **入力遅延**: 50-100ms

### **移行後（Rust + WebAssembly）**
- **FPS**: 60 FPS（安定）
- **メモリ使用量**: 30-50MB
- **初期化時間**: 1-2秒
- **入力遅延**: <16ms

### **改善率**
- **パフォーマンス**: 2-3倍向上
- **メモリ効率**: 60-70%削減
- **初期化速度**: 2-3倍高速化
- **応答性**: 3-6倍改善

---

## 🔧 **品質保証**

### **テスト戦略**
1. **単体テスト**: 各Rustモジュールの個別テスト
2. **統合テスト**: WebAssemblyとNext.jsの統合テスト
3. **パフォーマンステスト**: ベンチマーク測定
4. **E2Eテスト**: ブラウザでの動作確認

### **コード品質**
1. **型安全性**: 100% TypeScript + Rust型チェック
2. **エラーハンドリング**: 包括的なエラー処理
3. **ドキュメント**: 詳細なAPI仕様書
4. **コメント**: 複雑なロジックの説明

---

## 📈 **リスク管理**

### **技術的リスク**
- **WebAssembly対応**: ブラウザ互換性の問題
- **パフォーマンス**: 期待通りの改善が得られない可能性
- **統合**: RustとTypeScriptの連携問題

### **対策**
- **段階的移行**: 機能ごとに順次移行
- **テスト駆動**: 各段階での動作確認
- **フォールバック**: 問題発生時の代替案準備

---

## 🎯 **成功指標**

### **技術指標**
- [ ] 60FPSでの安定動作
- [ ] メモリ使用量50MB以下
- [ ] 初期化時間2秒以下
- [ ] 入力遅延16ms以下

### **品質指標**
- [ ] テストカバレッジ80%以上
- [ ] 型安全性100%
- [ ] エラー率0.1%以下
- [ ] ユーザビリティ向上

### **開発効率指標**
- [ ] ビルド時間30秒以下
- [ ] ホットリロード1秒以下
- [ ] デバッグ時間50%削減
- [ ] コード保守性向上

この移行計画により、**最高のパフォーマンス**と**開発効率**を両立したモダンなテトリスゲームが完成します。

# モダンテトリス Rust移行ロードマップ（Phase3以降・2024-06-27時点）

---

## Phase3: SRS・スピン判定・スコア計算のRust実装

### 目標
- SRS（Super Rotation System）回転・キックテーブル・壁蹴り処理をRust側に移植
- T-Spin/SZ-Spin/I-Spin/JL-Spin等のスピン判定ロジックをRust側に実装
- スコア計算・ライン消去・Back-to-Back/Combo等の判定もRust側に統合

### 主な作業
- `modern-tetris/src/features/game/utils/srs.ts`・`spinDetection.ts`のRust移植
- RustエンジンAPI（rotate_tetromino, move_tetromino等）でSRS・スピン判定を返却
- スコア・ライン消去・B2B/Combo管理のRust実装
- WASM/JS型変換・Redux/React連携の拡張

### 成果物
- RustエンジンでSRS回転・スピン判定・スコア計算が完結
- UI/ReduxはRustエンジンの結果を描画・管理するのみ

---

## Phase3 SRS・スピン判定 差分調査・実装方針（2024-06-27追記）

### 公式SRS仕様と現実装の主な差分
- 回転中心・キックテーブルが公式と異なる場合あり（特にI/Oミノ）
- テトリミノ形状・回転ごとのブロック配置が公式とズレている場合あり
- SRS回転・壁蹴り処理が一部簡略化・未対応
- T-Spin判定の中心・角・wallkick定義が公式と異なる場合あり
- テスト網羅性が不十分（全ミノ種・全回転パターン・壁際/床際/ブロック隣接時の挙動）

### 正しい実装手順（TTCガイドライン準拠）
1. 公式SRSテーブル・回転中心の定義（全ミノ種・全回転パターン）
2. テトリミノ形状・回転ごとのブロック配置テーブルの定義
3. SRS回転処理の公式準拠実装（kick offset順試行・wallkick情報返却）
4. fix_tetrominoで4マスすべてをフィールドに埋め込む
5. T-Spin判定ロジックの公式準拠化（中心・角・wallkick・Mini分岐）
6. 全ミノ種・全回転パターンのテストケース追加

### 今後の方針
- 公式SRS/T-Spin仕様を厳密に再現する設計・実装・テストを推進
- JS/Rustともに公式仕様との差分を意識し、段階的に修正・移植
- 参考: [Tetris Guideline SRS](https://tetris.wiki/SRS), [T-Spin判定仕様](https://tetris.wiki/T-Spin)

---

## Phase4: ゲーム進行・リプレイ・実績・永続化

### 目標
- ゲーム進行（自動落下・新規テトリミノ生成・ゲームオーバー判定等）をRust側で一元管理
- リプレイ保存・再生、実績判定、スコア/実績/統計の永続化
- 高速化・バグ修正・テスト自動化

### 主な作業
- Rustエンジンにゲーム進行管理API（tick, step, reset, is_game_over等）を追加
- リプレイデータのシリアライズ/デシリアライズ設計
- 実績判定・統計集計のRust実装
- WASM/JS間のデータ永続化・ロード/セーブAPI設計
- E2Eテスト・パフォーマンス最適化

### 成果物
- ゲーム進行・リプレイ・実績・統計がRustエンジンで一元管理
- UI/Reduxは状態表示・操作入力のみ

---

## Phase5: UI/UX最適化・マルチプラットフォーム展開

### 目標
- UI/UXの最適化・アクセシビリティ向上・レスポンシブ対応
- WASMエンジンのWeb以外（デスクトップ/モバイル/ネイティブ）展開基盤整備
- コミュニティ・拡張性・OSS化推進

### 主な作業
- UI/UX改善・アニメーション・アクセシビリティ対応
- WASMバイナリの最適化・キャッシュ・バージョン管理
- Electron/React Native等への展開検証
- ドキュメント・開発ガイド・コントリビュート体制整備

### 成果物
- 高品質なUI/UX・多様なプラットフォーム対応
- OSS/コミュニティ展開可能なモダンテトリス基盤

---

## 補足
- 各Phaseは段階的にPR・ドキュメント・テストを整備しつつ進行
- 既存JS実装の知見を活かしつつ、Rust側に段階的にロジックを移管
- 進捗・成果は`doc/files/phaseX.md`・`log/rust-migration/`等に随時記録 