use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen;
use std::cell::RefCell;
use rand::Rng;
use std::collections::HashMap;

thread_local! {
    static FIELD: RefCell<Field> = RefCell::new(Field {
        width: 10,
        height: 20,
        cells: vec![0; 200],
    });
    static TETROMINO: RefCell<Tetromino> = RefCell::new(Tetromino {
        kind: "I".to_string(),
        x: 4,
        y: 0,
        rotation: 0,
    });
    static LAST_TSPIN: RefCell<TSpinType> = RefCell::new(TSpinType::None);
    static LAST_WALLKICK: RefCell<bool> = RefCell::new(false);
    static LAST_KICK_INDEX: RefCell<i32> = RefCell::new(0);
}

const TETROMINO_TYPES: [&str; 7] = ["I", "O", "T", "S", "Z", "J", "L"];

// テトリミノ形状・回転ごとの4マス相対座標テーブル
const TETROMINO_SHAPES: &[(&str, [[(i32, i32); 4]; 4])] = &[
    // Iミノ
    ("I", [
        [(0, 1), (1, 1), (2, 1), (3, 1)], // 0度
        [(2, 0), (2, 1), (2, 2), (2, 3)], // 90度
        [(0, 2), (1, 2), (2, 2), (3, 2)], // 180度
        [(1, 0), (1, 1), (1, 2), (1, 3)], // 270度
    ]),
    // Oミノ
    ("O", [
        [(1, 0), (2, 0), (1, 1), (2, 1)], // 0度
        [(1, 0), (2, 0), (1, 1), (2, 1)], // 90度
        [(1, 0), (2, 0), (1, 1), (2, 1)], // 180度
        [(1, 0), (2, 0), (1, 1), (2, 1)], // 270度
    ]),
    // Tミノ
    ("T", [
        [(1, 0), (0, 1), (1, 1), (2, 1)], // 0度
        [(1, 0), (1, 1), (2, 1), (1, 2)], // 90度
        [(0, 1), (1, 1), (2, 1), (1, 2)], // 180度
        [(1, 0), (0, 1), (1, 1), (1, 2)], // 270度
    ]),
    // Sミノ
    ("S", [
        [(1, 0), (2, 0), (0, 1), (1, 1)], // 0度
        [(1, 0), (1, 1), (2, 1), (2, 2)], // 90度
        [(1, 1), (2, 1), (0, 2), (1, 2)], // 180度
        [(0, 0), (0, 1), (1, 1), (1, 2)], // 270度
    ]),
    // Zミノ
    ("Z", [
        [(0, 0), (1, 0), (1, 1), (2, 1)], // 0度
        [(2, 0), (1, 1), (2, 1), (1, 2)], // 90度
        [(0, 1), (1, 1), (1, 2), (2, 2)], // 180度
        [(1, 0), (0, 1), (1, 1), (0, 2)], // 270度
    ]),
    // Jミノ
    ("J", [
        [(0, 0), (0, 1), (1, 1), (2, 1)], // 0度
        [(1, 0), (2, 0), (1, 1), (1, 2)], // 90度
        [(0, 1), (1, 1), (2, 1), (2, 2)], // 180度
        [(1, 0), (1, 1), (0, 2), (1, 2)], // 270度
    ]),
    // Lミノ
    ("L", [
        [(2, 0), (0, 1), (1, 1), (2, 1)], // 0度
        [(1, 0), (1, 1), (1, 2), (2, 2)], // 90度
        [(0, 1), (1, 1), (2, 1), (0, 2)], // 180度
        [(0, 0), (1, 0), (1, 1), (1, 2)], // 270度
    ]),
];

#[derive(Serialize, Deserialize, Clone)]
pub struct Field {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tetromino {
    pub kind: String,
    pub x: i32,
    pub y: i32,
    pub rotation: u8,
}

#[derive(Clone, Copy)]
struct KickOffset { x: i32, y: i32 }

type KickTable = HashMap<(u8, u8), Vec<KickOffset>>;

fn random_tetromino_kind() -> String {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..TETROMINO_TYPES.len());
    TETROMINO_TYPES[idx].to_string()
}

#[wasm_bindgen]
pub fn init_field(width: u32, height: u32) -> JsValue {
    FIELD.with(|f| {
        *f.borrow_mut() = Field {
            width,
            height,
            cells: vec![0; (width * height) as usize],
        };
        serde_wasm_bindgen::to_value(&*f.borrow()).unwrap()
    })
}

