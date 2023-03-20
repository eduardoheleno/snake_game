mod snake_node;
mod fruit;

use snake_node::SnakeNode;
use fruit::Fruit;

use std::thread::sleep;
use std::time::Duration;
use std::mem::drop;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;

use rand::Rng;

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

fn generate_random_axis() -> i32 {
    let mut rng = rand::thread_rng();
    let mut random_axis = rng.gen_range(0..=SCREEN_WIDTH);

    while random_axis % 10 != 0 {
        random_axis = rng.gen_range(0..=SCREEN_WIDTH);
    }

    random_axis as i32
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
        let mut fruit: Option<Fruit> = None;
        let mut snake_head = SnakeNode::new();
        let mut frame_counter = 0;

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
            snake_head.frame_action(&mut canvas, &mut event_pump, &mut fruit);

            if fruit.is_some() {
                canvas.set_draw_color(Color::RGB(0, 255, 0));
                canvas.fill_rect(fruit.as_ref().unwrap().rect).unwrap();
            }

            if frame_counter == 40 {
                let random_x = generate_random_axis();
                let random_y = generate_random_axis();

                fruit = Some(Fruit::new(random_x, random_y));
                frame_counter = 0;
            }

            if snake_head.check_head_collision_with_body() {
                drop(snake_head);
                break 'eventloop;
            }

            canvas.present();
            frame_counter += 1;
            sleep(Duration::new(0, 1_000_000_000u32 / 10));
        }
    }
}
