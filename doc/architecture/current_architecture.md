# ClaudeTetris アーキテクチャ設計書

## 概要

ClaudeTetrisは、Next.js 15 + React 19を基盤としたモダンなTetrisゲームアプリケーションです。Rust + WebAssemblyによる高性能ゲームエンジン、フルスタック機能、リアルタイムマルチプレイヤー機能を備えた包括的なゲームプラットフォームです。

## 技術スタック

### フロントエンド
- **フレームワーク**: Next.js 15 (App Router)
- **UI**: React 19 + Material-UI (MUI)
- **状態管理**: Redux Toolkit + RTK Query
- **言語**: TypeScript
- **スタイリング**: Tailwind CSS + Emotion

### バックエンド
- **API**: Next.js API Routes
- **ORM**: Prisma
- **データベース**: PostgreSQL (AWS RDS)
- **キャッシュ**: Redis (Upstash)
- **認証**: Clerk
- **リアルタイム通信**: Socket.io

### ゲームエンジン
- **言語**: Rust
- **実行環境**: WebAssembly (WASM)
- **機能**: SRS回転システム、スピン検出、衝突判定

### インフラ・デプロイ
- **ホスティング**: Vercel
- **データベース**: AWS RDS
- **キャッシュ**: Upstash Redis
- **認証**: Clerk
- **CI/CD**: GitHub Actions

## システムアーキテクチャ

```
┌─────────────────────────────────────────────────────────────┐
│                        Client Layer                         │
├─────────────────────────────────────────────────────────────┤
│  Next.js App (React 19)                                     │
│  ├── Game Components (Material-UI)                          │
│  ├── Redux Toolkit + RTK Query                              │
│  ├── Rust WASM Game Engine                                  │
│  └── Socket.io Client                                       │
├─────────────────────────────────────────────────────────────┤
│                        API Layer                            │
├─────────────────────────────────────────────────────────────┤
│  Next.js API Routes                                         │
│  ├── Authentication (Clerk)                                 │
│  ├── User Management                                        │
│  ├── Score Management                                       │
│  ├── Achievement System                                     │
│  └── Ranking System                                         │
├─────────────────────────────────────────────────────────────┤
│                     Data Layer                              │
├─────────────────────────────────────────────────────────────┤
│  PostgreSQL (AWS RDS) ← Prisma ORM                          │
│  Redis (Upstash) ← Session/Cache                            │
│  Socket.io Server ← Real-time Communication                 │
└─────────────────────────────────────────────────────────────┘
```

## コンポーネント詳細

### 1. フロントエンド (Client Layer)

#### ゲームコンポーネント
- **TetrisGame**: メインゲームコンポーネント
- **GameField**: ゲームフィールド表示
- **GameControls**: ゲーム操作UI
- **Sidebar**: スコア・レベル・実績表示
- **FeverModeEffects**: フィーバーモードエフェクト

#### 状態管理
- **Redux Store**: アプリケーション全体の状態
- **RTK Query**: API通信とキャッシュ管理
- **Game Slice**: ゲーム状態管理
- **User Slice**: ユーザー情報管理
- **Score Slice**: スコア管理
- **Achievement Slice**: 実績管理

#### Rust WASM統合
- **rustGameEngine.ts**: WASMモジュールのラッパー
- **useGameEngine.ts**: ゲームエンジンフック
- **衝突判定**: Rust実装
- **SRS回転**: Rust実装
- **スピン検出**: Rust実装

### 2. API層 (API Layer)

#### 認証API
```typescript
POST /api/auth/login    // ユーザーログイン
POST /api/auth/logout   // ユーザーログアウト
```

#### ユーザー管理API
```typescript
GET  /api/users         // ユーザー一覧取得
POST /api/users         // 新規ユーザー作成
GET  /api/users/:id     // ユーザー詳細取得
PUT  /api/users/:id     // ユーザー情報更新
```

#### スコア管理API
```typescript
GET  /api/scores        // スコア一覧取得（フィルタリング・ページネーション）
POST /api/scores        // 新規スコア投稿
GET  /api/scores/:id    // スコア詳細取得
```

#### 実績管理API
```typescript
GET  /api/achievements  // ユーザー実績取得
POST /api/achievements  // 実績進捗更新・解除
```

#### ランキングAPI
```typescript
GET  /api/ranking       // リーダーボード取得
POST /api/ranking       // ランキング更新
```

### 3. データ層 (Data Layer)

#### PostgreSQL スキーマ
```sql
-- ユーザーテーブル
User (id, username, email, password, createdAt, lastLoginAt, rating)

-- スコアテーブル
Score (id, userId, score, level, lines, time, gameMode, createdAt, achievements)

-- ゲーム統計テーブル
GameStats (id, userId, totalGames, totalScore, highestScore, totalLines, totalTime)

-- 実績テーブル
Achievement (id, name, description, category, maxProgress)

-- ユーザー実績テーブル
UserAchievement (id, userId, achievementId, progress, unlockedAt)
```

