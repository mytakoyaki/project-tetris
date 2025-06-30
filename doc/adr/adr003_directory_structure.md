# ADR003: ディレクトリ構成に機能単位分割型（Feature-based Structure）を採用する

- **日付**: 2025-06-25
- **ステータス**: Accepted

---

## コンテキスト

ClaudeTetrisプロジェクトをSPA化（React+Next.js）し、今後の大規模開発やチーム開発、機能追加・保守性を考慮したディレクトリ構成を決定する必要があった。

近年の大規模React/Next.js開発では、機能ごとにディレクトリを分割する「Feature-based Structure（機能単位分割型）」が主流となっている。

## 決定内容

- ClaudeTetrisのディレクトリ構成は**機能単位分割型（Feature-based Structure）**を採用する。
- 各主要機能（例：game, tutorial, ranking, achievement, user等）ごとに`src/features/`配下にディレクトリを作成し、ロジック・UI・hooks・型定義等をまとめる。
- 汎用UI部品は`src/components/`、グローバル状態管理は`src/store/`、ルーティングは`src/pages/`（または`src/app/`）で管理する。
- assets, hooks, utils, styles等も用途ごとに分離する。

### 例:
```
src/
├── features/
│   ├── game/
│   ├── tutorial/
│   ├── ranking/
│   └── ...
├── components/
├── pages/（またはapp/）
├── store/
├── hooks/
├── utils/
├── assets/
└── styles/
```

## 影響・結果

- 機能ごとに責任範囲が明確になり、スケールしやすい構成となる。
- チーム開発や機能追加・保守が容易になる。
- 機能単位でのテストやリファクタリングも効率化。

## 代替案

- **従来型（components/pages分離型）**: 小規模には有効だが、機能横断的な開発や大規模化には不向き。
- **ドメイン駆動型**: 業務システム等では有効だが、ゲーム系ではfeature-basedが一般的。
- **Atomic Design型**: UI設計重視の場合は有効だが、今回は機能単位の整理を優先。

## ステータス

このADRは**Accepted**とする。今後のディレクトリ設計は機能単位分割型を前提とする。 