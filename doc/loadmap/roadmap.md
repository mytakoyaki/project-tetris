# ClaudeTetris再構成ロードマップ

---

## 1. 推奨ディレクトリ構造

```
tetris/
├── src/
│   ├── app/                # Next.js App Router（ページ・レイアウト・APIルート）
│   │   ├── page.tsx        # ルートページ
│   │   ├── layout.tsx      # ルートレイアウト
│   │   ├── api/            # API Route（RESTfulエンドポイント）
│   │   │   ├── users/route.ts
│   │   │   ├── scores/route.ts
│   │   │   ├── achievements/route.ts
│   │   │   └── ranking/route.ts
│   │   └── ...             # その他ページ・ルート
│   ├── features/           # 機能単位のディレクトリ（Feature-based Structure）
│   │   ├── game/           # ゲームロジック・UI
│   │   │   ├── components/
│   │   │   ├── hooks/
│   │   │   └── utils/
│   │   ├── user/           # ユーザー管理
│   │   ├── score/          # スコア管理
│   │   ├── achievement/    # 実績管理
│   │   └── ranking/        # ランキング管理
│   ├── shared/             # 共通ロジック・ユーティリティ
│   ├── types/              # 型定義（TypeScript型・API型等）
│   ├── store/              # Reduxストア・スライス
│   └── styles/             # グローバル/共通スタイル
├── prisma/                 # Prisma等ORM用のスキーマ・マイグレーション
├── public/                 # 静的ファイル
├── tests/                  # テストコード（unit, integration, e2e）
├── .env                    # 環境変数
├── package.json
├── README.md
└── ...（CI/CD, 設定ファイル等）
```

---

## 2. 開発ロードマップ

### STEP 1. プロジェクト初期化・基盤整備
- Next.js＋TypeScript新規プロジェクト作成
- MUI, Redux Toolkit, SWR/React Query, Prisma等の導入
- ディレクトリ構成（上記構造）を初期化
- Lint/Prettier/型チェック等の開発環境整備

### STEP 2. コア機能のSPA化・UI/ロジック移植
- ゲームロジック・UIをReactコンポーネント化（`features/game/`）
- ページ・レイアウト設計（`src/app/`）
- Reduxストア設計・状態管理実装（`store/`）

### STEP 3. API層・DB設計・バックエンド実装
- API Route（`src/app/api/`）でRESTfulエンドポイント実装
- Prisma等でRDBMSスキーマ設計・マイグレーション
- JWT認証API実装（`api/auth/`等）

### STEP 4. クライアント側データ取得・型安全化
- SWR/React QueryでAPIデータ取得・キャッシュ管理
- 型定義（`types/`）の整備、API通信の型安全化

### STEP 5. テスト・CI/CD体制構築
- ユニット/統合テスト（`tests/`、Jest＋React Testing Library）
- E2Eテスト（Playwright/Cypress）
- CI/CDパイプライン構築

### STEP 6. セキュリティ・運用設計
- 認証・認可・バリデーション・セキュリティ対策
- クラウドDB運用設計・バックアップ

### STEP 7. ドキュメント・リリース準備
- API仕様書・設計書・README等の整備
- 本番環境デプロイ

### STEP 8. 拡張・運用・改善
- OAuth2/Social Login導入
- PWA/モバイル対応、i18n、アクセシビリティ
- パフォーマンス最適化、監視、運用改善 

現在の実装をアーキテクチャ設計書と比較して確認します。

  アーキテクチャ遵守状況の分析

  ✅ 完全に遵守している項目

  1. 技術スタック
    - React (Next.js App Router) ✅
    - TypeScript ✅
    - Redux Toolkit ✅
    - Material UI (MUI) ✅
  2. ディレクトリ構成
    - src/配下の機能単位分割型（Feature-based Structure）✅
    - 型定義をsrc/types/に集約 ✅

  ⚠️ 部分的遵守・未実装の項目

  1. 状態管理
    - Redux Toolkit使用 ✅
    - しかし、アーキテクチャ設計書では言及されていないが、現在はサー
  バーサイド状態管理（SWR/React Query）は未実装
  2. API/バックエンド
    - Next.js API Route 未実装 ❌
    - 現在はクライアントサイドのみの実装
    - RESTful API設計 未実装 ❌
  3. データベース
    - 設計書ではRDBMS（PostgreSQL/MySQL）を想定
    - 現在はlocalStorageのみ使用 ❌
  4. 認証・認可
    - JWT認証 未実装 ❌
    - 現在は認証機能なし

  🔍 現在の実装における主要な特徴

  現在実装されている機能：
  - Redux Toolkitによる完全なクライアントサイド状態管理
  - Material UIベースのコンポーネント設計
  - TypeScriptによる型安全性
  - 機能別ディレクトリ構成（features/game/, features/achievement/
  など）
  - localStorageによるデータ永続化

  アーキテクチャ設計書との主な相違点：
  1. フルスタック → クライアントサイドのみ
    - 設計書はNext.js API Route + DBを想定
    - 現在はクライアントサイドのゲームとして実装
  2. 認証機能
    - 設計書ではJWT認証を想定
    - 現在は単体ゲームとして認証なし
  3. データ永続化
    - 設計書ではRDBMS + クラウドDB
    - 現在はlocalStorageのみ

  結論

  現在の実装は、クライアントサイド部分においてはアーキテクチャ設計書
  を適切に遵守しています。

  ただし、設計書が想定するフルスタックアプリケーションの要素（API、DB
  、認証）は未実装です。これは以下の理由によるものと考えられます：

  1. 段階的開発: まずコアゲーム機能をクライアントサイドで完成させる
  2. MVP（Minimum Viable Product）:
  基本的なテトリスゲームとして機能する
  3. 将来拡張性: 設計書に従ってサーバーサイド機能を後から追加可能

  現在の実装は設計書の方針に沿っており、将来的にサーバーサイド機能を
  追加する際も、既存のクライアントサイドアーキテクチャを活用できる構
  造になっています。
