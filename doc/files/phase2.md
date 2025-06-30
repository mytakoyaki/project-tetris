# Phase 2 ファイル仕様・設計方針

## 概要
- Rust製ゲームエンジン（rust-game-engine/）をWebAssembly（WASM）としてビルドし、Next.js（modern-tetris/）と連携する。
- コアなゲームロジック（テトリミノ操作・衝突判定・スコア計算・段位進行・実績判定など）をRust側に移管。
- Rust⇔JS間のデータ受け渡しインターフェース（API）を設計・実装。

---

## ディレクトリ構成（Phase2）

```
project-tetris/
├── rust-game-engine/         # Rust製ゲームエンジン（WASMビルド対応）
│   ├── src/
│   ├── Cargo.toml
│   ├── lib.rs
│   └── ...
├── modern-tetris/           # Next.js + Reactフロントエンド
│   ├── src/
│   ├── public/wasm/         # WASMバイナリ配置先
│   └── ...
└── ...
```

---

## Rust側（rust-game-engine/）

- **wasm-bindgen**でJSから呼び出せるAPIをエクスポート
- `lib.rs`にWASM用のエントリポイントを実装
- Cargo.tomlに`wasm-bindgen`依存追加
- `wasm-pack build --target web`でビルド
- 出力された`pkg/`配下のWASMとJSラッパーをNext.js側で利用

### 主要API例
- `init_field(width: u32, height: u32) -> Field`
- `spawn_tetromino() -> Tetromino`
- `move_tetromino(dir: String) -> Field`
- `rotate_tetromino(dir: String) -> Field`
- `hard_drop() -> Field`
- `get_score() -> u32`
- `get_rank() -> u8`
- `get_achievements() -> JsValue`

### データ構造
- `Field`, `Tetromino`, `Score`, `Rank`, `Achievement` などを`serde`でシリアライズ

---

## Rust⇔TypeScript型対応表・データ受け渡し設計

### Rust構造体例
```rust
#[derive(Serialize, Deserialize)]
pub struct Field {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Tetromino {
    pub kind: String,
    pub x: i32,
    pub y: i32,
    pub rotation: u8,
}
```

### TypeScript型例
```ts
export type Field = {
  width: number;
  height: number;
  cells: number[];
};

export type Tetromino = {
  kind: string;
  x: number;
  y: number;
  rotation: number;
};
```

### Rust→JSデータ受け渡し設計
- Rust側で`serde_wasm_bindgen::to_value`で`JsValue`に変換し、WASM APIで返却
- TypeScript側で`as any as Field`等で型アサーションして受け取る
- 今後は型安全な変換ユーティリティも検討

---

## Next.js側（modern-tetris/）

- `public/wasm/`にWASMバイナリを配置
- `@wasm-tool/wasm-pack-plugin`や`wasm-loader`でWASMを読み込む
- TypeScriptでRust APIのラッパーを作成
- ReduxやReactコンポーネントからRustエンジンAPIを呼び出し

### 連携例
```ts
import init, { init_field, spawn_tetromino } from '../public/wasm/rust_game_engine';

await init();
const field = init_field(10, 20);
const tetromino = spawn_tetromino();
```

### Next.js側でのWASM組み込み・API呼び出し設計例

#### WASMバイナリの配置
- `modern-tetris/public/wasm/`に`rust-game-engine/pkg/`の中身をコピー

#### TypeScriptでの呼び出し例
```ts
// src/wasm/rustGameEngine.ts
import init, { init_field, spawn_tetromino } from '../../public/wasm/rust_game_engine';

export async function initRustEngine() {
  await init();
}

export function createField(width: number, height: number) {
  return init_field(width, height);
}

export function createTetromino() {
  return spawn_tetromino();
}
```