#### Redis キャッシュ戦略
- **セッション管理**: ユーザーセッション
- **ランキングキャッシュ**: リアルタイムランキング
- **ゲームルーム**: アクティブルーム情報
- **ユーザー統計**: 頻繁にアクセスされる統計データ

## 認証・セキュリティ

### Clerk認証フロー
1. **ユーザー登録**: メール/パスワード or SNSログイン
2. **セッション管理**: JWTトークン + HTTP-only Cookie
3. **権限管理**: ロールベースアクセス制御
4. **セキュリティ**: 多要素認証、不正検知

### API セキュリティ
- **認証ミドルウェア**: JWT検証
- **CORS設定**: 適切なオリジン制限
- **レート制限**: API呼び出し制限
- **入力検証**: バリデーション強化

## リアルタイム通信

### Socket.io アーキテクチャ
```typescript
// サーバーサイド
io.on('connection', (socket) => {
  socket.on('join-room', (roomId) => {
    socket.join(roomId);
    // ルーム参加処理
  });
  
  socket.on('game-action', (data) => {
    // ゲームアクション処理
    socket.to(data.roomId).emit('game-update', data);
  });
});

// クライアントサイド
const socket = useSocket();
socket?.emit('join-room', roomId);
socket?.on('game-update', handleGameUpdate);
```

### リアルタイム機能
- **1v1マッチング**: スキルベースマッチング
- **ゲーム状態同期**: リアルタイムゲーム状態
- **チャット機能**: リアルタイムチャット
- **スペクテーター**: 観戦機能

## パフォーマンス最適化

### フロントエンド最適化
- **コード分割**: React.lazyによる遅延読み込み
- **メモ化**: React.memo, useMemo, useCallback
- **バンドル最適化**: Tree shaking, 動的インポート
- **WASM最適化**: Rustゲームエンジンの効率的な統合

### バックエンド最適化
- **データベース**: インデックス最適化、クエリ最適化
- **キャッシュ**: Redis活用、CDN活用
- **API**: レスポンス最適化、バッチ処理

### リアルタイム最適化
- **メッセージ最適化**: 最小限のデータ転送
- **接続管理**: 効率的な接続プール
- **スケーリング**: 水平スケーリング対応

## 開発・デプロイフロー

### 開発環境
```bash
# ローカル開発
npm run dev          # Next.js開発サーバー
docker-compose up    # PostgreSQL + Redis
npm run test         # テスト実行
```

### CI/CD パイプライン
```yaml
# GitHub Actions
- テスト実行
- ビルド
- デプロイ (Vercel)
- データベースマイグレーション
```

### 環境管理
- **開発環境**: ローカル + Docker
- **ステージング環境**: Vercel Preview
- **本番環境**: Vercel Production + AWS RDS

## 監視・ログ

### パフォーマンス監視
- **Vercel Analytics**: フロントエンドパフォーマンス
- **AWS CloudWatch**: データベース監視
- **Redis Monitoring**: キャッシュパフォーマンス

### エラー監視
- **Sentry**: エラー追跡
- **ログ管理**: 構造化ログ
- **アラート**: 異常検知

## 今後の拡張計画

### Phase 5: Clerk認証移行
- [ ] Clerkプロジェクト設定
- [ ] 認証UIコンポーネント実装
- [ ] SNSログイン統合
- [ ] JWT認証からの段階的移行

### Phase 6: リアルタイム機能
- [ ] Socket.io統合
- [ ] ゲームルーム機能
- [ ] マッチングシステム
- [ ] チャット機能

### Phase 7: 高度な機能
- [ ] リプレイシステム
- [ ] トーナメント機能
- [ ] カスタムテーマ
- [ ] モバイル対応

## 技術的制約・考慮事項

### ブラウザ互換性
- **Chrome**: 最新版
- **Firefox**: 最新版
- **Safari**: 最新版
- **Edge**: 最新版

### パフォーマンス要件
- **初期読み込み**: < 3秒
- **ゲーム応答性**: < 16ms (60fps)
- **API応答時間**: < 200ms
- **リアルタイム遅延**: < 100ms

### スケーラビリティ
- **同時接続数**: 1,000ユーザー
- **データベース**: 100万レコード
- **ストレージ**: 10GB
- **帯域幅**: 100Mbps

## まとめ

ClaudeTetrisは、モダンなWeb技術を活用した高性能なTetrisゲームプラットフォームです。Rust + WASMによるゲームエンジン、Next.jsによるフルスタック開発、リアルタイム通信機能により、ユーザーに優れたゲーム体験を提供します。

段階的な開発アプローチにより、リスクを最小化しながら機能を拡張し、将来的なスケーラビリティと保守性を確保しています。 