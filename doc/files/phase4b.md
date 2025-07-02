# Phase 4B 設定ファイルまとめ

## 1. .env
```env
DATABASE_URL="postgresql://tetris_user:tetris_password@localhost:5432/tetris_dev"
JWT_SECRET="your-super-secret-jwt-key-change-this-in-production"
REDIS_URL="redis://localhost:6379"
NEXTAUTH_URL="http://localhost:3000"
NEXTAUTH_SECRET="your-nextauth-secret-key-change-this-in-production"
NODE_ENV="development"
```

## 2. docker-compose.yml（抜粋）
```yaml
services:
  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_DB: tetris_dev
      POSTGRES_USER: tetris_user
      POSTGRES_PASSWORD: tetris_password
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./prisma/init.sql:/docker-entrypoint-initdb.d/init.sql
  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
  pgadmin:
    image: dpage/pgadmin4:latest
    environment:
      PGADMIN_DEFAULT_EMAIL: admin@tetris.local
      PGADMIN_DEFAULT_PASSWORD: admin
    ports:
      - "5050:80"
    volumes:
      - pgadmin_data:/var/lib/pgadmin
volumes:
  postgres_data:
  redis_data:
  pgadmin_data:
```

## 3. Prismaスキーマ（schema.prisma抜粋）
```prisma
model User {
  id        String   @id @default(uuid())
  username  String   @unique
  email     String   @unique
  ...
}
model Score { ... }
model Achievement { ... }
model UserAchievement { ... }
model Ranking { ... }
model Session { ... }
model GameRoom { ... }
```

## 4. Prismaコマンド
- `npx prisma generate` : Prismaクライアント生成
- `npx prisma db push` : スキーマをDBに適用 