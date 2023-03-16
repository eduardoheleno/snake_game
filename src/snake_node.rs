use super::{
    NODE_SIZE,
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    Direction,
    Canvas,
    Window,
    Color,
    clear_canvas
};

use std::thread::sleep;
use std::time::Duration;

use sdl2::rect::Rect;
use sdl2::EventPump;
use sdl2::keyboard::Scancode;

#[derive(Clone, Debug)]
pub struct SnakeNode {
    x: i32,
    y: i32,
    rect: Rect,
    direction: Direction,
    next_node: Option<Box<SnakeNode>>
}

impl SnakeNode {
    pub fn new() -> SnakeNode {
        SnakeNode {
            x: (SCREEN_WIDTH / 2) as i32,
            y: (SCREEN_HEIGHT / 2) as i32,
            rect: Rect::new((SCREEN_WIDTH / 2) as i32, (SCREEN_HEIGHT / 2) as i32, NODE_SIZE, NODE_SIZE),
            direction: Direction::Up,
            next_node: None
        }
    }

    pub fn append_new_node(&mut self) {
        if self.next_node.is_none() {
            let new_snake_node = self.create_new_node_by_self();
            self.next_node = Some(Box::new(new_snake_node));
        } else {
            let mut node_buffer = &mut self.next_node;
            while node_buffer.as_ref().unwrap().next_node.is_some() {
                node_buffer = &mut node_buffer.as_mut().unwrap().next_node;
            }

            let new_snake_node = node_buffer.as_ref().unwrap().create_new_node_by_self();
            node_buffer.as_mut().unwrap().next_node = Some(Box::new(new_snake_node));
        }
    }

    fn create_new_node_by_self(&self) -> SnakeNode {
        let first_position: (i32, i32) = match self.direction {
            Direction::Up => (self.x, self.y + (1 * NODE_SIZE) as i32),
            Direction::Down => (self.x, self.y - (1 * NODE_SIZE) as i32),
            Direction::Right => (self.x - (1 * NODE_SIZE) as i32, self.y),
            Direction::Left => (self.x + (1 * NODE_SIZE) as i32, self.y)
        };

        SnakeNode {
            x: first_position.0,
            y: first_position.1,
            rect: Rect::new(first_position.0, first_position.1, NODE_SIZE, NODE_SIZE),
            direction: self.direction,
            next_node: None
        }
    }

    fn move_node(&mut self) {
        match self.direction {
            Direction::Up => {
                if self.y == 0 {
                    self.y = SCREEN_HEIGHT as i32;
                } else {
                    self.y -= 1 * NODE_SIZE as i32;
                }
            },
            Direction::Down => {
                if self.y == SCREEN_HEIGHT as i32 {
                    self.y = 0;
                } else {
                    self.y += 1 * NODE_SIZE as i32;
                }
            },
            Direction::Right => {
                if self.x == SCREEN_WIDTH as i32 {
                    self.x = 0;
                } else {
                    self.x += 1 * NODE_SIZE as i32;
                }
            },
            Direction::Left => {
                if self.x == 0 {
                    self.x = SCREEN_WIDTH as i32;
                } else {
                    self.x -= 1 * NODE_SIZE as i32;
                }
            }
        }

        if let Some(next_node) = &mut self.next_node {
            next_node.move_node();
        }
    }

    fn draw_node(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.rect.reposition((self.x, self.y));
        canvas.fill_rect(self.rect).unwrap();
        canvas.present();

        if let Some(next_node) = &mut self.next_node {
            next_node.draw_node(canvas);
        }
    }

    fn watch_all_prev_nodes(&mut self) {
        let mut directions: Vec<Direction> = Vec::new();
        let mut node_buffer = &self.clone();

        while node_buffer.next_node.is_some() {
            directions.push(node_buffer.direction);
            node_buffer = node_buffer.next_node.as_ref().unwrap();
        }

        let mut loop_counter = 0;
        let mut next_node_buffer = &mut self.next_node;

        while next_node_buffer.is_some() {
            next_node_buffer.as_mut().unwrap().direction = directions[loop_counter];

            next_node_buffer = &mut next_node_buffer.as_mut().unwrap().next_node;
            loop_counter += 1;
        }
    }

    pub fn frame_action(&mut self, canvas: &mut Canvas<Window>, event_pump: &mut EventPump) {
        for scancode in event_pump.keyboard_state().pressed_scancodes() {
            match scancode {
                Scancode::Up => if self.direction != Direction::Down { self.direction = Direction::Up }
                Scancode::Down => if self.direction != Direction::Up { self.direction = Direction::Down }
                Scancode::Right => if self.direction != Direction::Left { self.direction = Direction::Right }
                Scancode::Left => if self.direction != Direction::Right { self.direction = Direction::Left }
                _ => {}
            }
        }

        clear_canvas(canvas);
        self.move_node();
        self.draw_node(canvas);
        self.watch_all_prev_nodes();
        
        sleep(Duration::new(0, 1_000_000_000u32 / 10));
    }
}

// TODO:
// - fix problem where player can hide in a edge between canvas and the area that teleports the player back to the otherside
