# Phase 4C: Rustバックエンド基盤構築 - ファイル詳細

## 📁 **プロジェクト概要**

**プロジェクト名**: `rust-backend`  
**言語**: Rust  
**Webフレームワーク**: Axum 0.7  
**ORM**: Diesel 2.1  
**データベース**: PostgreSQL  
**認証**: JWT + Clerk（予定）  
**キャッシュ**: Redis  

---

## 📂 **ファイル構造詳細**

### **ルートディレクトリ**
```
rust-backend/
├── Cargo.toml          # 依存関係定義
├── .env               # 環境変数（本番では除外）
└── env.example        # 環境変数テンプレート
```

### **src/ ディレクトリ**
```
src/
├── main.rs            # エントリーポイント
├── api/               # APIルート
├── models/            # データモデル
├── services/          # ビジネスロジック
├── db/                # データベース
└── config/            # 設定管理
```

---

## 📄 **ファイル詳細**

### **1. Cargo.toml**
**目的**: プロジェクトの依存関係とメタデータ定義  
**主要設定**:
- `name = "rust-backend"`
- `version = "0.1.0"`
- `edition = "2021"`

**主要依存関係**:
```toml
# Web Framework
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }

# Database
diesel = { version = "2.1", features = ["postgres", "chrono", "r2d2"] }
diesel_migrations = "2.1"

# Authentication
jsonwebtoken = "9.2"

# Cache & Session
redis = { version = "0.24", features = ["tokio-comp"] }

# HTTP & CORS
tower-http = { version = "0.5", features = ["cors"] }

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Utilities
dotenv = "0.15"
anyhow = "1.0"
thiserror = "1.0"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
reqwest = { version = "0.11", features = ["json"] }
r2d2 = "0.8"
```

### **2. src/main.rs**
**目的**: アプリケーションのエントリーポイント  
**主要機能**:
- 環境変数読み込み
- ログ設定
- データベース接続プール作成
- CORS設定
- ルーター設定
- サーバー起動

**主要コード**:
```rust
#[tokio::main]
async fn main() {
    // 設定読み込み
    let config = AppConfig::from_env().expect("Failed to load configuration");
    
    // ログ設定
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&config.rust_log))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // データベース接続プール作成
    let pool = establish_connection();

    // CORS設定
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // ルーター作成
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .nest("/api/users", api::users_router())
        .nest("/api/scores", api::scores_router())
        .nest("/api/achievements", api::achievements_router())
        .nest("/api/auth", api::auth_router())
        .layer(cors)
        .with_state(pool);

    // サーバー起動
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    tracing::info!("Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### **3. src/config/app_config.rs**
**目的**: アプリケーション設定の管理  
**構造体**:
```rust
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub clerk_secret_key: String,
    pub port: u16,
    pub rust_log: String,
}
```

**主要機能**:
- 環境変数からの設定読み込み
- デフォルト値の設定
- 設定値の検証

### **4. src/db/connection.rs**
**目的**: データベース接続プールの管理  
**主要機能**:
- PostgreSQL接続プール作成
- 接続管理
- エラーハンドリング

**主要コード**:
```rust
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

### **5. src/models/user.rs**
**目的**: ユーザーデータモデル定義  
**構造体**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
}
```

### **6. src/models/score.rs**
**目的**: スコアデータモデル定義  
**構造体**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Score {
    pub id: Uuid,
    pub user_id: Uuid,
    pub score: i32,
    pub lines_cleared: i32,
    pub level: i32,
    pub game_mode: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateScoreRequest {
    pub user_id: Uuid,
    pub score: i32,
    pub lines_cleared: i32,
    pub level: i32,
    pub game_mode: String,
}

#[derive(Serialize, Deserialize)]
pub struct ScoreWithUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub score: i32,
    pub lines_cleared: i32,
    pub level: i32,
    pub game_mode: String,
    pub created_at: DateTime<Utc>,
}
```

