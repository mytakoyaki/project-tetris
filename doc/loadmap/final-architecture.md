# 🏗️ 最終的なプロジェクトアーキテクチャ

## 📁 完全なディレクトリ構造

```
project-tetris/
├── .git/                        # Gitリポジトリ
├── .github/                     # GitHub設定
│   └── workflows/               # GitHub Actions
│       └── deploy.yml           # 自動デプロイメント
│
├── modern-tetris/               # Next.jsフロントエンド（UI）
│   ├── .next/                   # Next.jsビルド出力
│   ├── .swc/                    # SWCキャッシュ
│   ├── node_modules/            # npm依存関係
│   ├── out/                     # 静的エクスポート
│   ├── public/                  # 静的アセット
│   │   ├── assets/
│   │   │   └── videos/          # 動画ファイル
│   │   └── ...
│   ├── src/
│   │   ├── app/                 # Next.js App Router
│   │   │   ├── layout.tsx       # ルートレイアウト
│   │   │   ├── page.tsx         # ホームページ（ゲーム画面）
│   │   │   ├── achievements/    # 実績ページ
│   │   │   │   └── page.tsx
│   │   │   ├── providers.tsx    # Redux + WebAssembly初期化
│   │   │   ├── theme-registry.tsx # Material-UIテーマ
│   │   │   └── globals.css      # グローバルスタイル
│   │   ├── components/          # 共通コンポーネント
│   │   │   └── no-ssr.tsx       # SSR無効化コンポーネント
│   │   ├── features/            # 機能別モジュール
│   │   │   ├── game/            # ゲーム機能
│   │   │   │   ├── components/  # ゲームUIコンポーネント
│   │   │   │   │   ├── GameContainer.tsx
│   │   │   │   │   ├── GameField.tsx
│   │   │   │   │   ├── Sidebar.tsx
│   │   │   │   │   ├── GameControls.tsx
│   │   │   │   │   ├── ExchangeControls.tsx
│   │   │   │   │   ├── PointsDisplay.tsx
│   │   │   │   │   ├── RankDisplay.tsx
│   │   │   │   │   ├── FeverModeDisplay.tsx
│   │   │   │   │   ├── ProgressGauge.tsx
│   │   │   │   │   ├── GameMenu.tsx
│   │   │   │   │   └── effects/ # 視覚効果
│   │   │   │   │       └── LineClearEffect.tsx
│   │   │   │   ├── hooks/       # ゲームロジックフック
│   │   │   │   │   ├── useGameEngine.ts    # Rust API呼び出し
│   │   │   │   │   ├── useKeyboardInput.ts # キーボード入力
│   │   │   │   │   └── useRankUpdate.ts    # ランク更新
│   │   │   │   └── utils/       # 軽量ユーティリティ
│   │   │   │       └── __tests__/ # テスト
│   │   │   ├── achievement/     # 実績システム
│   │   │   │   ├── components/  # 実績UI
│   │   │   │   │   ├── AchievementDisplay.tsx
│   │   │   │   │   ├── AchievementNotification.tsx
│   │   │   │   │   └── AchievementSummary.tsx
│   │   │   │   └── hooks/       # 実績管理フック
│   │   │   │       └── useAchievementPersistence.ts
│   │   │   ├── ranking/         # ランキング機能
│   │   │   ├── score/           # スコア管理
│   │   │   └── user/            # ユーザー管理
│   │   ├── store/               # Redux状態管理
│   │   │   ├── store.ts         # Reduxストア設定
│   │   │   └── slices/          # Reduxスライス
│   │   │       ├── gameSlice.ts        # Rust API呼び出し
│   │   │       ├── achievementSlice.ts # Rust API呼び出し
│   │   │       ├── scoreSlice.ts       # スコア状態
│   │   │       ├── userSlice.ts        # ユーザー状態
│   │   │       └── fullAchievements.ts # 実績データ（参照用）
│   │   ├── wasm/                # WebAssembly統合
│   │   │   ├── game_engine.ts   # RustゲームエンジンAPI
│   │   │   ├── achievement_manager.ts # Rust実績システムAPI
│   │   │   └── types.ts         # WebAssembly型定義
│   │   ├── types/               # TypeScript型定義
│   │   │   ├── game.ts          # ゲーム関連型
│   │   │   ├── points.ts        # ポイントシステム型
│   │   │   ├── rank.ts          # ランクシステム型
│   │   │   ├── spin.ts          # T-Spin関連型
│   │   │   └── api.ts           # API関連型
│   │   ├── styles/              # スタイル
│   │   └── shared/              # 共有リソース
│   ├── package.json             # npm設定
│   ├── package-lock.json        # 依存関係ロック
│   ├── next.config.ts           # Next.js設定（WebAssembly対応）
│   ├── tsconfig.json            # TypeScript設定
│   ├── eslint.config.mjs        # ESLint設定
│   ├── jest.config.js           # テスト設定
│   ├── postcss.config.mjs       # PostCSS設定
│   ├── tailwind.config.js       # Tailwind CSS設定
│   ├── README.md                # プロジェクト説明
│   ├── DEPLOYMENT.md            # デプロイメント手順
│   └── IMPLEMENTATION_PLAN.md   # 実装計画
│
├── rust-game-engine/            # Rustバックエンド（ゲームロジック）
│   ├── .git/                    # 独立Gitリポジトリ
│   ├── target/                  # Rustビルド出力
│   ├── pkg/                     # WebAssembly出力
│   │   ├── rust_game_engine.js  # JavaScriptラッパー
│   │   ├── rust_game_engine_bg.wasm # WebAssemblyバイナリ
│   │   ├── rust_game_engine.d.ts # TypeScript型定義
│   │   └── package.json         # npmパッケージ情報
│   ├── src/
│   │   ├── lib.rs               # WebAssemblyエクスポート
│   │   ├── main.rs              # エントリーポイント（開発用）
│   │   ├── game/                # ゲームロジック
│   │   │   ├── mod.rs           # ゲームモジュール
│   │   │   ├── engine.rs        # ゲームエンジン
│   │   │   ├── tetromino.rs     # テトロミノ管理
│   │   │   ├── field.rs         # ゲームフィールド
│   │   │   ├── collision.rs     # 衝突検出
│   │   │   ├── scoring.rs       # スコアリングシステム
│   │   │   ├── srs.rs           # SRS（Super Rotation System）
│   │   │   └── spin_detection.rs # T-Spin検出
│   │   ├── achievement/         # 実績システム
│   │   │   ├── mod.rs           # 実績モジュール
│   │   │   ├── manager.rs       # 実績管理
│   │   │   ├── data.rs          # 実績データ（255個）
│   │   │   └── types.rs         # 実績型定義
│   │   ├── ui/                  # UI関連（将来のデスクトップアプリ用）
│   │   │   ├── mod.rs
│   │   │   ├── renderer.rs      # レンダリング
│   │   │   ├── input.rs         # 入力処理
│   │   │   └── effects.rs       # 視覚効果
│   │   ├── audio/               # 音声システム（将来実装）
│   │   │   ├── mod.rs
│   │   │   ├── manager.rs       # 音声管理
│   │   │   └── sounds.rs        # 音声データ
│   │   └── utils/               # ユーティリティ
│   │       ├── mod.rs           # ユーティリティモジュール
│   │       ├── math.rs          # 数学関数
│   │       ├── time.rs          # 時間管理
│   │       └── config.rs        # 設定管理
│   ├── assets/                  # アセット（将来実装）
│   │   ├── sounds/              # 音声ファイル
│   │   ├── fonts/               # フォントファイル
│   │   └── textures/            # テクスチャファイル
│   ├── tests/                   # 統合テスト
│   ├── examples/                # 使用例
│   ├── docs/                    # ドキュメント
│   ├── Cargo.toml               # Rustプロジェクト設定
│   ├── Cargo.lock               # 依存関係ロック
│   ├── .gitignore               # Git除外設定
│   ├── README.md                # Rustプロジェクト説明
│   └── build.sh                 # ビルドスクリプト
│
├── tetris/                      # 旧版テトリス（HTML/JavaScript）
│   ├── index.html               # メインHTML
│   ├── script.js                # ゲームロジック
│   ├── style.css                # スタイル
│   ├── assets/                  # アセット
│   └── tests/                   # テスト
│
├── doc/                         # プロジェクトドキュメント
│   ├── adr/                     # アーキテクチャ決定記録
│   │   ├── adr001_spa_framework.md
│   │   ├── adr002_state_management.md
│   │   ├── adr003_directory_structure.md
│   │   ├── adr004_ui_framework.md
│   │   ├── adr005_test_strategy.md
│   │   ├── adr006_routing.md
│   │   ├── adr007_api_data_strategy.md
│   │   ├── adr008_authentication_strategy.md
│   │   └── adr009_database_strategy.md
│   ├── architecture/            # システムアーキテクチャ
│   │   └── architecture.md      # アーキテクチャ概要
│   └── loadmap/                 # ロードマップ
│       ├── roadmap.md           # 開発ロードマップ
│       ├── final-architecture.md # 最終アーキテクチャ（このファイル）
│       └── migration-plan.md    # 移行計画
│
├── log/                         # 開発ログ
│   ├── 20241225_1400_react_migration_complete.md
│   ├── 20250625_1425_fever_effects_polish.md
│   ├── 20250625_1943_nextjs_foundation_setup.md
│   ├── 20250625_1959_検証まとめ.md
│   ├── 20250625_2006_react_components_implementation.md
│   ├── 20250625_2006_react_components_verification.md
│   ├── 20250625_2012_hydration_error_fix.md
│   ├── 20250625_2029_tetromino_system_implementation.md
│   ├── 20250625_2133_collision_detection_system_implementation.md
│   ├── 20250625_2200_auto_drop_timer_implementation.md
│   ├── 20250625_2230_comprehensive_implementation_plan.md
│   ├── 20250625_2235_claude_tetris_core_systems_complete.md
│   ├── 20250625_2315_point_exchange_system_implementation.md
│   ├── 20250625_2330_srs_spin_system_complete.md
│   ├── 20250626_ソフトドロップスコアリング修正.md
│   └── 20250626_自動落下開始問題の修正.md
│
├── scripts/                     # スクリプト
│   ├── deploy.sh                # デプロイメントスクリプト
│   └── convert_achievements.js  # 実績データ変換
│
├── .gitignore                   # Git除外設定
├── CLAUDE.md                    # Claude使用記録
└── README.md                    # プロジェクト概要
```

