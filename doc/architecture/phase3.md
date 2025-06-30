# Phase3: 公式SRS・スピン判定のRust実装・検証

## 概要
- 公式TTCガイドライン準拠のSRS（Super Rotation System）・スピン判定（T-Spin, S/Z-Spin, I-Spin, J/L-Spin）をRustで厳密に実装
- WASMバインディングを通じてNext.js/React/Redux側から公式判定を利用できる基盤を構築

## 公式仕様との対応
- SRS回転処理・キックテーブルはTTC公式通り
- スピン判定は「角埋まり数」「wallkick必須」など公式条件を厳密に再現
- 回転失敗時は現状維持（上にずらす等はしない）

## Rust/WASM実装のポイント
- `attempt_srs_rotation`関数でSRS回転・キックを厳密実装
- `detect_t_spin`等で公式スピン判定条件を再現
- WASMバインディングでJS/TSから呼び出し可能
- テスト用にフィールド高さ21・壁・床を活用し公式条件を再現

## テスト戦略
- 公式T-Spin（3角埋まり＋wallkick）を満たす状況でのみ成功するテストをRustで作成
- SRS回転失敗時は現状維持となることもテストで検証
- 他スピン技（S/Z/I/J/L-Spin）も同様にテスト可能

## 今後の展望
- WASM経由でJS/TS側のUI・スコア・エフェクト・実績連携を本格運用
- Phase4以降はAPI/DB/認証などフルスタック化やUI統合へ 