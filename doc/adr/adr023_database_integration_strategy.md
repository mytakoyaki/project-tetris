# ADR023: データベース統合戦略

## ステータス
承認

## 日付
2024-12-25

## コンテキスト
ClaudeTetrisプロジェクトのPhase 4B（データベース統合）において、本格的なデータベースシステムの導入が必要となった。現在はモックデータを使用しているが、ユーザー管理、スコア記録、実績管理、ランキング機能等の永続化と、将来的なスケーラビリティを考慮したデータベース戦略が求められている。

## 決定内容
- **ORMとしてPrismaを採用する**
- **データベースとしてPostgreSQLを採用する**
- **本番環境としてAWS RDSを採用する**
- **開発環境としてローカルPostgreSQL + Dockerを採用する**

## 理由

### Prisma選択の理由
1. **型安全性**: TypeScriptとの完全統合、コンパイル時型チェック
2. **開発者体験**: 直感的なスキーマ定義、自動生成されるクライアント
3. **マイグレーション**: 安全なスキーマ変更、バージョン管理
4. **パフォーマンス**: 最適化されたクエリ、N+1問題の解決
5. **エコシステム**: Next.jsとの統合、豊富なドキュメント

### PostgreSQL選択の理由
1. **信頼性**: ACID準拠、データ整合性保証
2. **パフォーマンス**: 複雑なクエリ、大量データ処理に優れる
3. **機能豊富**: JSONサポート、全文検索、地理情報処理
4. **スケーラビリティ**: 水平・垂直スケーリング対応
5. **コスト効率**: オープンソース、運用コスト低

### AWS RDS選択の理由
1. **マネージドサービス**: 運用負荷軽減、自動バックアップ
2. **スケーラビリティ**: 自動スケーリング、高可用性
3. **セキュリティ**: VPC、暗号化、IAM統合
4. **コスト効率**: 従量課金、リザーブドインスタンス
5. **統合性**: 他のAWSサービスとの統合が容易

## 代替案の検討

### Prisma + MySQL
- **メリット**: 軽量、学習コスト低、レガシーシステムとの互換性
- **デメリット**: 機能制限、大規模データでの性能劣化

### Prisma + SQLite
- **メリット**: 軽量、設定不要、開発環境に適している
- **デメリット**: 本番環境での性能・スケーラビリティ不足

### TypeORM + PostgreSQL
- **メリット**: 豊富な機能、柔軟性
- **デメリット**: 学習コスト高、TypeScript統合が不完全

### MongoDB + Mongoose
- **メリット**: スキーマレス、JSONデータに適している
- **デメリット**: トランザクション制限、リレーション管理複雑

## 実装計画

### Phase 4B: データベース基盤構築
- [ ] Prisma設定・スキーマ定義
- [ ] ローカル開発環境構築（Docker）
- [ ] マイグレーション実行
- [ ] 基本的なCRUD操作実装

### Phase 4C: API統合
- [ ] 既存API Routesのデータベース統合
- [ ] エラーハンドリング強化
- [ ] バリデーション実装
- [ ] パフォーマンス最適化

### Phase 4D: 本番環境準備
- [ ] AWS RDS設定
- [ ] 環境変数管理
- [ ] バックアップ戦略
- [ ] 監視体制構築

## 技術的詳細

### Prismaスキーマ定義
```prisma
// prisma/schema.prisma
generator client {
  provider = "prisma-client-js"
  output   = "../src/generated/prisma"
}

datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}

model User {
  id          String   @id @default(uuid())
  username    String   @unique
  email       String   @unique
  password    String
  createdAt   DateTime @default(now())
  lastLoginAt DateTime @updatedAt
  scores      Score[]
  achievements UserAchievement[]
  rating      Int      @default(1000)
  gameStats   GameStats?
}

model Score {
  id        String   @id @default(uuid())
  user      User     @relation(fields: [userId], references: [id])
  userId    String
  score     Int
  level     Int
  lines     Int
  time      Int      // seconds
  gameMode  String
  createdAt DateTime @default(now())
  achievements String[]
  
  @@index([userId, gameMode])
  @@index([score, gameMode])
}

model GameStats {
  id              String @id @default(uuid())
  user            User   @relation(fields: [userId], references: [id])
  userId          String @unique
  totalGames      Int    @default(0)
  totalScore      Int    @default(0)
  highestScore    Int    @default(0)
  totalLines      Int    @default(0)
  totalTime       Int    @default(0) // seconds
  lastUpdated     DateTime @default(now())
}

model Achievement {
  id          String   @id @default(uuid())
  name        String
  description String
  category    String
  maxProgress Int     @default(100)
  userAchievements UserAchievement[]
  
  @@index([category])
}

model UserAchievement {
  id             String   @id @default(uuid())
  user           User     @relation(fields: [userId], references: [id])
  userId         String
  achievement    Achievement @relation(fields: [achievementId], references: [id])
  achievementId  String
  progress       Int      @default(0)
  unlockedAt     DateTime?
  
  @@unique([userId, achievementId])
  @@index([userId, unlockedAt])
}
```

### API統合例
```typescript
// lib/prisma.ts
import { PrismaClient } from '../generated/prisma';

const globalForPrisma = globalThis as unknown as {
  prisma: PrismaClient | undefined;
};

export const prisma = globalForPrisma.prisma ?? new PrismaClient();

if (process.env.NODE_ENV !== 'production') globalForPrisma.prisma = prisma;

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

## パフォーマンス最適化

### インデックス戦略
- ユーザーID + ゲームモードの複合インデックス
- スコア + ゲームモードの複合インデックス
- 作成日時のインデックス

### クエリ最適化
- Prismaのselect/includeによる必要なフィールドのみ取得
- ページネーション実装
- バッチ処理による一括更新

### キャッシュ戦略
- Redisとの統合
- ランキングデータのキャッシュ
- ユーザー統計のキャッシュ

## セキュリティ対策

### データ保護
- パスワードハッシュ化（bcrypt）
- 個人情報の暗号化
- アクセスログ記録

### アクセス制御
- 行レベルセキュリティ（RLS）
- ユーザー権限管理
- API認証・認可

### バックアップ・復旧
- 自動バックアップ（日次）
- ポイントインタイム復旧
- 災害復旧計画

## コスト見積もり

### AWS RDS料金（月額）
- **開発環境**: db.t3.micro $15-20
- **本番環境**: db.t3.small $30-40
- **ストレージ**: $0.115/GB/月
- **バックアップ**: ストレージ料金の20%

### 開発コスト
- **データベース設計**: 1週間
- **API統合**: 2週間
- **テスト・最適化**: 1週間
- **本番移行**: 1週間

## リスクと対策

### データ損失
- **リスク**: ハードウェア障害、人為的ミス
- **対策**: 自動バックアップ、ポイントインタイム復旧

### パフォーマンス劣化
- **リスク**: データ増加によるクエリ遅延
- **対策**: インデックス最適化、クエリ監視

### セキュリティリスク
- **リスク**: データ漏洩、不正アクセス
- **対策**: 暗号化、アクセス制御、監査ログ

## 今後の課題
- データベース設計の詳細化
- マイグレーション戦略の策定
- パフォーマンス監視の設定
- バックアップ・復旧テストの実施
- セキュリティ監査の実施 