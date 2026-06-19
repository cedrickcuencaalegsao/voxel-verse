/// Chunk dimensions (in blocks)
pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE;

/// World dimensions
pub const RENDER_DISTANCE: i32 = 4; // chunks in each direction

/// Physics
pub const GRAVITY: f32 = 20.0;
pub const PLAYER_SPEED: f32 = 7.0;
pub const JUMP_VELOCITY: f32 = 8.0;