#[wasm_bindgen]
pub fn spawn_tetromino() -> JsValue {
    TETROMINO.with(|t| {
        let kind = random_tetromino_kind();
        *t.borrow_mut() = Tetromino {
            kind,
            x: 4,
            y: 0,
            rotation: 0,
        };
        // スピン判定状態をリセット
        LAST_TSPIN.with(|ts| *ts.borrow_mut() = TSpinType::None);
        LAST_WALLKICK.with(|w| *w.borrow_mut() = false);
        LAST_KICK_INDEX.with(|k| *k.borrow_mut() = 0);
        serde_wasm_bindgen::to_value(&*t.borrow()).unwrap()
    })
}

// 衝突判定の雛形
fn check_collision(field: &Field, tetromino: &Tetromino) -> bool {
    let blocks = get_tetromino_blocks(&tetromino.kind, tetromino.x, tetromino.y, tetromino.rotation);
    
    for (x, y) in blocks {
        // フィールド外チェック
        if x < 0 || y < 0 || (x as u32) >= field.width || (y as u32) >= field.height {
            return true;
        }
        
        // 既存ブロックとの衝突チェック
        let idx = (y as u32 * field.width + x as u32) as usize;
        if idx < field.cells.len() && field.cells[idx] != 0 {
            return true;
        }
    }
    
    false
}

fn get_tetromino_blocks(kind: &str, x: i32, y: i32, rotation: u8) -> Vec<(i32, i32)> {
    for (k, shapes) in TETROMINO_SHAPES {
        if *k == kind {
            let rot = (rotation % 4) as usize;
            return shapes[rot].iter().map(|(dx, dy)| (x + dx, y + dy)).collect();
        }
    }
    vec![]
}

// fix_tetrominoで4マスすべてをフィールドに埋め込む
fn fix_tetromino(field: &mut Field, tetromino: &Tetromino) {
    let blocks = get_tetromino_blocks(&tetromino.kind, tetromino.x, tetromino.y, tetromino.rotation);
    for (bx, by) in blocks {
        if bx >= 0 && by >= 0 && (bx as u32) < field.width && (by as u32) < field.height {
            let idx = (by as u32 * field.width + bx as u32) as usize;
            if idx < field.cells.len() {
                field.cells[idx] = 1;
            }
        }
    }
}

// ライン消去の雛形
fn clear_lines(field: &mut Field) -> u32 {
    let width = field.width as usize;
    let mut new_cells = vec![];
    let mut cleared = 0;
    for row in field.cells.chunks(width) {
        if row.iter().all(|&c| c != 0) {
            cleared += 1;
        } else {
            new_cells.extend_from_slice(row);
        }
    }
    // 上から空行を追加
    for _ in 0..cleared {
        new_cells.splice(0..0, vec![0; width]);
    }
    field.cells = new_cells;
    cleared
}

fn fix_tetromino_with_tspin(field: &mut Field, tetromino: &Tetromino) {
    let wallkick = LAST_WALLKICK.with(|w| *w.borrow());
    let kick_index = LAST_KICK_INDEX.with(|k| *k.borrow());
    let tspin = detect_t_spin(tetromino, field, wallkick, kick_index);
    LAST_TSPIN.with(|t| *t.borrow_mut() = tspin);
    fix_tetromino(field, tetromino);
    clear_lines(field);
}

#[wasm_bindgen]
pub fn move_tetromino(dir: &str) -> JsValue {
    let mut fixed = false;
    let mut result = None;
    FIELD.with(|f| {
        TETROMINO.with(|t| {
            let mut tetro = t.borrow().clone();
            let mut field = f.borrow().clone();
            match dir {
                "left" => { tetro.x -= 1; },
                "right" => { tetro.x += 1; },
                "down" => { tetro.y += 1; },
                _ => {}
            }
            if check_collision(&field, &tetro) {
                // 衝突したら元に戻す
                match dir {
                    "left" => { tetro.x += 1; },
                    "right" => { tetro.x -= 1; },
                    "down" => {
                        tetro.y -= 1;
                        // 下方向衝突時は固定
                        fix_tetromino_with_tspin(&mut field, &tetro);
                        clear_lines(&mut field);
                        fixed = true;
                    },
                    _ => {}
                }
            }
            *t.borrow_mut() = tetro.clone();
            *f.borrow_mut() = field.clone();
            result = Some(serde_wasm_bindgen::to_value(&tetro).unwrap());
        });
    });
    result.unwrap()
}

