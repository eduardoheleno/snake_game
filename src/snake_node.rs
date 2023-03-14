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

#[derive(Clone)]
pub struct SnakeNode {
    x: i32,
    y: i32,
    rect: Rect,
    direction: Direction,
    next_node: Box<Option<SnakeNode>>
}

impl SnakeNode {
    pub fn new(snake_node: Option<SnakeNode>) -> SnakeNode {
        if let Some(mut snake_node) = snake_node {
            if snake_node.next_node.is_none() {
                let new_snake_node = SnakeNode::create_new_node_by_node(&snake_node);
                snake_node.next_node = Box::new(Some(new_snake_node));

                return snake_node;
            } else {
                let mut node_buffer = snake_node.next_node.clone().unwrap();
                while node_buffer.next_node.is_some() {
                    node_buffer = node_buffer.next_node.unwrap();
                }
                
                let new_snake_node = SnakeNode::create_new_node_by_node(&node_buffer);

                node_buffer.next_node = Box::new(Some(new_snake_node));
                return snake_node;
            }
        } else {
            SnakeNode {
                x: 400,
                y: 400,
                rect: Rect::new((SCREEN_WIDTH / 2) as i32, (SCREEN_HEIGHT / 2) as i32, NODE_SIZE, NODE_SIZE),
                direction: Direction::Up,
                next_node: Box::new(None)
            }
        }
    }

    fn create_new_node_by_node(node: &SnakeNode) -> SnakeNode {
        let first_position: (i32, i32) = match node.direction {
            Direction::Up => (node.x - 1, node.y),
            Direction::Down => (node.x + 1, node.y),
            Direction::Right => (node.x, node.y - 1),
            Direction::Left => (node.x, node.y + 1)
        };

        SnakeNode {
            x: first_position.0,
            y: first_position.1,
            rect: Rect::new(first_position.0, first_position.1, NODE_SIZE, NODE_SIZE),
            direction: node.direction,
            next_node: Box::new(None)
        }
    }

    pub fn move_node(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1 * NODE_SIZE as i32,
            Direction::Down => self.y += 1 * NODE_SIZE as i32,
            Direction::Right => self.x += 1 * NODE_SIZE as i32,
            Direction::Left => self.x -= 1 * NODE_SIZE as i32
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

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.rect.reposition((self.x, self.y));
        canvas.fill_rect(self.rect).unwrap();
        canvas.present();

        sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }
}
