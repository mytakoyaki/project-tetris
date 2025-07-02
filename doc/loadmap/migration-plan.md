# 🔄 ClaudeTetris マイグレーション計画（2024-12-25更新・Rustバックエンド優先）

## 📋 **作業全体の流れ**

### **Phase 1: 基盤構築（完了済み）**
- ✅ **Next.js 15 + React 19 セットアップ**
- ✅ **Material-UI + TypeScript 統合**
- ✅ **Redux Toolkit 状態管理**
- ✅ **基本的なゲーム機能実装**

### **Phase 2: Rust + WASM統合（完了済み）**
- ✅ **rust-game-engineプロジェクト作成**
- ✅ **WebAssembly対応設定**
- ✅ **SRS回転システム実装**
- ✅ **スピン検出システム実装**
- ✅ **WASM統合完了**

### **Phase 3: ゲーム機能拡張（完了済み）**
- ✅ **フィーバーモード実装**
- ✅ **実績システム実装**
- ✅ **ポイント交換システム**
- ✅ **エフェクトシステム**

### **Phase 4: Rustバックエンド構築（進行中）**
- ✅ **データベース統合（Phase 4B）**
  - ✅ **Docker環境構築（PostgreSQL + Redis）**
  - ✅ **Prismaスキーマ定義・適用**
  - ✅ **データベース初期化（Seed）**
- 🔄 **Rustバックエンド基盤構築（Phase 4C）**
  - ⏳ **Rust Webフレームワーク選定・設定**
  - ⏳ **ORM設定（Diesel/SeaORM）**
  - ⏳ **基本的なAPIエンドポイント実装**
- ⏳ **認証システム実装（Phase 4D）**
  - ⏳ **JWT認証のRust実装**
  - ⏳ **Clerk連携の実装**
  - ⏳ **セッション管理**

### **Phase 5: フロントエンド・バックエンド統合（予定）**
- ⏳ **API通信実装**
- ⏳ **RTK Query統合**
- ⏳ **型安全性確保**
- ⏳ **エラーハンドリング**

### **Phase 6: リアルタイム機能（予定）**
- ⏳ **WebSocket/Socket.io実装**
- ⏳ **オンラインマルチプレイヤー**
- ⏳ **マッチングシステム**
- ⏳ **チャット機能**

### **Phase 7: 高度な機能（予定）**
- ⏳ **リプレイシステム**
- ⏳ **トーナメント機能**
- ⏳ **モバイル対応**
- ⏳ **パフォーマンス最適化**

---

## 🎯 **詳細移行計画**

### **Phase 4C: Rustバックエンド基盤構築（現在進行中）**

#### **Step 1: Rustプロジェクト作成**
```bash
# Rustバックエンドプロジェクト作成
cd ..
cargo new rust-backend
cd rust-backend

# 依存関係設定
# Cargo.toml
[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
diesel = { version = "2.1", features = ["postgres", "chrono"] }
diesel_migrations = "2.1"
jsonwebtoken = "9.2"
redis = { version = "0.24", features = ["tokio-comp"] }
tower-http = { version = "0.5", features = ["cors"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

#### **Step 2: プロジェクト構造**
```
rust-backend/
├── src/
│   ├── main.rs              # エントリーポイント
│   │   ├── api/                 # APIルート
│   │   │   ├── mod.rs
│   │   │   ├── users.rs
│   │   │   ├── scores.rs
│   │   │   ├── achievements.rs
│   │   │   └── auth.rs
│   │   ├── models/              # データモデル
│   │   │   ├── mod.rs
│   │   │   ├── user.rs
│   │   │   ├── score.rs
│   │   │   └── achievement.rs
│   │   ├── services/            # ビジネスロジック
│   │   │   ├── mod.rs
│   │   │   ├── auth_service.rs
│   │   │   └── game_service.rs
│   │   ├── db/                  # データベース
│   │   │   ├── mod.rs
│   │   │   └── connection.rs
│   │   └── config/              # 設定
│   │       ├── mod.rs
│   │       └── app_config.rs
│   ├── migrations/              # データベースマイグレーション
│   ├── Cargo.toml
│   └── .env
```

#### **Step 3: データベース接続・ORM設定**
```rust
// src/db/connection.rs
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Pool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
```

#### **Step 4: 基本的なAPIエンドポイント実装**
```rust
// src/api/users.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
}

