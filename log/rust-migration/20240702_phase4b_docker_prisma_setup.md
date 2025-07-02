# Phase 4B: Docker・Prismaセットアップ作業ログ

## 1. Dockerインストール確認
- `docker --version`
- `docker compose version`

## 2. Docker環境起動
- `docker compose up -d`
  - PostgreSQL, Redis, pgAdminのコンテナを起動
  - PostgreSQL/Redisは正常起動、pgAdminは再起動ループ（後で要調査）

## 3. Prismaスキーマ適用
- `.env`ファイル作成
  - DATABASE_URL, REDIS_URL, JWT_SECRET, NEXTAUTH_URL, NEXTAUTH_SECRET, NODE_ENV
- `npx prisma generate`
- `npx prisma db push`
  - スキーマがDBに適用され、テーブルが作成された

## 4. 補足
- 初期化SQLでテーブル未作成エラーが出たが、Prismaスキーマ適用後は問題なし
- 今後はseed投入・API/フロントからのDBアクセス検証へ進む 