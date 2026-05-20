pub struct Package {
    pub background: &'static [u8],
    pub level_params: LevelParams,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vec2d {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LevelParams {
    pub door: Vec2d,
    pub spawn: Vec2d,
}

pub const PACKAGES: &[Package] = &[Package {
    background: include_bytes!("../assets/level.png"),
    level_params: LevelParams {
        door: Vec2d { x: 69, y: 284 },
        spawn: Vec2d { x: 30, y: 240 },
    },
}];
