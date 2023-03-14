use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;

const NODE_WIDTH: u32 = 10;
const NODE_HEIGHT: u32 = 10;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}

#[derive(Clone)]
struct SnakeNode {
    x: i32,
    y: i32,
    rect: Rect,
    direction: Direction,
    next_node: Box<Option<SnakeNode>>
}

impl SnakeNode {
    fn new(snake_node: Option<SnakeNode>) -> SnakeNode {
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
                rect: Rect::new(400, 400, NODE_WIDTH, NODE_HEIGHT),
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
            rect: Rect::new(first_position.0, first_position.1, NODE_WIDTH, NODE_HEIGHT),
            direction: node.direction,
            next_node: Box::new(None)
        }
    }

    fn move_node(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.y -= 1
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Snake game", SCREEN_WIDTH, SCREEN_HEIGHT)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let snake_head = SnakeNode::new(None);

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'eventloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'eventloop;
                },
                _ => {}
            }
        }
        snake_head.frame_action(&mut canvas);
    }
}
