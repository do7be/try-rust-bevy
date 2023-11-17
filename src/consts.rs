use bevy::prelude::States;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Title,
    Loading,
    StageTitle,
    Game,
    Ending,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum StageState {
    #[default]
    Stage1,
    Stage2,
    Boss,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum BossState {
    #[default]
    InActive,
    Active,
}

pub const STAGE1_MAP: [&str; 15] = [
    "CAAAAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACA",
    "CAAAAAAABAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABACA",
    "CAAAAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAAAABAAAAAAAAACAAAAAAAAABAAAAAAACCCCCCCAAAAAAAAAAAAAAAACA",
    "CAAAAAAAAAAAAAAAAAACACAAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAACAAACCAAAAAAAAAAAAAAAAAAAAACCCAAACCCAAACCCA",
    "CBAACCCCCAAAAAAAACCCACCCAAAAACAAAAAAAAAAAAAAAAAAAACCAAAAACCAAAAAAAAAACCCAAAAAAABAAAAAAAAAAAAAAAAAACA",
    "CAAAAAAABAAAAAAAACCCAAAAAAAAACAAAAAAAAAAAAABAAAAAAAAAAAAACAAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACA",
    "CACAAAAAAAAACCCCCCCCAABAAAACCCAAACAAAAAAAAAAAAACCAAAAAACACCCAAAAAAAAAAAAAAAAACCCAAAAAAAAAAABAAAAAACA",
    "CACCAAAAAAAAAAAACCCCAAAAAAAAACAAABACABAAAAAAAAAACCAAAAACACAAAACCAAAACCCAAAAAAAAAAAAAAAAAAAAAAAAAAACA",
    "CAACCAAAAAAAAAAACCCCAACCCAAAACAAAAAAACAAAAAAAAAAACCAAAACACAAAAAAAAAAAAAAAABAAAAAAABAAAAAAAAAAAAAAACA",
    "AAAACCCAAAAAABAACCCCAAAAAAAAACAAAAAAABACAAAAAAAAAAAAACCCACAACAAABAAAAAACCCAAAAAAAAAAAAAAAAAAABAAAACA",
    "AAAAAACCCAAAAAAACCCCAABAAAACCCAAAACAAAAAAAAAAABAAAAAAAACACAAAAAAAAAAAAAAAAAAACCCAAAAAAAAAAAAAAAAAACA",
    "AAAAAAAACCCAAAAACCCCAAAACCAAACAAAAAAAAAACAAAAAACCCCCCCCCACCCCCAAAAAAAAAAAAAAAAAAAAAAAAAAABAAAAAAAAAA",
    "AAAAAAAAAAAAAAAACCCCAAAAAAAAACAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAABAACAAAAAABAAAAAAAAAAAAAAAAAAAAAA",
    "CCCCCCCCCCCCCCCCCCCCCCCAAAAAACCCCCCCCCCCCCAACCCCCCCCCCCCCCCCCCCAACAAAACAABACAAAACABAACCCCCCCCCCCCCCC",
    "CCCCCCCCCCCCCCCCCCCCAAAAABAAACCCCCCCCCCCCCAACCCCCCCCCCCCCCCCCCCAACAAAACAAAACAAAACAAAACCCCCCCCCCCCCCC",
];

pub const STAGE2_MAP: [&str; 15] = [
    "CAAAAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAA",
    "CAAAABAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACAAAAAAAAABAAAAAAAAAAA",
    "CAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAAAAAAAAAAAAACAAAAAAACCCCCCCCCCCCCCCCAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAA",
    "CAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAACAAAAACAAAAAAAAAAAAAAAAAAAAAACAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAA",
    "CAAAAAAAACAAAAAAAAAAACAACCCCCCCCCAACCCCCCCAACAAAAAAAAAAAAAAAAAAAAAACAAAAAAABAACAAAAAAAAAAAABAAAAAAAA",
    "CAAAAAAAAAAAAAAAAAAAACAAAAAAAAAACCCCAAAAAAAACCCCCCCCCCCCCCCAAAAAAAACAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAA",
    "CAAAACAAAABACAAABACAACAAAAAAAAAAACAAAAAAAAAACAAAAAAAAAAAAAAAACCAAAACAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAA",
    "CAAAAAAAAAAAAAAAAAAAACAACCCCCCCAACAAACCCCCCCCAAAAAAAAAAAAAAAAAAAAAACAAAAAAAAAACAAAAAACCAAAACCAAAAAAA",
    "CAAAAAACAAAAAAACAAAAACAAAAAAAAAAACCAAAAABAAAAAAAAAAAAAAAAAAAAAAAACACAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAA",
    "CAAAAAAAABAACAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAABCAAAAAAAAAACAAAAAABAAAAAAAAAAAAAA",
    "CAAAAAAAAAAAAAAAAAAAACCCAAAAAAACCAACCAAAAAAAAAAAAAAAAAAAAAAAAAAACAACAAAAAACCAACAAACCAAAACCAAAACCAAAA",
    "AAAAAACAAACAAAAACAAAACAAAACAACAAAAAAAAAAAAAAAAAAAAAAAAAAAAACAAAAAAACAAAAAAABAAAAAAAAAAAAAAAAAAAAAAAA",
    "AAAAAAAAAAAAAAAAAAAAACAAAACCCCAAACCAAAACCAAAAAAAAAAAAACBAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
    "CCCCCCCCCCCCCCCCCCCCCCAAAAAAAAAAAAAAAAAAAAACAAAAACAAAAAAAAAAAAAAAAACCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC",
    "CCCCCCCCCCCCCCCCCCCCCCAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC",
];

#[derive(Clone, Copy, Debug)]
pub struct EnemyPosition {
    pub x: u8,
    pub y: u8,
}

pub const STAGE1_ENEMY_POSITION: [EnemyPosition; 25] = [
    EnemyPosition { x: 6, y: 8 },
    EnemyPosition { x: 5, y: 3 },
    EnemyPosition { x: 27, y: 3 },
    EnemyPosition { x: 22, y: 3 },
    EnemyPosition { x: 12, y: 12 },
    EnemyPosition { x: 28, y: 9 },
    EnemyPosition { x: 35, y: 3 },
    EnemyPosition { x: 40, y: 12 },
    EnemyPosition { x: 37, y: 12 },
    EnemyPosition { x: 24, y: 7 },
    EnemyPosition { x: 45, y: 2 },
    EnemyPosition { x: 58, y: 10 },
    EnemyPosition { x: 62, y: 2 },
    EnemyPosition { x: 59, y: 12 },
    EnemyPosition { x: 50, y: 7 },
    EnemyPosition { x: 72, y: 8 },
    EnemyPosition { x: 78, y: 9 },
    EnemyPosition { x: 69, y: 3 },
    EnemyPosition { x: 78, y: 5 },
    EnemyPosition { x: 79, y: 1 },
    EnemyPosition { x: 90, y: 12 },
    EnemyPosition { x: 94, y: 12 },
    EnemyPosition { x: 95, y: 2 },
    EnemyPosition { x: 96, y: 12 },
    EnemyPosition { x: 93, y: 12 },
];

pub const STAGE2_ENEMY_POSITION: [EnemyPosition; 20] = [
    EnemyPosition { x: 6, y: 12 },
    EnemyPosition { x: 14, y: 12 },
    EnemyPosition { x: 23, y: 12 },
    EnemyPosition { x: 16, y: 2 },
    EnemyPosition { x: 12, y: 12 },
    EnemyPosition { x: 28, y: 3 },
    EnemyPosition { x: 35, y: 9 },
    EnemyPosition { x: 40, y: 3 },
    EnemyPosition { x: 37, y: 3 },
    EnemyPosition { x: 24, y: 6 },
    EnemyPosition { x: 45, y: 2 },
    EnemyPosition { x: 58, y: 1 },
    EnemyPosition { x: 62, y: 1 },
    EnemyPosition { x: 59, y: 1 },
    EnemyPosition { x: 50, y: 7 },
    EnemyPosition { x: 72, y: 12 },
    EnemyPosition { x: 78, y: 12 },
    EnemyPosition { x: 69, y: 12 },
    EnemyPosition { x: 74, y: 5 },
    EnemyPosition { x: 79, y: 12 },
];
