use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub field: Vec<Vec<u8>>,
    pub current_tetromino: Tetromino,
    pub next_tetromino: Tetromino,
    pub score: u32,
    pub level: u32,
    pub lines_cleared: u32,
    pub game_over: bool,
    pub paused: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tetromino {
    pub shape: Vec<Vec<u8>>,
    pub x: i32,
    pub y: i32,
    pub rotation: u8,
    pub color: u8,
}

#[wasm_bindgen]
pub struct GameEngine {
    state: GameState,
}

#[wasm_bindgen]
impl GameEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            state: GameState {
                field: vec![vec![0; 10]; 20],
                current_tetromino: Self::create_tetromino(),
                next_tetromino: Self::create_tetromino(),
                score: 0,
                level: 1,
                lines_cleared: 0,
                game_over: false,
                paused: false,
            }
        }
    }

    pub fn get_state(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.state).unwrap()
    }

    pub fn move_tetromino(&mut self, direction: &str) -> JsValue {
        match direction {
            "left" => self.move_left(),
            "right" => self.move_right(),
            "down" => self.move_down(),
            _ => {}
        }
        self.get_state()
    }

    pub fn rotate_tetromino(&mut self, direction: &str) -> JsValue {
        match direction {
            "clockwise" => self.rotate_clockwise(),
            "counterclockwise" => self.rotate_counterclockwise(),
            _ => {}
        }
        self.get_state()
    }

    pub fn drop_tetromino(&mut self) -> JsValue {
        while self.move_down() {}
        self.get_state()
    }

    pub fn update(&mut self) -> JsValue {
        if !self.state.game_over && !self.state.paused {
            if !self.move_down() {
                self.lock_tetromino();
                self.clear_lines();
                self.spawn_new_tetromino();
            }
        }
        self.get_state()
    }

    fn create_tetromino() -> Tetromino {
        // 簡易的なテトロミノ作成（後で実装）
        Tetromino {
            shape: vec![vec![1, 1], vec![1, 1]], // I字型
            x: 4,
            y: 0,
            rotation: 0,
            color: 1,
        }
    }

    fn move_left(&mut self) -> bool {
        self.state.current_tetromino.x -= 1;
        if !self.is_valid_position() {
            self.state.current_tetromino.x += 1;
            false
        } else {
            true
        }
    }

    fn move_right(&mut self) -> bool {
        self.state.current_tetromino.x += 1;
        if !self.is_valid_position() {
            self.state.current_tetromino.x -= 1;
            false
        } else {
            true
        }
    }

    fn move_down(&mut self) -> bool {
        self.state.current_tetromino.y += 1;
        if !self.is_valid_position() {
            self.state.current_tetromino.y -= 1;
            false
        } else {
            true
        }
    }

    fn rotate_clockwise(&mut self) -> bool {
        // 回転ロジック（後で実装）
        true
    }

    fn rotate_counterclockwise(&mut self) -> bool {
        // 回転ロジック（後で実装）
        true
    }

    fn is_valid_position(&self) -> bool {
        // 衝突検出ロジック（後で実装）
        true
    }

    fn lock_tetromino(&mut self) {
        // テトロミノを固定するロジック（後で実装）
    }

    fn clear_lines(&mut self) {
        // ライン消去ロジック（後で実装）
    }

    fn spawn_new_tetromino(&mut self) {
        // 新しいテトロミノを生成するロジック（後で実装）
    }
} 