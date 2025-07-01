# Phase 4: フルスタック機能実装 - API Routes・認証・型定義

## 概要
Phase 4では、Next.js API Routes、JWT認証、型安全なAPI通信を実装し、クライアントサイドのみの実装からフルスタックアプリケーションへの移行を開始。

## 実装内容

### 1. Next.js API Routes 実装

#### 1.1 ユーザー管理 API (`/api/users`)
- **GET /api/users**: 全ユーザー取得
- **POST /api/users**: 新規ユーザー作成
- 基本的なバリデーション（username、email必須）
- 重複チェック機能

#### 1.2 スコア管理 API (`/api/scores`)
- **GET /api/scores**: スコア一覧取得（フィルタリング・ページネーション対応）
- **POST /api/scores**: 新規スコア投稿
- クエリパラメータ: `userId`, `gameMode`, `limit`, `offset`
- スコア順ソート機能

#### 1.3 実績管理 API (`/api/achievements`)
- **GET /api/achievements**: ユーザー実績取得
- **POST /api/achievements**: 実績進捗更新・解除
- クエリパラメータ: `userId`, `category`, `unlocked`
- 新規解除判定機能

#### 1.4 ランキング API (`/api/ranking`)
- **GET /api/ranking**: リーダーボード取得
- **POST /api/ranking**: ランキング更新
- クエリパラメータ: `gameMode`, `limit`, `offset`, `timeRange`
- 時間範囲フィルタリング（daily/weekly/monthly/all）

### 2. JWT認証システム

#### 2.1 認証 API (`/api/auth`)
- **POST /api/auth/login**: ユーザーログイン
- **POST /api/auth/logout**: ユーザーログアウト
- JWTトークン生成・検証
- HTTP-only Cookie設定

#### 2.2 認証ミドルウェア (`src/middleware.ts`)
- JWTトークン検証
- 認証必須ルートの保護
- ユーザー情報をリクエストヘッダーに追加
- 未認証時のリダイレクト処理

### 3. 型定義の整備

#### 3.1 API型定義 (`src/types/api.ts`)
- ユーザー関連型: `User`, `CreateUserRequest`, `CreateUserResponse`
- スコア関連型: `Score`, `CreateScoreRequest`, `GetScoresResponse`
- 実績関連型: `UserAchievement`, `CreateAchievementRequest`
- ランキング関連型: `Ranking`, `CreateRankingRequest`
- 認証関連型: `LoginRequest`, `LoginResponse`, `JWTPayload`
- エラーレスポンス型: `ErrorResponse`

#### 3.2 型安全性の確保
- API リクエスト/レスポンスの型定義
- ミドルウェアでの認証情報型定義
- TypeScriptによるコンパイル時型チェック

### 4. 開発・テスト環境

#### 4.1 API テストページ (`/api-test`)
- 全APIエンドポイントの動作確認
- 認証フローのテスト
- レスポンス表示機能

#### 4.2 モックデータ
- 開発用のサンプルデータ
- ユーザー、スコア、実績、ランキングの初期データ
- テスト用の認証情報

## 技術スタック

### バックエンド
- **Next.js API Routes**: RESTful API実装
- **JWT**: 認証トークン管理
- **TypeScript**: 型安全性確保

### セキュリティ
- **HTTP-only Cookies**: XSS攻撃対策
- **JWT Secret**: 環境変数による秘密鍵管理
- **CORS**: 適切なオリジン制御

### 開発ツール
- **jsonwebtoken**: JWT生成・検証
- **@types/jsonwebtoken**: TypeScript型定義

## 次のステップ

### Phase 4B: データベース統合
- Prisma ORM導入
- PostgreSQL/MySQLスキーマ設計
- マイグレーション実行

### Phase 4C: クライアント統合
- SWR/React Query導入
- 型安全なAPI呼び出し
- 認証状態管理

### Phase 4D: 本格運用準備
- 環境変数設定
- 本番環境デプロイ
- セキュリティ強化

## 実装状況

### ✅ 完了
- [x] Next.js API Routes基本構造
- [x] ユーザー管理API
- [x] スコア管理API
- [x] 実績管理API
- [x] ランキングAPI
- [x] JWT認証システム
- [x] 認証ミドルウェア
- [x] 型定義整備
- [x] APIテストページ

### 🔄 進行中
- [ ] データベース統合
- [ ] クライアント側API呼び出し
- [ ] エラーハンドリング強化

### ⏳ 未着手
- [ ] 本番環境設定
- [ ] パフォーマンス最適化
- [ ] セキュリティ監査

## アーキテクチャ設計書との対応

Phase 4の実装により、以下のアーキテクチャ設計書の要件を満たしている：

1. **Next.js App Router**: API Routes実装 ✅
2. **RESTful API設計**: 標準的なHTTPメソッド使用 ✅
3. **JWT認証**: セキュアな認証システム ✅
4. **型安全性**: TypeScriptによる型定義 ✅
5. **機能別ディレクトリ構成**: API Routesの適切な配置 ✅

現在の実装は、設計書が想定するフルスタックアプリケーションの基盤部分を完成させており、次のPhaseでデータベース統合とクライアント側統合を進める準備が整っている。 