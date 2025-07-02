# ClaudeTetris アーキテクチャ設計書（2025-07-02更新・Rustバックエンド優先）

## 1. 全体アーキテクチャ概要
ClaudeTetrisは、**最高のパフォーマンス**と**開発効率**を両立したモダンなWebゲームアプリケーションである。フロントエンドはNext.js + React、バックエンドはRust、ゲームエンジンはRust + WebAssemblyを採用し、型安全性・メモリ安全性・並行処理の安全性を重視する。

### **アーキテクチャの特徴**
- **フロントエンド**: Next.js 15 (App Router) + React 19 + TypeScript
- **バックエンド**: Rust (Axum/Actix-web) + PostgreSQL + Redis
- **ゲームエンジン**: Rust + WebAssembly
- **認証**: Clerk + JWT
- **リアルタイム通信**: WebSocket (tokio-tungstenite)

---

## 2. 技術スタック・主要構成要素

### **フロントエンド**
- **フレームワーク**: Next.js 15 (App Router) + React 19 + TypeScript
- **UI**: Material-UI (MUI) + Tailwind CSS
- **状態管理**: Redux Toolkit + RTK Query
- **ゲームエンジン**: Rust + WebAssembly
- **テスト**: Jest + React Testing Library

### **バックエンド**
- **Webフレームワーク**: Axum/Actix-web/Poem
- **ORM**: Diesel/SeaORM/sqlx
- **データベース**: PostgreSQL (Docker) + Redis
- **認証**: Rust JWTクレート + Clerk連携
- **リアルタイム通信**: WebSocket（tokio-tungstenite）
- **テスト**: Rust標準テスト + integration tests

### **インフラ・デプロイ**
- **フロントエンド**: Vercel
- **バックエンド**: AWS ECS/Fargate等
- **データベース**: AWS RDS
- **キャッシュ**: Upstash Redis
- **CI/CD**: GitHub Actions

---

## 3. ディレクトリ構成方針

### **フロントエンド（Next.js）**
```
modern-tetris/
├── src/
│   ├── app/                 # Next.js App Router
│   │   ├── components/          # 共通コンポーネント
│   │   ├── features/            # 機能別モジュール
│   │   │   ├── game/           # ゲーム機能
│   │   │   ├── achievement/    # 実績システム
│   │   │   ├── ranking/        # ランキング機能
│   │   │   └── user/           # ユーザー機能
│   │   ├── store/              # Redux Store
│   │   ├── types/              # 型定義
│   │   └── wasm/               # WASM統合
│   ├── public/
│   │   └── wasm/               # WASMファイル
│   └── rust-game-engine/       # Rustゲームエンジン
```

### **バックエンド（Rust）**
```
rust-backend/
├── src/
│   ├── main.rs              # エントリーポイント
│   ├── api/                 # APIルート
│   │   ├── mod.rs
│   │   ├── users.rs
│   │   ├── scores.rs
│   │   ├── achievements.rs
│   │   └── auth.rs
│   ├── models/              # データモデル
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── score.rs
│   │   └── achievement.rs
│   ├── services/            # ビジネスロジック
│   │   ├── mod.rs
│   │   ├── auth_service.rs
│   │   └── game_service.rs
│   ├── db/                  # データベース
│   │   ├── mod.rs
│   │   └── connection.rs
│   └── config/              # 設定
│       ├── mod.rs
│       └── app_config.rs
├── migrations/              # データベースマイグレーション
├── Cargo.toml
└── .env
```

---

## 4. API設計方針

### **RESTful API設計**
- **RustバックエンドでRESTful APIを実装**
- **OpenAPI/Swagger等によるAPIドキュメント自動生成**
- **認証・認可・バリデーション・エラーハンドリング・バージョニング・セキュリティ対策を重視**

### **リアルタイム通信**
- **WebSocket（tokio-tungstenite）によるリアルタイム通信**
- **オンラインマルチプレイヤー・チャット機能**
- **マッチングシステム・ルーム管理**