#### Reactコンポーネントからの利用例
```tsx
import { useEffect, useState } from 'react';
import { initRustEngine, createField, createTetromino } from '../wasm/rustGameEngine';

export default function RustTest() {
  const [field, setField] = useState<any>(null);
  const [tetromino, setTetromino] = useState<any>(null);

  useEffect(() => {
    (async () => {
      await initRustEngine();
      setField(createField(10, 20));
      setTetromino(createTetromino());
    })();
  }, []);

  return (
    <div>
      <h2>Rust Field</h2>
      <pre>{JSON.stringify(field, null, 2)}</pre>
      <h2>Rust Tetromino</h2>
      <pre>{JSON.stringify(tetromino, null, 2)}</pre>
    </div>
  );
}
```

---

## Next.js側TypeScriptラッパー設計
- `src/wasm/rustGameEngine.ts`でWASM APIをラップ
- 型定義を付与し、React/Reduxから安全に利用

---

## Redux連携設計例

### Rustエンジン連携用stateの追加
```ts
export interface GameState {
  // ... 既存のstate ...
  rustField: Field | null;
  rustTetromino: Tetromino | null;
}
```

### Rustエンジン連携action/reducer例
```ts
const gameSlice = createSlice({
  name: 'game',
  initialState,
  reducers: {
    setRustField(state, action) {
      state.rustField = action.payload;
    },
    setRustTetromino(state, action) {
      state.rustTetromino = action.payload;
    },
    asyncInitRustGame: {
      reducer(state, action) {
        state.rustField = action.payload.field;
        state.rustTetromino = action.payload.tetromino;
      },
      prepare() {
        const field = createField(10, 20);
        const tetromino = createTetromino();
        return { payload: { field, tetromino } };
      },
    },
  },
});
```

### 今後の拡張方針
- move/rotate/hard_drop等のRust APIもRedux action経由でstateに反映
- Rustエンジンの状態をReduxで一元管理し、UI描画・ゲーム進行を統合

---

## 今後のRust側API拡張方針
- move_tetromino(dir: &str) -> JsValue
- rotate_tetromino(dir: &str) -> JsValue
- hard_drop() -> JsValue
- get_score() -> u32
- get_rank() -> u8
- get_achievements() -> JsValue
- ゲーム進行・判定・スコア計算・実績判定などを段階的にRust側に移管

---

## 今後の拡張
- Rust側APIの段階的拡充（スコア計算、実績判定、リプレイ保存など）
- JS⇔Rust間のパフォーマンス最適化
- WASMバイナリのキャッシュ・バージョン管理

---

## Phase2成果物・実装まとめ

### Rustエンジン側
- テトリミノ種類（I, O, T, S, Z, J, L）をランダム生成
- move/rotate/hard_dropでグローバル状態を更新
- 衝突判定・固定処理・ライン消去の雛形を実装
- フィールド・テトリミノ状態をWASM経由でJSに返却

### Next.js/Redux/UI側
- RustエンジンAPIをTypeScriptラッパーで呼び出し
- Redux stateでRustエンジンの状態を一元管理
- Reactコンポーネントでmove/rotate/hard_dropをボタン操作で実行
- Rustエンジンの状態が即時UIに反映

### 設計・連携ポイント
- Rust⇔TypeScript型対応表・データ受け渡し設計を明確化
- Redux/ReactとRustエンジンの責務分離
- 今後はゲーム進行・スコア計算・実績判定などもRust側に段階的に移管予定

### 今後の展望
- テトリミノ形状ごとの衝突判定・固定処理の本実装
- スコア計算・段位・実績APIの拡張
- UI/Reduxとのさらなる統合・パフォーマンス最適化

---

## Phase3（2024-06-27）

- 公式TTCガイドライン準拠のSRS（Super Rotation System）・スピン判定（T-Spin, S/Z-Spin, I-Spin, J/L-Spin）をRustで厳密に実装
- WASMバインディングを通じてNext.js/React/Redux側から公式判定を利用できる基盤を構築
- 公式条件（3角埋まり＋wallkick必須）を満たすテストがRustで全てパス
- SRS回転失敗時は「現状維持」とする公式仕様も実装・検証済み
- 今後はWASM経由でUI・スコア・エフェクト・実績連携、Phase4以降はAPI/DB/認証などフルスタック化やUI統合へ

--- 