# ClaudeTetris アーキテクチャ設計書

## 1. 全体アーキテクチャ概要
ClaudeTetrisは、現代的なSPA（Single Page Application）アーキテクチャを採用し、React＋Next.jsを基盤としたWebゲームアプリケーションである。フロントエンド・バックエンド一体型の開発体験と、拡張性・保守性・ユーザー体験の最適化を重視する。

---

## 2. 技術スタック・主要構成要素
- **フロントエンド**: React（Next.js App Router）、TypeScript、Redux（Redux Toolkit）、Material UI（MUI）
- **バックエンド/API**: Next.js API Route（Node.js）、TypeScript
- **状態管理**: Redux Toolkit
- **UIフレームワーク**: Material UI（MUI）
- **データベース**: リレーショナルDB（PostgreSQLまたはMySQL系、クラウドDBサービス活用）
- **認証方式**: JWTトークン認証（HttpOnly Cookie保存）、将来的にOAuth2/Social Loginを段階的導入
- **データ取得**: RESTful API＋必要に応じてWebSocket、SWR/React Queryによるキャッシュ・再取得管理
- **テスト**: Jest＋React Testing Library（ユニット/統合）、Playwright/Cypress（E2E）、CI自動化

---

## 3. ディレクトリ構成方針
- **src/配下に機能単位分割型（Feature-based Structure）で整理**
- **APIルートは`src/app/api/`に実装**
- **型定義・共通ロジックは`src/types/`や`src/shared/`等に集約**

---

## 4. API設計方針
- **RESTful APIを基本とし、必要に応じてWebSocket等のリアルタイム通信を併用**
- **Next.js API RouteでサーバーサイドAPIを実装**
- **OpenAPI/Swagger等によるAPIドキュメント自動生成を推奨**
- **認証・認可・バリデーション・エラーハンドリング・バージョニング・セキュリティ対策を重視**

---

## 5. 認証・認可設計方針
- **初期リリースはJWTトークン認証（HttpOnly Cookie保存）を採用**
- **リリース後、OAuth2/Social Login（Google, Twitter, GitHub等）を段階的に導入**
- **認可（管理者/一般ユーザー等の権限管理）も順次設計**

---

## 6. データベース設計方針
- **RDBMS（PostgreSQLまたはMySQL系）をメインDBとして採用**
- **クラウドDBサービス（PlanetScale, Supabase, AWS RDS等）で運用負荷を低減**
- **必要に応じてNoSQL/インメモリDB（例：Redis）を補助的に利用**
- **スキーマ設計・マイグレーション・セキュリティ・バックアップを重視**

---

## 7. テスト・CI/CD戦略
- **Jest＋React Testing Libraryでユニット/統合テスト**
- **Playwright/CypressでE2Eテスト**
- **Lint/型チェック/テストのCI自動化**

---

## 8. 今後の拡張・未決事項
- API/DBの詳細設計・スキーマ設計
- OAuth2/Social Login導入時のユーザー統合・アカウント管理
- PWA/モバイル対応、国際化（i18n）、アクセシビリティ
- パフォーマンス最適化、ロギング、監視、運用設計
- APIドキュメント整備（OpenAPI/Swagger等）

---

## 9. 参考：ADR一覧
- doc/adr/adr001_spa_framework.md ～ adr009_database_strategy.md に各技術選定・設計方針の詳細を記録

---

本設計書は現時点の合意内容を反映しており、今後の要件・技術動向に応じて随時アップデートする。 