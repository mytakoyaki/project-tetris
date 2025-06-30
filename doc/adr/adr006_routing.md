# ADR006: ルーティング方式にNext.js App Routerを採用する

- **日付**: 2025-06-25
- **ステータス**: Accepted

---

## コンテキスト

ClaudeTetrisプロジェクト（React+Next.js, TypeScript, Redux, MUI）において、ページ遷移や画面構成を管理するルーティング方式を決定する必要があった。

Next.js 13以降では、従来のPages Router（src/pages/）に加え、App Router（src/app/）が導入され、柔軟なレイアウト管理やサーバーコンポーネント、ディレクトリ単位のUI管理など新機能が豊富に提供されている。

## 決定内容

- ClaudeTetrisのルーティング方式は**Next.js App Router（src/app/）**を採用する。
- 各ページは`src/app/`配下のディレクトリ・`page.tsx`で管理し、共通レイアウトは`layout.tsx`でディレクトリ単位に設置する。
- 動的ルーティングやローディングUI、エラーUIもApp Routerの標準機能を活用する。
- 今後のNext.jsの主流機能・長期運用を見据え、App Routerを前提とした設計・実装を行う。

## 影響・結果

- 柔軟なレイアウト管理・サーバーコンポーネント・新機能の活用が可能
- ページ・レイアウト・ローディング・エラーUIをディレクトリ単位で整理でき、保守性・拡張性が向上
- 今後のNext.jsアップデートにも追従しやすい構成となる

## 代替案

- **Pages Router（src/pages/）**: シンプルだが、柔軟なレイアウトや新機能の活用、長期運用の観点でApp Routerを優先

## ステータス

このADRは**Accepted**とする。今後のルーティング設計・実装はApp Routerを前提とする。 