### **型安全性**
- **フロントエンド・バックエンド間の型共有**
- **OpenAPI Generatorによる型定義自動生成**
- **コンパイル時エラー検出**

---

## 5. 認証・認可設計方針

### **認証システム**
- **Clerk認証サービスを採用**
- **JWTトークンによるセッション管理**
- **SNSログイン（Google, Twitter, GitHub等）対応**

### **認可・権限管理**
- **ユーザー権限の段階的管理**
- **管理者/一般ユーザー等の権限分離**
- **APIアクセス制御**

---

## 6. データベース設計方針

### **メインデータベース**
- **PostgreSQL（AWS RDS）をメインDBとして採用**
- **Diesel/SeaORMによる型安全なデータベースアクセス**
- **マイグレーション・スキーマ設計・セキュリティ・バックアップを重視**

### **キャッシュ・セッション**
- **Redis（Upstash）によるキャッシュ・セッション管理**
- **ゲーム状態・ユーザーセッションの高速アクセス**
- **リアルタイムデータの一時保存**

---

## 7. テスト・CI/CD戦略

### **フロントエンドテスト**
- **Jest + React Testing Libraryでユニット/統合テスト**
- **Playwright/CypressでE2Eテスト**
- **WASM統合テスト**

### **バックエンドテスト**
- **Rust標準テスト + integration tests**
- **データベース統合テスト**
- **API エンドポイントテスト**

### **CI/CD自動化**
- **Lint/型チェック/テストのCI自動化**
- **GitHub Actionsによる自動デプロイ**
- **Docker化による環境統一**

---

## 8. パフォーマンス目標

### **フロントエンド目標**
- **FPS**: 60 FPS（WASM統合後）
- **メモリ使用量**: 30-50MB（WASM統合後）
- **初期化時間**: 1-2秒（WASM統合後）
- **入力遅延**: <16ms（WASM統合後）

### **バックエンド目標**
- **API応答時間**: <50ms
- **同時接続数**: 1000+（WebSocket）
- **メモリ使用量**: <100MB
- **スループット**: 10000+ req/sec

---

## 9. 今後の拡張・未決事項

### **短期目標**
- **Rustバックエンド基盤構築**
- **基本的なAPIエンドポイント実装**
- **フロントエンド・バックエンド統合**

### **中期目標**
- **オンラインマルチプレイヤー機能**
- **リアルタイム通信実装**
- **マッチングシステム**

### **長期目標**
- **PWA/モバイル対応**
- **国際化（i18n）**
- **アクセシビリティ対応**
- **高度なゲーム機能（リプレイ・トーナメント）**

---

## 10. 参考：ADR一覧
- `doc/adr/adr001_spa_framework.md` ～ `adr023_database_integration_strategy.md` に各技術選定・設計方針の詳細を記録

### **主要ADR**
- **ADR020**: 認証技術選定（Clerk + Redis）
- **ADR021**: リアルタイム通信技術選定（Socket.io）
- **ADR022**: 状態管理技術選定（Redux Toolkit + RTK Query）
- **ADR023**: データベース統合戦略（Prisma + PostgreSQL + AWS RDS）

---

## 11. 移行計画

### **Phase 4: Rustバックエンド構築（進行中）**
- ✅ **Phase 4B**: データベース統合（完了済み）
- 🔄 **Phase 4C**: Rustバックエンド基盤構築（現在進行中）
- ⏳ **Phase 4D**: 認証システム実装（予定）

### **Phase 5: フロントエンド・バックエンド統合（予定）**
- ⏳ **API通信実装**
- ⏳ **RTK Query統合**
- ⏳ **型安全性確保**

### **Phase 6: リアルタイム機能（予定）**
- ⏳ **WebSocket/Socket.io実装**
- ⏳ **オンラインマルチプレイヤー**
- ⏳ **マッチングシステム**

---

本設計書は現時点の合意内容を反映しており、今後の要件・技術動向に応じて随時アップデートする。 