fn srs_kick_table() -> KickTable {
    let mut table = HashMap::new();
    // 通常ミノ用SRSキックテーブル（例: 0->1, 1->0, ...）
    table.insert((0, 1), vec![KickOffset {x:0, y:0}, KickOffset {x:-1, y:0}, KickOffset {x:-1, y:1}, KickOffset {x:0, y:-2}, KickOffset {x:-1, y:-2}]);
    table.insert((1, 0), vec![KickOffset {x:0, y:0}, KickOffset {x:1, y:0}, KickOffset {x:1, y:-1}, KickOffset {x:0, y:2}, KickOffset {x:1, y:2}]);
    table.insert((1, 2), vec![KickOffset {x:0, y:0}, KickOffset {x:1, y:0}, KickOffset {x:1, y:-1}, KickOffset {x:0, y:2}, KickOffset {x:1, y:2}]);
    table.insert((2, 1), vec![KickOffset {x:0, y:0}, KickOffset {x:-1, y:0}, KickOffset {x:-1, y:1}, KickOffset {x:0, y:-2}, KickOffset {x:-1, y:-2}]);
    table.insert((2, 3), vec![KickOffset {x:0, y:0}, KickOffset {x:1, y:0}, KickOffset {x:1, y:1}, KickOffset {x:0, y:-2}, KickOffset {x:1, y:-2}]);
    table.insert((3, 2), vec![KickOffset {x:0, y:0}, KickOffset {x:-1, y:0}, KickOffset {x:-1, y:-1}, KickOffset {x:0, y:2}, KickOffset {x:-1, y:2}]);
    table.insert((3, 0), vec![KickOffset {x:0, y:0}, KickOffset {x:-1, y:0}, KickOffset {x:-1, y:-1}, KickOffset {x:0, y:2}, KickOffset {x:-1, y:2}]);
    table.insert((0, 3), vec![KickOffset {x:0, y:0}, KickOffset {x:1, y:0}, KickOffset {x:1, y:1}, KickOffset {x:0, y:-2}, KickOffset {x:1, y:-2}]);
    table
}

