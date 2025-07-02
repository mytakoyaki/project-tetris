# 🔄 ClaudeTetris マイグレーション計画

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

### **Phase 4: フルスタック機能実装（進行中）**
- ✅ **Next.js API Routes実装**
- ✅ **JWT認証システム**
- ✅ **型安全なAPI通信**
- 🔄 **データベース統合（Phase 4B）**
- ⏳ **Clerk認証移行（Phase 5）**

### **Phase 5: 認証・UI改善（予定）**
- ⏳ **Clerk認証システム導入**
- ⏳ **SNSログイン統合**
- ⏳ **認証UI改善**
- ⏳ **段階的JWT移行**

### **Phase 6: リアルタイム機能（予定）**
- ⏳ **Socket.io統合**
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

### **Phase 4B: データベース統合（現在進行中）**

#### **Step 1: Prisma設定・スキーマ定義**
```bash
# 依存関係インストール
npm install @prisma/client
npm install -D prisma

# Prisma初期化
npx prisma init

# スキーマ定義
# prisma/schema.prisma
```

#### **Step 2: ローカル開発環境構築**
```bash
# Docker Compose設定
# docker-compose.yml
version: '3.8'
services:
  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: tetris_dev
      POSTGRES_USER: tetris_user
      POSTGRES_PASSWORD: tetris_password
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data

volumes:
  postgres_data:
  redis_data:
```

#### **Step 3: API統合**
```typescript
// lib/prisma.ts
import { PrismaClient } from '../generated/prisma';

const globalForPrisma = globalThis as unknown as {
  prisma: PrismaClient | undefined;
};

export const prisma = globalForPrisma.prisma ?? new PrismaClient();

if (process.env.NODE_ENV !== 'production') globalForPrisma.prisma = prisma;
```

#### **Step 4: 既存API Routes更新**
```typescript
// app/api/scores/route.ts
import { prisma } from '@/lib/prisma';

export async function GET(request: NextRequest) {
  try {
    const { searchParams } = new URL(request.url);
    const userId = searchParams.get('userId');
    const gameMode = searchParams.get('gameMode');
    const limit = parseInt(searchParams.get('limit') || '10');
    const offset = parseInt(searchParams.get('offset') || '0');

    const where = {
      ...(userId && { userId }),
      ...(gameMode && { gameMode }),
    };

    const [scores, total] = await Promise.all([
      prisma.score.findMany({
        where,
        orderBy: { score: 'desc' },
        take: limit,
        skip: offset,
        include: {
          user: {
            select: { username: true }
          }
        }
      }),
      prisma.score.count({ where })
    ]);

    return NextResponse.json({
      scores,
      total,
      hasMore: offset + limit < total
    });
  } catch (error) {
    return NextResponse.json(
      { error: 'Failed to fetch scores' },
      { status: 500 }
    );
  }
}
```

### **Phase 4C: RTK Query統合**

#### **Step 1: RTK Query設定**
```typescript
// store/api.ts
import { createApi, fetchBaseQuery } from '@reduxjs/toolkit/query/react';

export const api = createApi({
  reducerPath: 'api',
  baseQuery: fetchBaseQuery({ 
    baseUrl: '/api',
    credentials: 'include',
  }),
  tagTypes: ['User', 'Score', 'Achievement', 'Ranking'],
  endpoints: (builder) => ({
    getScores: builder.query({
      query: (params) => ({
        url: '/scores',
        params,
      }),
      providesTags: ['Score'],
    }),
    submitScore: builder.mutation({
      query: (score) => ({
        url: '/scores',
        method: 'POST',
        body: score,
      }),
      invalidatesTags: ['Score', 'Ranking'],
    }),
  }),
});
```

#### **Step 2: ストア統合**
```typescript
// store/store.ts
import { configureStore } from '@reduxjs/toolkit';
import { setupListeners } from '@reduxjs/toolkit/query';
import { api } from './api';

export const store = configureStore({
  reducer: {
    [api.reducerPath]: api.reducer,
    game: gameReducer,
    user: userReducer,
    score: scoreReducer,
    achievement: achievementReducer,
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware().concat(api.middleware),
});

setupListeners(store.dispatch);
```

### **Phase 4D: 本番環境準備**

#### **Step 1: AWS RDS設定**
- PostgreSQLインスタンス作成
- セキュリティグループ設定
- バックアップ設定

#### **Step 2: Upstash Redis設定**
- Redisインスタンス作成
- 接続設定
- 監視設定

#### **Step 3: 環境変数管理**
```bash
# .env.local
DATABASE_URL="postgresql://username:password@localhost:5432/tetris_dev"
REDIS_URL="redis://localhost:6379"

# .env.production
DATABASE_URL="postgresql://username:password@aws-rds-endpoint:5432/tetris_prod"
REDIS_URL="redis://upstash-redis-endpoint:6379"
```

