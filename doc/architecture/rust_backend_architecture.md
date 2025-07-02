# ClaudeTetris Rustバックエンド アーキテクチャ設計書

## 1. 全体アーキテクチャ概要
ClaudeTetrisは、React＋Next.jsフロントエンドとRustバックエンドの分離型アーキテクチャを採用し、高パフォーマンス・安全性・WASM連携を重視したWebゲームアプリケーションである。フロントエンドの開発効率とバックエンドの性能・安全性の両立を図る。

---

## 2. 技術スタック・主要構成要素

### フロントエンド（現状維持）
- **フレームワーク**: React（Next.js App Router）、TypeScript
- **状態管理**: Redux（Redux Toolkit）、RTK Query
- **UIフレームワーク**: Material UI（MUI）
- **ゲームロジック**: Rust（WASM）

### バックエンド（Rust移行）
- **Webフレームワーク**: Rust（Axum/Actix-web/Poem）
- **API設計**: RESTful API + WebSocket/Socket.io互換
- **認証**: JWT/OAuth2（Rustクレート）、Clerk連携
- **データベース**: PostgreSQL + Diesel/SeaORM/sqlx
- **キャッシュ**: Redis（redis-rsクレート）
- **リアルタイム通信**: rust-socketio or tokio-tungstenite

### インフラ・運用
- **データベース**: PostgreSQL（開発: Docker、本番: AWS RDS）
- **キャッシュ**: Redis（開発: Docker、本番: Upstash等）
- **認証**: Clerk + Redis（セッション管理）
- **テスト**: cargo test（Rust）、Jest + Playwright（フロント）
- **CI/CD**: GitHub Actions（Rust/JS両方）

---

## 3. ディレクトリ構成方針

### プロジェクト構造
```
project-tetris/
├── modern-tetris/          # フロントエンド（Next.js）
│   ├── src/
│   │   ├── features/       # 機能単位分割
│   │   ├── components/     # 汎用UI
│   │   ├── store/         # Redux状態管理
│   │   └── ...
│   └── ...
├── rust-backend/           # Rustバックエンド
│   ├── src/
│   │   ├── api/           # APIルート
│   │   ├── auth/          # 認証・認可
│   │   ├── models/        # データモデル
│   │   ├── services/      # ビジネスロジック
│   │   └── ...
│   ├── Cargo.toml
│   └── ...
└── rust-game-engine/      # ゲームロジック（WASM）
```

---

## 4. API設計方針

### RESTful API
- **Rust製Webフレームワーク**でRESTful APIを実装
- **OpenAPI/Swagger**ドキュメント自動生成（Rust用クレート利用）
- **型安全性**: Rustの型システムによるコンパイル時チェック
- **エラーハンドリング**: 統一されたエラー型とレスポンス形式

### リアルタイム通信
- **WebSocket**: tokio-tungstenite等による実装
- **Socket.io互換**: rust-socketio等による実装
- **ルーム管理**: Redisによる状態管理
- **認証・認可**: JWTトークンによる接続制御

---

## 5. 認証・認可設計方針

### JWT認証
- **Rust実装**: jsonwebtokenクレート等を使用
- **トークン管理**: HttpOnly Cookie保存
- **セキュリティ**: CSRF/XSS対策、トークン失効管理

### SNSログイン/OAuth2
- **Rust実装**: oauth2-rsクレート等を使用
- **プロバイダー**: Google、Twitter、GitHub等
- **Clerk連携**: 外部IDaaSとのAPI連携

### セッション管理
- **Redis**: セッション・キャッシュ管理
- **リアルタイム**: WebSocket接続時の認証状態管理

---

## 6. データベース設計方針

### ORM・マイグレーション
- **Diesel**: 型安全なSQLクエリビルダー
- **SeaORM**: 非同期ORM（将来的な選択肢）
- **sqlx**: 生SQL + 型安全性
- **マイグレーション**: diesel_cli、sea-orm-cli等

### スキーマ管理
- **既存Prismaスキーマ**: Rust用スキーマ定義に変換
- **マイグレーション**: バージョン管理・ロールバック対応
- **型生成**: データベーススキーマからRust型の自動生成

---

## 7. テスト・CI/CD戦略

### Rust側テスト
- **ユニットテスト**: cargo test
- **統合テスト**: APIエンドポイントテスト
- **パフォーマンステスト**: cargo bench
- **静的解析**: cargo clippy、cargo fmt

### フロントエンドテスト
- **ユニット/統合**: Jest + React Testing Library
- **E2E**: Playwright/Cypress（API連携テスト含む）

### CI/CD
- **GitHub Actions**: Rust/JS両方のテスト自動化
- **デプロイ**: フロントエンド（Vercel等）+ バックエンド（AWS等）

---

## 8. パフォーマンス・セキュリティ

### パフォーマンス
- **非同期処理**: tokioランタイムによる高並行処理
- **メモリ効率**: Rustのゼロコスト抽象化
- **キャッシュ**: Redisによる高速データアクセス
- **WASM連携**: ゲームロジックの高速実行

### セキュリティ
- **型安全性**: Rustのコンパイル時チェック
- **メモリ安全**: バッファオーバーフロー等の防止
- **認証・認可**: 堅牢なJWT/OAuth2実装
- **入力検証**: 強力なバリデーション

---

## 9. 移行戦略

### Phase 1: 基盤構築
- Rustバックエンドの基本構造構築
- データベース接続・ORM設定
- 基本的なAPIエンドポイント実装

### Phase 2: 認証移行
- JWT認証のRust実装
- Clerk連携の実装
- セッション管理の移行

### Phase 3: 機能移行
- 既存APIの段階的移行
- リアルタイム通信の実装
- フロントエンドとの統合

### Phase 4: 最適化
- パフォーマンス最適化
- セキュリティ強化
- 運用・監視の整備

---

## 10. 今後の拡張・未決事項

### 技術的課題
- Rustエコシステムの学習・習熟
- フロントエンド・バックエンド間の型共有
- WASMとRustバックエンド間のロジック共有
- デプロイ・運用の複雑化対応

### 機能拡張
- リアルタイム対戦機能の実装
- 高度な不正検出システム
- パフォーマンス分析・監視
- 国際化（i18n）対応

---

## 11. 参考：ADR一覧
- doc/adr/adr001_spa_framework.md ～ adr023_database_integration_strategy.md
- **新規**: Rustバックエンド採用のADR（作成予定）

---

本設計書はRustバックエンド移行時の技術選定・設計方針を反映しており、既存のADRに基づいて策定されている。今後の要件・技術動向に応じて随時アップデートする。 