# Rust Tetris Game Engine

WebAssembly対応のRust製テトリスゲームエンジンです。

## 🚀 機能

- **高性能ゲームロジック**: Rustによる高速なゲーム計算
- **WebAssembly対応**: ブラウザでの高速実行
- **実績システム**: 255個の実績を管理
- **型安全性**: コンパイル時のエラー検出
- **クロスプラットフォーム**: 単一コードベースで複数プラットフォーム対応

## 🛠️ 技術スタック

- **言語**: Rust 2021
- **WebAssembly**: wasm-bindgen
- **シリアライゼーション**: serde
- **乱数生成**: rand + getrandom

## 📦 セットアップ

### 前提条件

```bash
# Rust と Cargo のインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# wasm-pack のインストール
cargo install wasm-pack
```

### ビルド

```bash
# 開発ビルド
cargo build

# WebAssembly ビルド
wasm-pack build --target web

# リリースビルド
wasm-pack build --target web --release
```

### テスト

```bash
# 単体テスト
cargo test

# WebAssembly テスト
wasm-pack test --headless --firefox
```

## 📁 プロジェクト構造

```
src/
├── lib.rs                    # WebAssembly エクスポート
├── game/                     # ゲームロジック
│   ├── mod.rs
│   ├── engine.rs             # ゲームエンジン
│   ├── tetromino.rs          # テトロミノ管理
│   ├── field.rs              # ゲームフィールド
│   ├── collision.rs          # 衝突検出
│   ├── scoring.rs            # スコアリング
│   ├── srs.rs                # SRS（Super Rotation System）
│   └── spin_detection.rs     # T-Spin検出
├── achievement/              # 実績システム
│   ├── mod.rs
│   ├── manager.rs            # 実績管理
│   ├── data.rs               # 実績データ
│   └── types.rs              # 型定義
└── utils/                    # ユーティリティ
    ├── mod.rs
    ├── math.rs               # 数学関数
    ├── time.rs               # 時間管理
    └── config.rs             # 設定管理
```

## 🔧 使用方法

### JavaScript/TypeScript からの呼び出し

```typescript
import init, { GameEngine, AchievementManager } from './pkg/rust_game_engine';

// WebAssembly 初期化
await init();

// ゲームエンジン作成
const gameEngine = new GameEngine();

// ゲーム状態取得
const state = gameEngine.get_state();

// テトロミノ移動
gameEngine.move_tetromino("left");

// 実績管理
const achievementManager = new AchievementManager();
const achievements = achievementManager.get_achievements();
```

## 🎮 ゲーム機能

### テトロミノ操作
- **移動**: 左右下移動
- **回転**: 時計回り・反時計回り
- **ハードドロップ**: 即座に落下
- **ソフトドロップ**: 高速落下

### ゲームシステム
- **SRS**: Super Rotation System
- **T-Spin検出**: T-Spin判定
- **ライン消去**: 重力効果付き
- **スコアリング**: 複雑なポイント計算
- **レベルシステム**: 段階的難易度上昇

### 実績システム
- **255個の実績**: 多様なカテゴリ
- **進捗追跡**: 実績達成度の可視化
- **ポイント報酬**: 実績解除によるポイント獲得

## 🚀 パフォーマンス

- **60FPS**: 滑らかなゲームプレイ
- **低レイテンシ**: 即座の入力応答
- **メモリ効率**: 最小限のメモリ使用量
- **CPU最適化**: 効率的なアルゴリズム

## 🔗 統合

### Next.js との統合

```typescript
// next.config.ts
const nextConfig = {
  webpack: (config) => {
    config.experiments = {
      ...config.experiments,
      asyncWebAssembly: true,
    };
    return config;
  },
};
```

### Redux との統合

```typescript
// gameSlice.ts
import { GameEngine } from '@/wasm/game_engine';

export const gameSlice = createSlice({
  name: 'game',
  initialState,
  reducers: {
    moveTetromino: (state, action) => {
      const result = gameEngine.move_tetromino(action.payload);
      // 状態更新
    },
  },
});
```

## 📝 ライセンス

MIT License

## 🤝 貢献

プルリクエストやイシューの報告を歓迎します。

## Phase2: WASMビルド & Next.js連携

### 1. ビルド要件
- Rust 1.70以上
- wasm-pack
- wasm-bindgen

### 2. WASMビルド手順
```sh
cd rust-game-engine
wasm-pack build --target web --release
```
- `pkg/`配下にWASMバイナリとJSラッパーが生成されます

### 3. Next.jsへの組み込み例
- `modern-tetris/public/wasm/`に`pkg/`の中身をコピー
- Next.js側で`import init, { init_field, spawn_tetromino } from '../public/wasm/rust_game_engine';`
- 初回のみ`await init()`で初期化

### 4. 主要API（例）
- `init_field(width, height)`
- `spawn_tetromino()`
- `move_tetromino(dir)`
- `rotate_tetromino(dir)`
- `hard_drop()`
- `get_score()`
- `get_rank()`
- `get_achievements()`

### 5. 開発メモ
- API/データ構造は今後段階的に拡張
- 詳細設計は`doc/files/phase2.md`参照 