### **7. src/models/achievement.rs**
**目的**: 実績データモデル定義  
**構造体**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Achievement {
    pub id: Uuid,
    pub user_id: Uuid,
    pub achievement_type: String,
    pub title: String,
    pub description: String,
    pub unlocked_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateAchievementRequest {
    pub user_id: Uuid,
    pub achievement_type: String,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct AchievementWithUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub achievement_type: String,
    pub title: String,
    pub description: String,
    pub unlocked_at: DateTime<Utc>,
}
```

### **8. src/services/auth_service.rs**
**目的**: JWT認証サービスの実装  
**主要機能**:
- JWTトークン生成
- JWTトークン検証
- クレーム管理

**主要コード**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: usize,  // expiration
    pub iat: usize,  // issued at
}

pub struct AuthService;

impl AuthService {
    pub fn create_token(user_id: &str) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let issued_at = Utc::now().timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            iat: issued_at,
        };

        let secret = env::var("JWT_SECRET")?;
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )?;

        Ok(token)
    }

    pub fn verify_token(token: &str) -> Result<Claims> {
        let secret = env::var("JWT_SECRET")?;
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }
}
```

### **9. src/services/game_service.rs**
**目的**: ゲーム関連のビジネスロジック  
**主要機能**:
- スコア計算
- 実績チェック

**主要コード**:
```rust
pub struct GameService;

impl GameService {
    pub fn calculate_score(lines_cleared: i32, level: i32) -> i32 {
        let base_score = match lines_cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0,
        };
        
        base_score * level
    }

    pub fn check_achievements(score: i32, lines_cleared: i32, level: i32) -> Vec<CreateAchievementRequest> {
        let mut achievements = Vec::new();
        
        if score >= 10000 {
            achievements.push(CreateAchievementRequest {
                user_id: Uuid::nil(),
                achievement_type: "score".to_string(),
                title: "High Scorer".to_string(),
                description: "Reach 10,000 points".to_string(),
            });
        }
        
        // 他の実績チェック...
        
        achievements
    }
}
```

### **10. src/api/users.rs**
**目的**: ユーザー管理APIエンドポイント  
**エンドポイント**:
- `GET /api/users` - ユーザー一覧取得
- `POST /api/users` - ユーザー作成
- `GET /api/users/:id` - ユーザー詳細取得
- `PUT /api/users/:id` - ユーザー更新
- `DELETE /api/users/:id` - ユーザー削除

**主要機能**:
- CRUD操作のエンドポイント
- モックデータによる応答
- エラーハンドリング

### **11. src/api/scores.rs**
**目的**: スコア管理APIエンドポイント  
**エンドポイント**:
- `GET /api/scores` - スコア一覧取得
- `POST /api/scores` - スコア作成
- `GET /api/scores/:id` - スコア詳細取得
- `GET /api/scores/ranking` - ランキング取得

**主要機能**:
- スコアCRUD操作
- ランキング取得
- ゲームモード別フィルタリング

### **12. src/api/achievements.rs**
**目的**: 実績管理APIエンドポイント  
**エンドポイント**:
- `GET /api/achievements` - 実績一覧取得
- `POST /api/achievements` - 実績作成
- `GET /api/achievements/:id` - 実績詳細取得
- `GET /api/achievements/user/:user_id` - ユーザー実績取得

**主要機能**:
- 実績CRUD操作
- ユーザー別実績取得
- 実績タイプ別フィルタリング

### **13. src/api/auth.rs**
**目的**: 認証APIエンドポイント  
**エンドポイント**:
- `POST /api/auth/login` - ログイン
- `POST /api/auth/verify` - トークン検証
- `POST /api/auth/logout` - ログアウト

**主要機能**:
- JWT認証
- トークン検証
- セッション管理

---

## 🔧 **環境変数設定**

### **.envファイル**
```bash
# データベース
DATABASE_URL="postgresql://tetris_user:tetris_password@localhost:5432/tetris_dev"
REDIS_URL="redis://localhost:6379"

# 認証
JWT_SECRET="your-super-secret-jwt-key-change-this-in-production"
CLERK_SECRET_KEY="your-clerk-secret"

# サーバー設定
RUST_LOG="info"
PORT="8080"
```

---

## 🚀 **ビルド・実行方法**

### **依存関係インストール**
```bash
cargo build
```

### **コンパイルチェック**
```bash
cargo check
```

### **サーバー起動**
```bash
cargo run
```

### **テスト実行**
```bash
cargo test
```

---

## 📊 **APIエンドポイント一覧**

### **基本エンドポイント**
- `GET /` - ルート（"ClaudeTetris Rust Backend API"）
- `GET /health` - ヘルスチェック（"OK"）

### **ユーザーAPI**
- `GET /api/users` - ユーザー一覧取得
- `POST /api/users` - ユーザー作成
- `GET /api/users/:id` - ユーザー詳細取得
- `PUT /api/users/:id` - ユーザー更新
- `DELETE /api/users/:id` - ユーザー削除

### **スコアAPI**
- `GET /api/scores` - スコア一覧取得
- `POST /api/scores` - スコア作成
- `GET /api/scores/:id` - スコア詳細取得
- `GET /api/scores/ranking` - ランキング取得

### **実績API**
- `GET /api/achievements` - 実績一覧取得
- `POST /api/achievements` - 実績作成
- `GET /api/achievements/:id` - 実績詳細取得
- `GET /api/achievements/user/:user_id` - ユーザー実績取得

### **認証API**
- `POST /api/auth/login` - ログイン
- `POST /api/auth/verify` - トークン検証
- `POST /api/auth/logout` - ログアウト

---

## ⚠️ **制限事項・TODO**

### **現在の制限**
- モックデータによるAPI応答
- 実際のデータベースアクセス未実装
- Clerk認証連携未実装
- Redis連携未実装
- エラーハンドリングが基本的な実装
- バリデーション未実装
- テスト未実装

### **次のステップ**
- データベースマイグレーション実装
- 実際のデータベースアクセス実装
- Clerk認証連携実装
- Redis連携実装
- エラーハンドリング強化
- バリデーション実装
- テスト実装
- APIドキュメント生成

---

## 🎯 **成果**

- ✅ **Rustバックエンドの基盤構築完了**
- ✅ **Axum Webフレームワーク統合**
- ✅ **基本的なAPIエンドポイント実装**
- ✅ **型安全なデータモデル定義**
- ✅ **JWT認証の基本実装**
- ✅ **CORS設定済み**
- ✅ **ログ設定済み**
- ✅ **環境変数管理実装**
- ✅ **コンパイル成功確認**

この基盤により、**最高のパフォーマンス**と**型安全性**を備えたRustバックエンドが構築されました。 