fn srs_i_kick_table() -> KickTable {
    let mut table = HashMap::new();
    // Iミノ専用SRSキックテーブル
    table.insert((0, 1), vec![KickOffset {x:0, y:0}, KickOffset {x:-2, y:0}, KickOffset {x:1, y:0}, KickOffset {x:-2, y:-1}, KickOffset {x:1, y:2}]);
    table.insert((1, 0), vec![KickOffset {x:0, y:0}, KickOffset {x:2, y:0}, KickOffset {x:-1, y:0}, KickOffset {x:2, y:1}, KickOffset {x:-1, y:-2}]);
    table.insert((1, 2), vec![KickOffset {x:0, y:0}, KickOffset {x:-1, y:0}, KickOffset {x:2, y:0}, KickOffset {x:-1, y:2}, KickOffset {x:2, y:-1}]);
    table.insert((2, 1), vec![KickOffset {x:0, y:0}, KickOffset {x:1, y:0}, KickOffset {x:-2, y:0}, KickOffset {x:1, y:-2}, KickOffset {x:-2, y:1}]);
    table.insert((2, 3), vec![KickOffset {x:0, y:0}, KickOffset {x:2, y:0}, KickOffset {x:-1, y:0}, KickOffset {x:2, y:1}, KickOffset {x:-1, y:-2}]);
    table.insert((3, 2), vec![KickOffset {x:0, y:0}, KickOffset {x:-2, y:0}, KickOffset {x:1, y:0}, KickOffset {x:-2, y:-1}, KickOffset {x:1, y:2}]);
    table.insert((3, 0), vec![KickOffset {x:0, y:0}, KickOffset {x:1, y:0}, KickOffset {x:-2, y:0}, KickOffset {x:1, y:-2}, KickOffset {x:-2, y:1}]);
    table.insert((0, 3), vec![KickOffset {x:0, y:0}, KickOffset {x:-1, y:0}, KickOffset {x:2, y:0}, KickOffset {x:-1, y:2}, KickOffset {x:2, y:-1}]);
    table
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RotationResult {
    pub tetromino: Tetromino,
    pub wallkick: bool,
    pub kick_index: i32,
}

// SRS回転処理の返り値を(RotationResult)に
fn attempt_srs_rotation(tetromino: &Tetromino, field: &Field, clockwise: bool) -> RotationResult {
    let from = tetromino.rotation;
    let to = if clockwise {
        (from + 1) % 4
    } else {
        (from + 3) % 4
    };
    let kick_table = if tetromino.kind == "I" {
        srs_i_kick_table()
    } else {
        srs_kick_table()
    };
    let kicks = kick_table.get(&(from, to)).cloned().unwrap_or_else(|| vec![KickOffset {x:0, y:0}]);
    for (i, kick) in kicks.iter().enumerate() {
        let mut test = tetromino.clone();
        test.x += kick.x;
        test.y += kick.y;
        test.rotation = to;
        if !check_collision(field, &test) {
            return RotationResult {
                tetromino: test,
                wallkick: i > 0,
                kick_index: i as i32,
            };
        }
    }
    RotationResult {
        tetromino: tetromino.clone(),
        wallkick: false,
        kick_index: -1,
    }
}

#[wasm_bindgen]
pub fn rotate_tetromino(dir: &str) -> JsValue {
    let clockwise = dir != "left";
    let mut result = None;
    FIELD.with(|f| {
        TETROMINO.with(|t| {
            let tetro = t.borrow().clone();
            let field = f.borrow().clone();
            let rot_result = attempt_srs_rotation(&tetro, &field, clockwise);
            
            // wallkick情報を保存
            LAST_WALLKICK.with(|w| *w.borrow_mut() = rot_result.wallkick);
            LAST_KICK_INDEX.with(|k| *k.borrow_mut() = rot_result.kick_index);
            
            *t.borrow_mut() = rot_result.tetromino.clone();
            result = Some(serde_wasm_bindgen::to_value(&rot_result).unwrap());
        });
    });
    result.unwrap()
}

#[wasm_bindgen]
pub fn hard_drop() -> JsValue {
    let mut fixed = false;
    let mut result = None;
    FIELD.with(|f| {
        TETROMINO.with(|t| {
            let mut tetro = t.borrow().clone();
            let mut field = f.borrow().clone();
            // 一番下まで落とす
            while !check_collision(&field, &tetro) {
                tetro.y += 1;
            }
            tetro.y -= 1;
            fix_tetromino_with_tspin(&mut field, &tetro);
            clear_lines(&mut field);
            fixed = true;
            *t.borrow_mut() = tetro.clone();
            *f.borrow_mut() = field.clone();
            result = Some(serde_wasm_bindgen::to_value(&tetro).unwrap());
        });
    });
    result.unwrap()
}

#[wasm_bindgen]
pub fn get_score() -> u32 { 0 }

#[wasm_bindgen]
pub fn get_rank() -> u8 { 0 }

#[wasm_bindgen]
pub fn get_achievements() -> JsValue { JsValue::NULL }

#[wasm_bindgen]
pub fn get_field() -> JsValue {
    FIELD.with(|f| serde_wasm_bindgen::to_value(&*f.borrow()).unwrap())
}

#[wasm_bindgen]
pub fn get_current_tetromino() -> JsValue {
    TETROMINO.with(|t| serde_wasm_bindgen::to_value(&*t.borrow()).unwrap())
}

#[wasm_bindgen]
pub fn get_game_state() -> JsValue {
    let field = FIELD.with(|f| serde_wasm_bindgen::to_value(&*f.borrow()).unwrap());
    let tetromino = TETROMINO.with(|t| serde_wasm_bindgen::to_value(&*t.borrow()).unwrap());
    let last_tspin = LAST_TSPIN.with(|t| serde_wasm_bindgen::to_value(&*t.borrow()).unwrap());
    let last_wallkick = LAST_WALLKICK.with(|w| serde_wasm_bindgen::to_value(&*w.borrow()).unwrap());
    let last_kick_index = LAST_KICK_INDEX.with(|k| serde_wasm_bindgen::to_value(&*k.borrow()).unwrap());
    
    // JsValueを直接組み立て
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"field".into(), &field).unwrap();
    js_sys::Reflect::set(&obj, &"tetromino".into(), &tetromino).unwrap();
    js_sys::Reflect::set(&obj, &"lastTspin".into(), &last_tspin).unwrap();
    js_sys::Reflect::set(&obj, &"lastWallkick".into(), &last_wallkick).unwrap();
    js_sys::Reflect::set(&obj, &"lastKickIndex".into(), &last_kick_index).unwrap();
    
    obj.into()
}

// Tミノの中心ブロック座標を取得
fn get_t_center_block(tetromino: &Tetromino) -> (i32, i32) {
    // Tミノの形状テーブルの1,1が中心
    (tetromino.x + 1, tetromino.y + 1)
}

// フィールド上の(x, y)が埋まっているか
fn is_filled(x: i32, y: i32, field: &Field) -> bool {
    if x < 0 || y < 0 || (x as u32) >= field.width || (y as u32) >= field.height {
        return true; // フィールド外は埋まっている扱い（壁・床・天井）
    }
    let idx = (y as u32 * field.width + x as u32) as usize;
    field.cells.get(idx).copied().unwrap_or(0) != 0
}