## 🔄 **データフロー**

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   React UI      │    │   Redux Store   │    │  Rust Engine    │
│                 │    │                 │    │                 │
│ - GameField     │◄──►│ - gameSlice     │◄──►│ - GameEngine    │
│ - Sidebar       │    │ - achievement   │    │ - Tetromino     │
│ - Controls      │    │   Slice         │    │ - Collision     │
│ - Effects       │    │ - scoreSlice    │    │ - Scoring       │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   WebAssembly   │    │   LocalStorage  │    │   Achievement   │
│   API Wrapper   │    │   Persistence   │    │   System        │
│                 │    │                 │    │                 │
│ - game_engine   │    │ - Game State    │    │ - 255 Achievements
│ - achievement   │    │ - User Progress │    │ - Progress      │
│   _manager      │    │ - Settings      │    │   Tracking      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## 🎯 **技術スタック**

### **フロントエンド（modern-tetris）**
- **Framework**: Next.js 14 (App Router)
- **Language**: TypeScript
- **State Management**: Redux Toolkit
- **UI Library**: Material-UI (MUI)
- **Styling**: CSS Modules + PostCSS
- **Testing**: Jest + React Testing Library
- **Build Tool**: Webpack (WebAssembly対応)
- **Package Manager**: npm

### **バックエンド（rust-game-engine）**
- **Language**: Rust 2021
- **WebAssembly**: wasm-bindgen
- **Serialization**: serde
- **Random**: rand + getrandom
- **Testing**: 標準テストフレームワーク
- **Build Tool**: Cargo + wasm-pack

