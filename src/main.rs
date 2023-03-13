use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

enum Directions {
    Up,
    Down,
    Right,
    Left
}

struct SnakeNode {
    x: u16,
    y: u16,
    direction: Directions,
    next_node: Box<Option<SnakeNode>>
}

impl SnakeNode {
    fn new(x: u16, y: u16, direction: Directions, snake_node: Option<SnakeNode>) -> SnakeNode {
        let new_snake_node = SnakeNode { x, y, direction, next_node: Box::new(None) };
        if let Some(mut snake_node) = snake_node {
            if snake_node.next_node.is_none() {
                snake_node.next_node = Box::new(Some(new_snake_node));
                return snake_node;
            } else {
                let mut node_buffer = snake_node.next_node.unwrap();
                while node_buffer.next_node.is_some() {
                    node_buffer = node_buffer.next_node.unwrap();
                }

                node_buffer.next_node = Box::new(Some(new_snake_node));
                return snake_node;
            }
        }

        new_snake_node
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Snake game", 800, 500)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

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
    }
}