---

## 🎯 **技術スタック概要**

### **フロントエンド**
- **フレームワーク**: Next.js 15 (App Router)
- **UI**: React 19 + Material-UI (MUI)
- **状態管理**: Redux Toolkit + RTK Query
- **言語**: TypeScript
- **スタイリング**: Tailwind CSS + Emotion

### **バックエンド**
- **API**: Next.js API Routes
- **ORM**: Prisma
- **データベース**: PostgreSQL (AWS RDS)
- **キャッシュ**: Redis (Upstash)
- **認証**: Clerk (Phase 5)
- **リアルタイム通信**: Socket.io (Phase 6)

### **ゲームエンジン**
- **言語**: Rust
- **実行環境**: WebAssembly (WASM)
- **機能**: SRS回転システム、スピン検出、衝突判定

### **インフラ・デプロイ**
- **ホスティング**: Vercel
- **データベース**: AWS RDS
- **キャッシュ**: Upstash Redis
- **認証**: Clerk
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
- [x] Next.js API Routes実装
- [x] JWT認証システム
- [x] 型安全なAPI通信

### **進行中（🔄）**
- [ ] Prisma設定・スキーマ定義
- [ ] ローカル開発環境構築（Docker）
- [ ] API統合（データベース）
- [ ] RTK Query統合

### **未着手（⏳）**
- [ ] 本番環境準備（AWS RDS + Upstash Redis）
- [ ] Clerk認証システム導入
- [ ] SNSログイン統合
- [ ] Socket.io統合
- [ ] オンラインマルチプレイヤー機能
- [ ] マッチングシステム
- [ ] チャット機能
- [ ] リプレイシステム
- [ ] トーナメント機能
- [ ] モバイル対応
- [ ] パフォーマンス最適化

---

## 🚀 **次のステップ**

### **Phase 4B: データベース統合（現在進行中）**
1. **Prisma設定・スキーマ定義**
   - Prisma初期化
   - データベーススキーマ設計
   - マイグレーション作成

2. **ローカル開発環境構築**
   - Docker Compose設定
   - PostgreSQL + Redis起動
   - 接続確認

3. **API統合**
   - 既存API Routes更新
   - データベース接続
   - エラーハンドリング

4. **RTK Query統合**
   - API定義作成
   - キャッシュ戦略設定
   - フロントエンド統合

### **Phase 5: Clerk認証移行（予定）**
1. **Clerkプロジェクト設定**
2. **認証UIコンポーネント実装**
3. **SNSログイン統合**
4. **段階的JWT移行**

### **Phase 6: リアルタイム機能（予定）**
1. **Socket.io統合**
2. **ゲームルーム機能**
3. **マッチングシステム**
4. **チャット機能**

---

## 🛠️ **開発環境セットアップ**

### **前提条件**
```bash
# Node.js 20以上
node --version

# Docker & Docker Compose
docker --version
docker-compose --version

# Rust (WASM用)
rustc --version
wasm-pack --version
```

### **データベース環境**
```bash
# PostgreSQL + Redis 起動
cd modern-tetris
docker-compose up -d

# データベース接続確認
psql -h localhost -U tetris_user -d tetris_dev
redis-cli ping
```

### **Prisma設定**
```bash
# 依存関係インストール
npm install @prisma/client
npm install -D prisma

# Prisma初期化
npx prisma init

# スキーマ生成
npx prisma generate

# マイグレーション実行
npx prisma migrate dev --name init

# Prisma Studio起動
npx prisma studio
```

### **Next.js環境**
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

### **Rust WASM環境**
```bash
# Rustゲームエンジンビルド
cd rust-game-engine
./build.sh

# WASMファイル確認
ls -la public/wasm/
```

## 🔧 **環境変数設定**

### **開発環境 (.env.local)**
```bash
# データベース
DATABASE_URL="postgresql://tetris_user:tetris_password@localhost:5432/tetris_dev"
REDIS_URL="redis://localhost:6379"

# 認証
JWT_SECRET="your-secret-key"
NEXTAUTH_SECRET="your-nextauth-secret"

# Clerk (Phase 5で使用)
NEXT_PUBLIC_CLERK_PUBLISHABLE_KEY="your-clerk-key"
CLERK_SECRET_KEY="your-clerk-secret"
```

### **本番環境 (.env.production)**
```bash
# AWS RDS
DATABASE_URL="postgresql://username:password@aws-rds-endpoint:5432/tetris_prod"

# Upstash Redis
REDIS_URL="redis://upstash-redis-endpoint:6379"

# Clerk
NEXT_PUBLIC_CLERK_PUBLISHABLE_KEY="your-clerk-key"
CLERK_SECRET_KEY="your-clerk-secret"
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