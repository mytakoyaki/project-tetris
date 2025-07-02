# ClaudeTetris アーキテクチャ変更差分

## 概要
現状のNext.js API/Prisma構成からRustバックエンド構成への変更点をまとめる。

---

## 1. 技術スタック変更

### バックエンド/API
```diff
- **バックエンド/API**: Next.js API Route（Node.js）、TypeScript
+ **バックエンド/API**: Rust（Axum/Actix-web/Poem）、TypeScript（WASM連携）
```

### データベース
```diff
- **ORM**: Prisma（Node.js）
+ **ORM**: Diesel/SeaORM/sqlx（Rust）
- **マイグレーション**: Prisma Migrate
+ **マイグレーション**: diesel_cli、sea-orm-cli等
```

### 認証
```diff
- **JWT実装**: Node.js（jsonwebtoken等）
+ **JWT実装**: Rust（jsonwebtokenクレート等）
- **OAuth2実装**: NextAuth.js等
+ **OAuth2実装**: oauth2-rsクレート等
```

### リアルタイム通信
```diff
- **Socket.io**: Node.js + Socket.io
+ **Socket.io**: rust-socketio or WebSocket（tokio-tungstenite）
```

---

## 2. ディレクトリ構成変更

### プロジェクト構造
```diff
project-tetris/
├── modern-tetris/          # フロントエンド（Next.js）
│   ├── src/
│   │   ├── app/
│   │   │   ├── api/        # 削除予定
│   │   │   └── ...
│   │   ├── features/
│   │   ├── components/
│   │   ├── store/
│   │   └── ...
│   ├── prisma/             # 削除予定
│   └── ...
+ ├── rust-backend/         # 新規追加
+ │   ├── src/
+ │   │   ├── api/          # APIルート
+ │   │   ├── auth/         # 認証・認可
+ │   │   ├── models/       # データモデル
+ │   │   ├── services/     # ビジネスロジック
+ │   │   └── ...
+ │   ├── Cargo.toml
+ │   └── ...
└── rust-game-engine/       # 既存（WASM）
```

---

## 3. API設計変更

### API実装場所
```diff
- **API実装**: src/app/api/（Next.js API Routes）
+ **API実装**: rust-backend/src/api/（Rust Webフレームワーク）
```

### 型安全性
```diff
- **型チェック**: TypeScript（実行時）
+ **型チェック**: Rust（コンパイル時）+ TypeScript（フロント）
```

### ドキュメント生成
```diff
- **OpenAPI**: Next.js + Swagger等
+ **OpenAPI**: Rust用クレート（utoipa等）
```

---

## 4. データベース変更

### スキーマ定義
```diff
- **スキーマ**: prisma/schema.prisma
+ **スキーマ**: rust-backend/src/models/（Rust構造体）
```

### クエリ実行
```diff
- **クエリ**: Prisma Client（TypeScript）
+ **クエリ**: Diesel/SeaORM/sqlx（Rust）
```

### マイグレーション
```diff
- **マイグレーション**: npx prisma migrate
+ **マイグレーション**: diesel migration run
```

---

## 5. 認証・認可変更

### JWT実装
```diff
- **JWT**: Node.js（jsonwebtoken等）
+ **JWT**: Rust（jsonwebtokenクレート等）
```

### OAuth2実装
```diff
- **OAuth2**: NextAuth.js
+ **OAuth2**: oauth2-rsクレート + Clerk連携
```

### セッション管理
```diff
- **セッション**: Next.js + Redis
+ **セッション**: Rust + Redis（redis-rsクレート）
```

---

## 6. テスト・CI/CD変更

### テスト実行
```diff
- **バックエンドテスト**: Jest（Node.js）
+ **バックエンドテスト**: cargo test（Rust）
```

### CI/CD
```diff
- **CI**: GitHub Actions（Node.js/TypeScript）
+ **CI**: GitHub Actions（Rust + Node.js/TypeScript）
```

### デプロイ
```diff
- **デプロイ**: Vercel（フロントエンド + API）
+ **デプロイ**: Vercel（フロントエンド）+ AWS等（Rustバックエンド）
```

---

## 7. 開発環境変更

### 依存関係管理
```diff
- **パッケージ管理**: npm/yarn（package.json）
+ **パッケージ管理**: npm/yarn（フロント）+ Cargo（バックエンド）
```

### 開発サーバー
```diff
- **開発サーバー**: npm run dev（Next.js）
+ **開発サーバー**: npm run dev（フロント）+ cargo run（バックエンド）
```

### 環境変数
```diff
- **環境変数**: .env（Next.js）
+ **環境変数**: .env（フロント）+ .env（Rustバックエンド）
```

---

## 8. パフォーマンス・セキュリティ変更

### パフォーマンス
```diff
+ **非同期処理**: tokioランタイムによる高並行処理
+ **メモリ効率**: Rustのゼロコスト抽象化
+ **型安全性**: コンパイル時チェック
```

### セキュリティ
```diff
+ **メモリ安全**: バッファオーバーフロー等の防止
+ **強力なバリデーション**: Rustの型システム
```

---

## 9. 移行時の影響

### 既存機能への影響
- **フロントエンド**: 大幅な変更なし（API呼び出し部分のみ調整）
- **ゲームロジック**: 変更なし（WASMは維持）
- **データベース**: スキーマは維持、アクセス方法のみ変更

### 開発効率への影響
- **学習コスト**: Rustエコシステムの習熟が必要
- **開発速度**: 初期は低下、習熟後は向上
- **デバッグ**: Rustの強力な型システムにより改善

### 運用への影響
- **デプロイ**: 複雑化（フロントエンド + バックエンド分離）
- **監視**: 両方のサービス監視が必要
- **スケーリング**: バックエンドのみ独立スケール可能

---

## 10. 段階的移行戦略

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

## まとめ

Rustバックエンド移行により、**パフォーマンス・セキュリティ・型安全性**が大幅に向上する一方、**開発コスト・運用複雑性**が増加する。段階的な移行により、リスクを最小化しつつ、長期的な技術的優位性を獲得できる。 