pub async fn get_users(
    State(pool): State<Pool>,
) -> Result<Json<Vec<User>>, StatusCode> {
    // データベースからユーザー取得
    // ...
    Ok(Json(users))
}
```

### **Phase 4D: 認証システム実装**

#### **Step 1: JWT認証実装**
```rust
// src/services/auth_service.rs
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: usize,  // expiration
}

pub fn create_token(user_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()),
    )
    .map_err(|e| e.into())
}
```

#### **Step 2: Clerk連携実装**
```rust
// src/services/clerk_service.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClerkUser {
    pub id: String,
    pub email: String,
    pub username: Option<String>,
}

pub async fn verify_clerk_token(token: &str) -> Result<ClerkUser, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .get("https://api.clerk.dev/v1/me")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    let user: ClerkUser = response.json().await?;
    Ok(user)
}
```

---

## 🎯 **技術スタック概要**

### **フロントエンド（維持）**
- **フレームワーク**: Next.js 15 (App Router) + React 19 + TypeScript
- **UI**: Material-UI (MUI) + Tailwind CSS
- **状態管理**: Redux Toolkit + RTK Query
- **ゲームエンジン**: Rust + WebAssembly

### **バックエンド（Rust構築）**
- **Webフレームワーク**: Axum/Actix-web/Poem
- **ORM**: Diesel/SeaORM/sqlx
- **データベース**: PostgreSQL (Docker) + Redis
- **認証**: Rust JWTクレート + Clerk連携
- **リアルタイム通信**: WebSocket（tokio-tungstenite）

### **インフラ・デプロイ**
- **フロントエンド**: Vercel
- **バックエンド**: AWS ECS/Fargate等
- **データベース**: AWS RDS
- **キャッシュ**: Upstash Redis
- **CI/CD**: GitHub Actions

---

## 📊 **移行進捗管理**

### **完了済み（✅）**
- [x] Next.js 15 + React 19 セットアップ
- [x] Material-UI + TypeScript 統合
- [x] Redux Toolkit 状態管理
- [x] 基本的なゲーム機能実装
- [x] rust-game-engineプロジェクト作成
- [x] WebAssembly対応設定
- [x] SRS回転システム実装
- [x] スピン検出システム実装
- [x] WASM統合完了
- [x] フィーバーモード実装
- [x] 実績システム実装
- [x] ポイント交換システム
- [x] エフェクトシステム
- [x] Docker環境構築（PostgreSQL + Redis）
- [x] Prismaスキーマ定義・適用
- [x] データベース初期化（Seed）

### **進行中（🔄）**
- [ ] Rustバックエンド基盤構築
- [ ] ORM設定（Diesel/SeaORM）
- [ ] 基本的なAPIエンドポイント実装

### **未着手（⏳）**
- [ ] JWT認証のRust実装
- [ ] Clerk連携の実装
- [ ] フロントエンド・バックエンド統合
- [ ] RTK Query統合
- [ ] WebSocket/Socket.io実装
- [ ] オンラインマルチプレイヤー機能
- [ ] マッチングシステム
- [ ] チャット機能
- [ ] リプレイシステム
- [ ] トーナメント機能
- [ ] モバイル対応
- [ ] パフォーマンス最適化

---

## 🚀 **次のステップ**

### **Phase 4C: Rustバックエンド基盤構築（現在進行中）**
1. **Rustプロジェクト作成**
   - Cargo.toml設定
   - プロジェクト構造作成
   - 依存関係インストール

2. **データベース接続・ORM設定**
   - Diesel/SeaORM設定
   - 接続プール設定
   - マイグレーション設定

3. **基本的なAPIエンドポイント実装**
   - ユーザー管理API
   - スコア管理API
   - 実績管理API

### **Phase 4D: 認証システム実装（予定）**
1. **JWT認証のRust実装**
2. **Clerk連携の実装**
3. **セッション管理**

### **Phase 5: フロントエンド・バックエンド統合（予定）**
1. **API通信実装**
2. **RTK Query統合**
3. **型安全性確保**

---

## 🛠️ **開発環境セットアップ**

### **前提条件**
```bash
# Node.js 20以上
node --version

