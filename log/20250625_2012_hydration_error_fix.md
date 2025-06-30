# Material UI Hydration エラー修正

**日時**: 2025-06-25 20:12  
**作業内容**: Next.js App Router + Material UI での Hydration エラー解決

---

## 🐛 発生していた問題

### エラー内容
```
Error: Hydration failed because the server rendered HTML didn't match the client.
```

### 原因分析
- **Material UI + Next.js App Router** の組み合わせで発生する典型的な問題
- **Emotion CSS-in-JS** のサーバーサイドとクライアントサイドでのスタイル生成の差異
- **動的スタイルクラス名** がSSRとCSRで一致しない状態

### 影響範囲
- GameContainer コンポーネントの初期レンダリング時にエラー
- 開発体験の悪化（console エラー表示）
- 本番環境での潜在的なパフォーマンス問題

---

## 🔧 実装した修正

### 1. @mui/material-nextjs パッケージ追加
```bash
npm install @mui/material-nextjs
```

**目的**: Next.js App Router 専用の Material UI 統合パッケージ

### 2. theme-registry.tsx 作成
**ファイル**: `src/app/theme-registry.tsx`

```typescript
'use client'

import { ThemeProvider } from '@mui/material/styles'
import CssBaseline from '@mui/material/CssBaseline'
import { createTheme } from '@mui/material/styles'
import { AppRouterCacheProvider } from '@mui/material-nextjs/v15-appRouter'

const theme = createTheme({
  palette: {
    mode: 'dark',
    primary: { main: '#00ff88' },
    secondary: { main: '#ffd700' },
    background: {
      default: '#1a1a1a',
      paper: '#2d2d2d',
    },
  },
  typography: {
    fontFamily: '"Segoe UI", "Roboto", "Oxygen", "Ubuntu", "Cantarell", sans-serif',
  },
})

export default function ThemeRegistry({ children }: { children: React.ReactNode }) {
  return (
    <AppRouterCacheProvider>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        {children}
      </ThemeProvider>
    </AppRouterCacheProvider>
  )
}
```

### 3. providers.tsx 更新
**変更前**: 独自の ThemeProvider + createCache 実装
**変更後**: AppRouterCacheProvider を使用した統合

```typescript
'use client'

import { Provider } from 'react-redux'
import { store } from '@/store/store'
import ThemeRegistry from './theme-registry'

export function Providers({ children }: { children: React.ReactNode }) {
  return (
    <Provider store={store}>
      <ThemeRegistry>
        {children}
      </ThemeRegistry>
    </Provider>
  )
}
```

---

## 🎯 技術的解決ポイント

### 1. AppRouterCacheProvider の役割
- **Emotion キャッシュ** の適切な管理
- **SSR/CSR 間でのスタイル一貫性** 保証
- **Next.js App Router** との最適化された統合

### 2. クライアントコンポーネント化
- `'use client'` ディレクティブによる明示的なクライアント実行
- **サーバーサイドでのスタイル計算回避**

### 3. CSSBaseline 統合
- Material UI の **グローバルCSS リセット**
- **一貫したベーススタイル** の適用

---

## ✅ 修正結果

### 開発サーバー状況
```bash
npm run dev
✓ Ready in 589ms
- Local: http://localhost:3000
```

### エラー解消確認
- ✅ Hydration エラー完全解消
- ✅ コンソールエラーなし
- ✅ UI コンポーネント正常表示
- ✅ レイアウト切り替え機能正常動作

### パフォーマンス
- **初期レンダリング** の安定化
- **スタイル適用** の一貫性向上
- **開発体験** の大幅改善

---

## 📦 依存関係の変更

### 追加パッケージ
```json
{
  "@mui/material-nextjs": "^6.1.8"
}
```

### 削除/変更ファイル
- `src/components/mui-ssr-provider.tsx` (削除)
- `src/app/theme-registry.tsx` (新規作成)
- `src/app/providers.tsx` (更新)

---

## 🔍 学習ポイント

### Next.js App Router + Material UI ベストプラクティス
1. **AppRouterCacheProvider** の必須使用
2. **'use client'** による適切なバウンダリー設定
3. **公式パッケージ** (@mui/material-nextjs) の活用

### Hydration エラー対策
- **サーバー/クライアント間の状態一致** が重要
- **動的スタイル生成** の適切な管理
- **CSS-in-JS** ライブラリの統合時の注意点

---

## 🚀 今後の開発への影響

### ポジティブな効果
- **安定した開発環境** の確立
- **Material UI コンポーネント** の安全な使用
- **プロダクション環境** での信頼性向上

### 開発継続準備
- **ゲームロジック実装** への障壁除去
- **追加 UI コンポーネント** の安心な開発
- **パフォーマンス最適化** の基盤確立

---

**修正完了**: Next.js + Material UI 環境の安定化が完了し、本格的なゲーム機能実装への準備が整いました。