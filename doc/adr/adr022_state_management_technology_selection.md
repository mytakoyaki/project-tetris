# ADR022: 状態管理技術の選定

## ステータス
承認

## 日付
2024-12-25

## コンテキスト
ClaudeTetrisプロジェクトにおいて、フロントエンドの状態管理技術の選定が必要となった。ゲーム状態、ユーザー情報、スコア、実績、リアルタイム通信状態等の複雑な状態管理と、API通信の効率化を考慮した技術選択が求められている。

## 決定内容
- **状態管理ライブラリとしてRedux Toolkitを採用する**
- **API通信ライブラリとしてRTK Queryを採用する**
- **既存のRedux実装を拡張・最適化する**

## 理由

### Redux Toolkit選択の理由
1. **開発効率**: ボイラープレート削減、DevTools統合、TypeScriptサポート
2. **パフォーマンス**: 自動的なメモ化、最適化されたレンダリング
3. **予測可能性**: 単方向データフロー、デバッグ容易性
4. **エコシステム**: 豊富なミドルウェア、ツール、コミュニティ
5. **学習コスト**: 既に実装済み、チームの習熟度が高い

### RTK Query選択の理由
1. **API統合**: 自動的なキャッシュ、バックグラウンド更新、同期
2. **パフォーマンス**: 重複リクエスト防止、最適化されたレンダリング
3. **開発者体験**: TypeScriptサポート、自動生成されるフック
4. **リアルタイム対応**: リアルタイムデータ更新との統合が容易
5. **エラーハンドリング**: 統一されたエラー処理、ローディング状態管理

## 代替案の検討

### Zustand
- **メリット**: 軽量、シンプル、TypeScriptサポート優秀
- **デメリット**: 大規模アプリでの複雑性、エコシステム不足

### Recoil
- **メリット**: React統合、細粒度の状態管理
- **デメリット**: 実験的、学習コスト高、エコシステム未成熟

### SWR
- **メリット**: 軽量、React Hooks統合、キャッシュ機能
- **デメリット**: 状態管理機能制限、大規模アプリでの複雑性

### React Query (TanStack Query)
- **メリット**: 豊富な機能、優秀なキャッシュ戦略
- **デメリット**: 状態管理機能制限、Reduxとの統合複雑

## 実装計画

### Phase 4B: RTK Query統合
- [ ] RTK Query設定
- [ ] API スライス実装（ユーザー、スコア、実績、ランキング）
- [ ] キャッシュ戦略設定
- [ ] エラーハンドリング統一

### Phase 5: 認証状態管理
- [ ] Clerk認証状態のRedux統合
- [ ] 認証フロー管理
- [ ] セッション状態管理
- [ ] 権限ベースのUI制御

### Phase 6: リアルタイム状態管理
- [ ] Socket.io状態のRedux統合
- [ ] リアルタイムゲーム状態管理
- [ ] マッチング状態管理
- [ ] チャット状態管理

## 技術的詳細

### ストア構成
```typescript
// store/store.ts
import { configureStore } from '@reduxjs/toolkit';
import { setupListeners } from '@reduxjs/toolkit/query';
import { api } from './api';
import gameReducer from './slices/gameSlice';
import userReducer from './slices/userSlice';
import scoreReducer from './slices/scoreSlice';
import achievementReducer from './slices/achievementSlice';

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

### RTK Query API定義
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

export const { useGetScoresQuery, useSubmitScoreMutation } = api;
```

### リアルタイム状態管理
```typescript
// store/slices/gameSlice.ts
import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface GameState {
  isOnline: boolean;
  currentRoom: string | null;
  players: Player[];
  gameStatus: 'waiting' | 'playing' | 'finished';
}

const initialState: GameState = {
  isOnline: false,
  currentRoom: null,
  players: [],
  gameStatus: 'waiting',
};

const gameSlice = createSlice({
  name: 'game',
  initialState,
  reducers: {
    setOnlineStatus: (state, action: PayloadAction<boolean>) => {
      state.isOnline = action.payload;
    },
    joinRoom: (state, action: PayloadAction<string>) => {
      state.currentRoom = action.payload;
    },
    updatePlayers: (state, action: PayloadAction<Player[]>) => {
      state.players = action.payload;
    },
  },
});
```

## パフォーマンス最適化

### メモ化戦略
- React.memoによるコンポーネント最適化
- useMemo/useCallbackによる計算・関数最適化
- Redux Toolkitの自動メモ化活用

### キャッシュ戦略
- RTK Queryの自動キャッシュ
- カスタムキャッシュTTL設定
- バックグラウンド更新設定

### バンドルサイズ最適化
- コード分割（React.lazy）
- 動的インポート
- 不要な依存関係の削除

## 開発者体験向上

### DevTools統合
- Redux DevTools Extension
- RTK Query DevTools
- タイムトラベルデバッグ

### TypeScript統合
- 完全な型安全性
- 自動生成される型定義
- IntelliSenseサポート

### テスト戦略
- Redux Toolkitのテストユーティリティ
- RTK Queryのモック機能
- 統合テストの自動化

## コスト見積もり

### 開発コスト
- **RTK Query統合**: 1週間
- **認証状態管理**: 1週間
- **リアルタイム統合**: 2週間
- **テスト・最適化**: 1週間

### 運用コスト
- **メンテナンス**: 月1-2日
- **パフォーマンス監視**: 継続的
- **アップデート**: 四半期1回

## リスクと対策

### 複雑性増加
- **リスク**: 状態管理の複雑化
- **対策**: 明確なアーキテクチャ、ドキュメント整備

### パフォーマンス劣化
- **リスク**: 過度なレンダリング
- **対策**: メモ化戦略、パフォーマンス監視

### 学習コスト
- **リスク**: チームの学習負荷
- **対策**: 段階的導入、トレーニング実施

## 今後の課題
- RTK Queryの詳細設計
- リアルタイム状態管理の実装
- パフォーマンス監視の設定
- テスト戦略の策定
- ドキュメント整備 