# Docker & Docker Compose
docker --version
docker-compose --version

# Rust
rustc --version
cargo --version

# WASM
wasm-pack --version
```

### **データベース環境（現在稼働中）**
```bash
# PostgreSQL + Redis 起動
cd modern-tetris
docker-compose up -d

# データベース接続確認
psql -h localhost -U tetris_user -d tetris_dev
redis-cli ping
```

### **Rustバックエンド環境**
```bash
# Rustバックエンドプロジェクト作成
cd ..
cargo new rust-backend
cd rust-backend

# 依存関係インストール
cargo build

# 開発サーバー起動
cargo run

# テスト実行
cargo test
```

### **Next.js環境（フロントエンド）**
```bash
# プロジェクトディレクトリ移動
cd modern-tetris

# 依存関係インストール
npm install

# 開発サーバー起動
npm run dev

# テスト実行
npm test
```

## 🔧 **環境変数設定**

### **Rustバックエンド (.env)**
```bash
# データベース
DATABASE_URL="postgresql://tetris_user:tetris_password@localhost:5432/tetris_dev"
REDIS_URL="redis://localhost:6379"

# 認証
JWT_SECRET="your-super-secret-jwt-key-change-this-in-production"

# Clerk
CLERK_SECRET_KEY="your-clerk-secret"

# サーバー設定
RUST_LOG="info"
PORT="8080"
```

### **Next.jsフロントエンド (.env)**
```bash
# API接続
NEXT_PUBLIC_API_URL="http://localhost:8080"

# Clerk
NEXT_PUBLIC_CLERK_PUBLISHABLE_KEY="your-clerk-key"

# Next.js
NEXTAUTH_URL="http://localhost:3000"
NODE_ENV="development"
```

---

## 🚀 **パフォーマンス目標**

### **Rustバックエンド目標**
- **API応答時間**: <50ms
- **同時接続数**: 1000+（WebSocket）
- **メモリ使用量**: <100MB
- **スループット**: 10000+ req/sec

### **フロントエンド目標（維持）**
- **FPS**: 60 FPS（WASM統合後）
- **メモリ使用量**: 30-50MB（WASM統合後）
- **初期化時間**: 1-2秒（WASM統合後）
- **入力遅延**: <16ms（WASM統合後）

---

## 🔧 **品質保証**

### **テスト戦略**
1. **単体テスト**: 各Rustモジュール・Reactコンポーネント
2. **統合テスト**: API・データベース統合
3. **E2Eテスト**: フロントエンド・バックエンド統合
4. **パフォーマンステスト**: ベンチマーク測定

### **コード品質**
1. **型安全性**: 100% TypeScript + Rust型チェック
2. **エラーハンドリング**: 包括的なエラー処理
3. **ドキュメント**: 詳細なAPI仕様書
4. **コメント**: 複雑なロジックの説明

---

## 📈 **リスク管理**

### **技術的リスク**
- **Rust学習コスト**: チームの習熟度向上が必要
- **フロントエンド・バックエンド統合**: 型共有・通信設計
- **デプロイ複雑化**: 複数サービスの管理

### **対策**
- **段階的実装**: 機能ごとに順次実装
- **テスト駆動**: 各段階での動作確認
- **ドキュメント整備**: 設計・実装の記録

---

## 🎯 **成功指標**

### **技術指標**
- [x] 60FPSでの安定動作（フロントエンド）
- [x] メモリ使用量50MB以下（フロントエンド）
- [x] 初期化時間2秒以下（フロントエンド）
- [x] 入力遅延16ms以下（フロントエンド）
- [ ] API応答時間50ms以下（バックエンド）
- [ ] 同時接続数1000+（バックエンド）

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

---

## 📚 **参考資料**

### **アーキテクチャ設計**
- `doc/architecture/rust_backend_architecture.md` - Rustバックエンド設計
- `doc/architecture/architecture_diff.md` - 変更差分

### **技術選定**
- `doc/adr/` - アーキテクチャ決定記録（ADR）

### **実装詳細**
- `doc/files/phase4b.md` - Phase 4B実装内容
- `log/rust-migration/` - Rust移行ログ

この移行計画により、**Rustバックエンドを最初から構築**し、**最高のパフォーマンス**と**開発効率**を両立したモダンなテトリスゲームが完成します。 