use super::NODE_SIZE;

use sdl2::rect::Rect;

pub struct Fruit {
    x: i32,
    y: i32,
    pub rect: Rect
}

impl Fruit {
    pub fn new(x: i32, y: i32) -> Fruit {
        Fruit { x, y, rect: Rect::new(x, y, NODE_SIZE, NODE_SIZE) }
    }
}
