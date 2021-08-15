mod snake;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::messagebox::*;
use snake::{Direction, Snake};
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

const HIGHT: u32 = 600;
const WIDTH: u32 = 780;
const BLOCK_SIZE: u32 = 15;
const HIGHT_BLOCK: u32 = HIGHT / BLOCK_SIZE;
const WIDTH_BLOCK: u32 = WIDTH / BLOCK_SIZE;
const MOVE_BETWEEN_MILLISECOND: u128 = 100;

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Rust Snake", WIDTH, HIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let buttons: Vec<_> = vec![
        ButtonData {
            flags: MessageBoxButtonFlag::RETURNKEY_DEFAULT,
            button_id: 1,
            text: "Ok",
        }
    ];
    // let timer = sdl_context.timer()?;
    let mut snake = Snake::new(WIDTH_BLOCK, HIGHT_BLOCK);
    let mut last_time = get_epoch_ms();
    let mut gameover = false; 
    'mainloop: loop {
        // 获取输入
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => snake.change_direction(Direction::UP),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => snake.change_direction(Direction::DOWN),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => snake.change_direction(Direction::LEFT),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => snake.change_direction(Direction::RIGHT),
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    // 重开
                    if gameover {
                        gameover = false;
                        snake = Snake::new(WIDTH_BLOCK, HIGHT_BLOCK);
                    }
                },
                _ => {}
            }
        }

        // update the game loop here
        let time_now = get_epoch_ms();
        if !gameover && time_now.wrapping_sub(last_time) >= MOVE_BETWEEN_MILLISECOND {
            // 更新状态
            // println!("{} {}", last_time, time_now);
            last_time = time_now;
            snake.take_step();
            // 是否游戏结束
            if snake.check_game_over(){
                gameover = true;
                let _res = show_message_box(
                    MessageBoxFlag::WARNING,
                    buttons.as_slice(),
                    "Game over",
                    r#"Press "r" to restart a new game."#,
                    None,
                    None,
                ).map_err(|e| e.to_string())?;
                // println!("{:?}", res);
                continue;
            }
            // 渲染
            canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
            canvas.clear();
            // 格子
            canvas.set_draw_color(Color::RGBA(200, 200, 200, 255));
            // 横线
            for height in (BLOCK_SIZE..HIGHT).step_by(BLOCK_SIZE as usize) {
                canvas.draw_line(
                    Point::new(0 as i32, height as i32),
                    Point::new(WIDTH as i32, height as i32),
                )?;
            }
            // 竖线
            for width in (BLOCK_SIZE..WIDTH).step_by(BLOCK_SIZE as usize) {
                canvas.draw_line(
                    Point::new(width as i32, 0 as i32),
                    Point::new(width as i32, HIGHT as i32),
                )?;
            }
            // 蛇
            let body = snake.get_body();
            // println!("get_body {:?}", body);
            canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
            for block in body.iter() {
                // println!("block {:?}", block);
                canvas.fill_rect(Rect::new(
                    BLOCK_SIZE as i32 * block.0,
                    BLOCK_SIZE as i32 * block.1,
                    BLOCK_SIZE,
                    BLOCK_SIZE,
                ))?;
            }
            // 食物
            let food = snake.get_food_pos();
            canvas.set_draw_color(Color::RGBA(0, 200, 0, 255));
            canvas.fill_rect(Rect::new(
                BLOCK_SIZE as i32 * food.0,
                BLOCK_SIZE as i32 * food.1,
                BLOCK_SIZE,
                BLOCK_SIZE,
            ))?;
            canvas.present();
        }

        std::thread::sleep(Duration::from_millis(10));
    }
    Ok(())
}