### **デプロイメント**
- **Hosting**: GitHub Pages
- **CI/CD**: GitHub Actions
- **Static Export**: Next.js Export
- **Domain**: https://mytakoyaki.github.io/modern-tetris/

## 🚀 **パフォーマンス目標**

### **ゲームパフォーマンス**
- **FPS**: 60FPS（滑らかなゲームプレイ）
- **Latency**: <16ms（即座の入力応答）
- **Memory**: <50MB（軽量メモリ使用量）
- **Load Time**: <2秒（高速初期化）

### **開発効率**
- **Build Time**: <30秒（高速ビルド）
- **Hot Reload**: <1秒（即座の変更反映）
- **Type Safety**: 100%（コンパイル時エラー検出）
- **Test Coverage**: >80%（高品質保証）

## 🔧 **開発環境**

### **必須ツール**
- **Node.js**: v18+
- **npm**: v9+
- **Rust**: v1.70+
- **wasm-pack**: v0.12+
- **Git**: v2.30+

### **推奨ツール**
- **VS Code**: 推奨拡張機能
- **Rust Analyzer**: Rust開発支援
- **ESLint**: コード品質管理
- **Prettier**: コードフォーマット

## 📈 **スケーラビリティ**

### **短期目標（3ヶ月）**
- WebAssembly統合完了
- パフォーマンス最適化
- 実績システム完全移行

### **中期目標（6ヶ月）**
- デスクトップアプリ版
- モバイル対応
- マルチプレイヤー機能

### **長期目標（1年）**
- サーバーサイド実装
- クラウド同期
- 国際化対応

このアーキテクチャにより、**最高のパフォーマンス**、**開発効率**、**保守性**を実現したモダンなテトリスゲームが完成します。 