// Tミノ中心4角の埋まり数を判定
fn count_t_spin_corners(center: (i32, i32), field: &Field) -> usize {
    let (cx, cy) = center;
    let corners = [
        (cx - 1, cy - 1), // 左上
        (cx + 1, cy - 1), // 右上
        (cx - 1, cy + 1), // 左下
        (cx + 1, cy + 1), // 右下
    ];
    corners.iter().filter(|&&(x, y)| is_filled(x, y, field)).count()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TSpinType {
    None,
    Mini,
    Normal,
}

// T-Spin判定関数
fn detect_t_spin(
    tetromino: &Tetromino,
    field: &Field,
    wallkick: bool,
    kick_index: i32,
) -> TSpinType {
    // Tミノ以外はT-Spinなし
    if tetromino.kind != "T" {
        return TSpinType::None;
    }
    // 中心座標
    let center = get_t_center_block(tetromino);
    let corners = count_t_spin_corners(center, field);
    // 3つ以上角が埋まっていればT-Spin
    if corners >= 3 {
        // Mini判定: SRS公式では特定のkick index（0以外）や特定回転でMini
        // ここではkick_index==0（最初のkick）ならNormal, それ以外はMiniとする
        if wallkick && kick_index > 0 {
            TSpinType::Mini
        } else {
            TSpinType::Normal
        }
    } else {
        TSpinType::None
    }
}

#[wasm_bindgen]
pub fn detect_t_spin_wasm(
    tetromino: JsValue,
    field: JsValue,
    wallkick: bool,
    kick_index: i32,
) -> JsValue {
    let tetromino: Tetromino = serde_wasm_bindgen::from_value(tetromino).unwrap();
    let field: Field = serde_wasm_bindgen::from_value(field).unwrap();
    let result = detect_t_spin(&tetromino, &field, wallkick, kick_index);
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn get_last_tspin_type() -> JsValue {
    LAST_TSPIN.with(|t| serde_wasm_bindgen::to_value(&*t.borrow()).unwrap())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpinType {
    None,
    TSpin,
    SZSpin,
    ISpin,
    JLSpin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpinVariant {
    None,
    Single,
    Double,
    Triple,
    Mini,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinResult {
    pub spin_type: SpinType,
    pub variant: SpinVariant,
    pub bonus: u32,
    pub lines: u32,
}

// 各ミノの中心ブロック座標を取得
fn get_center_block(tetromino: &Tetromino) -> (i32, i32) {
    match tetromino.kind.as_str() {
        "T" => (tetromino.x + 1, tetromino.y + 1), // Tミノの形状テーブルの1,1が中心
        "S" | "Z" => (tetromino.x + 1, tetromino.y + 1), // S/Zミノも同様
        "I" => (tetromino.x + 1, tetromino.y + 1), // Iミノも同様
        "J" | "L" => (tetromino.x + 1, tetromino.y + 1), // J/Lミノも同様
        _ => (tetromino.x, tetromino.y), // その他は基準位置
    }
}

// 各ミノの角位置を取得
fn get_corner_positions(center: (i32, i32), tetromino: &Tetromino) -> [(i32, i32); 4] {
    let (cx, cy) = center;
    match tetromino.kind.as_str() {
        "I" => {
            // Iミノは長い形状なので、回転に応じて角位置が変わる
            match tetromino.rotation {
                0 | 2 => [ // 横長
                    (cx - 2, cy - 1), // 左上
                    (cx + 2, cy - 1), // 右上
                    (cx - 2, cy + 1), // 左下
                    (cx + 2, cy + 1), // 右下
                ],
                1 | 3 => [ // 縦長
                    (cx - 1, cy - 2), // 左上
                    (cx + 1, cy - 2), // 右上
                    (cx - 1, cy + 2), // 左下
                    (cx + 1, cy + 2), // 右下
                ],
                _ => [
                    (cx - 1, cy - 1), // 左上
                    (cx + 1, cy - 1), // 右上
                    (cx - 1, cy + 1), // 左下
                    (cx + 1, cy + 1), // 右下
                ],
            }
        },
        _ => {
            // その他のミノは通常の角位置
            [
                (cx - 1, cy - 1), // 左上
                (cx + 1, cy - 1), // 右上
                (cx - 1, cy + 1), // 左下
                (cx + 1, cy + 1), // 右下
            ]
        }
    }
}

// 中心4角の埋まり数を判定
fn count_corners(center: (i32, i32), tetromino: &Tetromino, field: &Field) -> usize {
    let corners = get_corner_positions(center, tetromino);
    corners.iter().filter(|&&(x, y)| is_filled(x, y, field)).count()
}

// S/Z-Spin判定
fn detect_sz_spin(tetromino: &Tetromino, field: &Field, wallkick: bool, _kick_index: i32) -> SpinResult {
    if tetromino.kind != "S" && tetromino.kind != "Z" {
        return SpinResult {
            spin_type: SpinType::None,
            variant: SpinVariant::None,
            bonus: 0,
            lines: 0,
        };
    }
    
    if !wallkick {
        return SpinResult {
            spin_type: SpinType::None,
            variant: SpinVariant::None,
            bonus: 0,
            lines: 0,
        };
    }
    
    let center = get_center_block(tetromino);
    let corners = count_corners(center, tetromino, field);
    
    if corners >= 3 {
        SpinResult {
            spin_type: SpinType::SZSpin,
            variant: SpinVariant::Single,
            bonus: 100,
            lines: 1,
        }
    } else {
        SpinResult {
            spin_type: SpinType::None,
            variant: SpinVariant::None,
            bonus: 0,
            lines: 0,
        }
    }
}

// I-Spin判定
fn detect_i_spin(tetromino: &Tetromino, field: &Field, wallkick: bool, _kick_index: i32) -> SpinResult {
    if tetromino.kind != "I" {
        return SpinResult {
            spin_type: SpinType::None,
            variant: SpinVariant::None,
            bonus: 0,
            lines: 0,
        };
    }
    
    if !wallkick {
        return SpinResult {
            spin_type: SpinType::None,
            variant: SpinVariant::None,
            bonus: 0,
            lines: 0,
        };
    }
    
    let center = get_center_block(tetromino);
    let corners = count_corners(center, tetromino, field);
    
    if corners >= 3 {
        SpinResult {
            spin_type: SpinType::ISpin,
            variant: SpinVariant::Single,
            bonus: 100,
            lines: 1,
        }
    } else {
        SpinResult {
            spin_type: SpinType::None,
            variant: SpinVariant::None,
            bonus: 0,
            lines: 0,
        }
    }
}

// J/L-Spin判定
fn detect_jl_spin(tetromino: &Tetromino, field: &Field, wallkick: bool, _kick_index: i32) -> SpinResult {
    if tetromino.kind != "J" && tetromino.kind != "L" {
        return SpinResult {
            spin_type: SpinType::None,
            variant: SpinVariant::None,
            bonus: 0,
            lines: 0,
        };
    }
    
    if !wallkick {
        return SpinResult {
            spin_type: SpinType::None,
            variant: SpinVariant::None,
            bonus: 0,
            lines: 0,
        };
    }
    
    let center = get_center_block(tetromino);
    let corners = count_corners(center, tetromino, field);
    
    if corners >= 3 {
        SpinResult {
            spin_type: SpinType::JLSpin,
            variant: SpinVariant::Single,
            bonus: 100,
            lines: 1,
        }
    } else {
        SpinResult {
            spin_type: SpinType::None,
            variant: SpinVariant::None,
            bonus: 0,
            lines: 0,
        }
    }
}

// 統合スピン判定関数
fn detect_spin(tetromino: &Tetromino, field: &Field, wallkick: bool, kick_index: i32, lines_cleared: u32) -> SpinResult {
    let result = match tetromino.kind.as_str() {
        "T" => {
            let tspin = detect_t_spin(tetromino, field, wallkick, kick_index);
            match tspin {
                TSpinType::None => SpinResult { spin_type: SpinType::None, variant: SpinVariant::None, bonus: 0, lines: lines_cleared },
                TSpinType::Mini => {
                    let bonus = match lines_cleared {
                        1 => 1000,
                        2 => 2000,
                        3 => 3000,
                        _ => 0,
                    };
                    let _variant = match lines_cleared {
                        1 => SpinVariant::Single,
                        2 => SpinVariant::Double,
                        3 => SpinVariant::Triple,
                        _ => SpinVariant::None,
                    };
                    SpinResult { spin_type: SpinType::TSpin, variant: SpinVariant::Mini, bonus, lines: lines_cleared }
                },
                TSpinType::Normal => {
                    let bonus = match lines_cleared {
                        1 => 2000,
                        2 => 5000,
                        3 => 10000,
                        _ => 0,
                    };
                    let variant = match lines_cleared {
                        1 => SpinVariant::Single,
                        2 => SpinVariant::Double,
                        3 => SpinVariant::Triple,
                        _ => SpinVariant::None,
                    };
                    SpinResult { spin_type: SpinType::TSpin, variant, bonus, lines: lines_cleared }
                }
            }
        },
        "S" | "Z" => {
            let mut result = detect_sz_spin(tetromino, field, wallkick, kick_index);
            result.lines = lines_cleared;
            result.bonus = match lines_cleared {
                1 => 800,
                2 => 2000,
                3 => 4000,
                _ => 0,
            };
            result.variant = match lines_cleared {
                1 => SpinVariant::Single,
                2 => SpinVariant::Double,
                3 => SpinVariant::Triple,
                _ => SpinVariant::None,
            };
            result
        },
        "I" => {
            let mut result = detect_i_spin(tetromino, field, wallkick, kick_index);
            result.lines = lines_cleared;
            result.bonus = match lines_cleared {
                1 => 600,
                2 => 1500,
                3 => 3000,
                _ => 0,
            };
            result.variant = match lines_cleared {
                1 => SpinVariant::Single,
                2 => SpinVariant::Double,
                3 => SpinVariant::Triple,
                _ => SpinVariant::None,
            };
            result
        },
        "J" | "L" => {
            let mut result = detect_jl_spin(tetromino, field, wallkick, kick_index);
            result.lines = lines_cleared;
            result.bonus = match lines_cleared {
                1 => 700,
                2 => 1800,
                3 => 3500,
                _ => 0,
            };
            result.variant = match lines_cleared {
                1 => SpinVariant::Single,
                2 => SpinVariant::Double,
                3 => SpinVariant::Triple,
                _ => SpinVariant::None,
            };
            result
        },
        _ => SpinResult { spin_type: SpinType::None, variant: SpinVariant::None, bonus: 0, lines: lines_cleared },
    };
    
    result
}

// WASMバインディング用の統合スピン判定API
#[wasm_bindgen]
pub fn detect_spin_wasm(
    tetromino: JsValue,
    field: JsValue,
    wallkick: bool,
    kick_index: i32,
    lines_cleared: u32,
) -> JsValue {
    let tetromino: Tetromino = serde_wasm_bindgen::from_value(tetromino).unwrap();
    let field: Field = serde_wasm_bindgen::from_value(field).unwrap();
    let result = detect_spin(&tetromino, &field, wallkick, kick_index, lines_cleared);
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_spin_detection() {
        // 空のフィールド
        let field = Field {
            width: 10,
            height: 20,
            cells: vec![0; 200],
        };

        // Tミノ（0度回転）
        let t_tetromino = Tetromino {
            kind: "T".to_string(),
            x: 4,
            y: 18, // 下の方に配置
            rotation: 0,
        };

        // 通常のT-Spin（角が3つ埋まっている）
        let mut field_with_walls = field.clone();
        // 下の行を埋める
        for i in 0..10 {
            field_with_walls.cells[19 * 10 + i] = 1;
        }
        // 左右の壁を埋める（Tミノ中心(5,19)の左右）
        field_with_walls.cells[18 * 10 + 4] = 1; // 左 (4,18)
        field_with_walls.cells[18 * 10 + 6] = 1; // 右 (6,18)

        let result = detect_t_spin(&t_tetromino, &field_with_walls, false, 0);
        assert_eq!(result, TSpinType::Normal);

        // wallkickありのT-Spin Mini
        let result = detect_t_spin(&t_tetromino, &field_with_walls, true, 1);
        assert_eq!(result, TSpinType::Mini);

        // Tミノ以外はT-Spinなし
        let i_tetromino = Tetromino {
            kind: "I".to_string(),
            x: 4,
            y: 18,
            rotation: 0,
        };
        let result = detect_t_spin(&i_tetromino, &field_with_walls, false, 0);
        assert_eq!(result, TSpinType::None);

        // 角が2つ以下ならT-Spinなし
        let mut field_partial = field.clone();
        field_partial.cells[19 * 10 + 5] = 1; // 下のみ（中心の下）
        let result = detect_t_spin(&t_tetromino, &field_partial, false, 0);
        assert_eq!(result, TSpinType::None);
    }

    #[test]
    fn test_t_center_block() {
        let t_tetromino = Tetromino {
            kind: "T".to_string(),
            x: 4,
            y: 18,
            rotation: 0,
        };
        let center = get_t_center_block(&t_tetromino);
        assert_eq!(center, (5, 19)); // x+1, y+1
    }

    #[test]
    fn test_corner_counting() {
        let field = Field {
            width: 10,
            height: 20,
            cells: vec![0; 200],
        };
        let tetromino = Tetromino {
            kind: "T".to_string(),
            x: 4,
            y: 4,
            rotation: 0,
        };
        let center = get_t_center_block(&tetromino);
        let corners = count_t_spin_corners(center, &field);
        assert_eq!(corners, 0);
    }

    #[test]
    fn test_srs_rotation() {
        let field = Field {
            width: 10,
            height: 20,
            cells: vec![0; 200],
        };
        let tetromino = Tetromino {
            kind: "T".to_string(),
            x: 4,
            y: 4,
            rotation: 0,
        };
        let result = attempt_srs_rotation(&tetromino, &field, true);
        assert_eq!(result.tetromino.rotation, 1);
        assert!(!result.wallkick); // 通常回転なのでwallkickなし
    }

    #[test]
    fn test_srs_rotation_with_wallkick() {
        let mut field = Field {
            width: 10,
            height: 20,
            cells: vec![0; 200],
        };
        // 右端にブロックを配置
        for y in 0..20 {
            field.cells[y * 10 + 9] = 1;
        }
        let tetromino = Tetromino {
            kind: "T".to_string(),
            x: 7,
            y: 4,
            rotation: 0,
        };
        let result = attempt_srs_rotation(&tetromino, &field, true);
        assert_eq!(result.tetromino.rotation, 1);
        assert!(result.wallkick); // 壁蹴りが必要
        assert!(result.kick_index > 0);
    }

    #[test]
    fn test_spin_detection_integration() {
        let mut field = Field {
            width: 10,
            height: 21, // 21行に拡張（y=20が床）
            cells: vec![0; 210], // 10 * 21 = 210
        };
        
        // 公式TTC仕様に準拠したT-Spinテスト
        // 1. 床部分（y=20）を埋める
        for x in 0..10 {
            field.cells[20 * 10 + x] = 1; // 床を埋める
        }
        
        // 2. 左右の壁を埋める
        for y in 0..21 {
            field.cells[y * 10 + 0] = 1; // 左端を埋める
            field.cells[y * 10 + 9] = 1; // 右端を埋める
        }
        
        // 3. Tミノを90度回転済みで配置（中心が(5,19)）
        let tetromino = Tetromino {
            kind: "T".to_string(),
            x: 0, // 左壁に隣接（中心が(1,19)）
            y: 18,
            rotation: 1, // 90度回転済み
        };
        
        // 4. 回転後の中心座標を計算
        let center_x = tetromino.x + 1;
        let center_y = tetromino.y + 1;
        println!("center: ({}, {})", center_x, center_y);
        
        // 5. 中心4角の座標を計算
        let corners = [
            (center_x - 1, center_y - 1), // 左上
            (center_x + 1, center_y - 1), // 右上
            (center_x - 1, center_y + 1), // 左下
            (center_x + 1, center_y + 1), // 右下
        ];
        println!("corners: {:?}", corners);
        
        // 6. 各角の埋まり状況を確認
        for (i, &(x, y)) in corners.iter().enumerate() {
            let filled = is_filled(x, y, &field);
            println!("corner {}: ({}, {}) -> filled: {} (field: {}x{})", i, x, y, filled, field.width, field.height);
        }
        
        // フィールド外のテスト
        println!("field boundary test:");
        println!("(-1, 0) -> filled: {}", is_filled(-1, 0, &field));
        println!("(10, 0) -> filled: {}", is_filled(10, 0, &field));
        println!("(0, -1) -> filled: {}", is_filled(0, -1, &field));
        println!("(0, 21) -> filled: {}", is_filled(0, 21, &field));
        
        // 7. スピン判定を実行（wallkick=true, kick_index=1でT-Spin Mini）
        let spin_result = detect_spin(&tetromino, &field, true, 1, 1);
        println!("spin_result: {:?}", spin_result);
        
        // T-Spin判定の詳細を確認
        let center = get_t_center_block(&tetromino);
        let corner_count = count_t_spin_corners(center, &field);
        println!("T-Spin corner count: {} (center: {:?})", corner_count, center);
        
        // 8. 公式仕様の条件を確認
        assert_eq!(spin_result.spin_type, SpinType::TSpin, "T-Spinが検出されていない");
        assert!(spin_result.bonus > 0, "ボーナススコアが0");
    }

    #[test]
    fn test_collision_detection() {
        let mut field = Field {
            width: 10,
            height: 20,
            cells: vec![0; 200],
        };
        // 衝突しない位置（フィールド中央は空）
        let tetromino = Tetromino {
            kind: "T".to_string(),
            x: 4,
            y: 9,
            rotation: 0,
        };
        let blocks = get_tetromino_blocks(&tetromino.kind, tetromino.x, tetromino.y, tetromino.rotation);
        println!("tetromino blocks: {:?}", blocks);
        assert!(!check_collision(&field, &tetromino));
        // 衝突する位置（中央にブロックを配置）
        field.cells[10 * 10 + 5] = 1;
        let mut colliding_tetromino = tetromino.clone();
        let blocks2 = get_tetromino_blocks(&colliding_tetromino.kind, colliding_tetromino.x, colliding_tetromino.y, colliding_tetromino.rotation);
        println!("colliding blocks: {:?}", blocks2);
        assert!(check_collision(&field, &colliding_tetromino));
    }
} 