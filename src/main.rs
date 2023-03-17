mod snake_node;
use snake_node::SnakeNode;

use std::thread::sleep;
use std::time::Duration;
use std::mem::drop;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;

const SCREEN_WIDTH: u32 = 400;
const SCREEN_HEIGHT: u32 = 400;

const NODE_SIZE: u32 = 10;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn clear_canvas(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Snake game", SCREEN_WIDTH, SCREEN_HEIGHT)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    clear_canvas(&mut canvas);

    'gameloop: loop {
        let mut snake_head = SnakeNode::new();

        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        snake_head.append_new_node();
        'eventloop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'gameloop;
                    },
                    _ => {}
                }
            }
            snake_head.frame_action(&mut canvas, &mut event_pump);

            if snake_head.check_head_collision() {
                drop(snake_head);
                break 'eventloop;
            }

            sleep(Duration::new(0, 1_000_000_000u32 / 10));
        }
    }
}
