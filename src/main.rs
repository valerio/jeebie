#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate sdl2;

mod jeebie;

use jeebie::core::cpu::CPU;
use jeebie::memory::MMU;
use jeebie::cart::Cartridge;

use std::env;
use std::thread;
use std::time::Duration;
use std::error::Error;

use sdl2::render::{Renderer, Texture};
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut cpu = match args.len() {
        1 => {
            CPU::new()
        },
        _ => {
            // ignore other args
            let file_path = &args[1];

            let cart = match Cartridge::new_with_path(file_path) {
                Ok(c) => c,
                Err(e) => {
                    println!("Error: {:?}", e);
                    return
                },
            };

            let mmu = MMU::new_with_rom(&cart);
            CPU::with_mmu(mmu)
        },
    };

    run_emulator(&mut cpu).expect("error during execution");
}

pub fn run_emulator(emulator: &mut CPU) -> Result<(), Box<Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let (width, height) = (160, 144);

    let window = video_subsystem.window("Jeebie", width * 3, height * 3)
        .position_centered()
        .resizable()
        .opengl()
        .build()?;
        
    let mut renderer = window.renderer().build()?;
    let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, width, height)?;
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        // Handle inputs
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {},
            };
        }

        // Execute
        let fb = emulator.exec_one_frame();

        // Draw
        draw_step(&mut renderer, &mut texture, &fb)?;

        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}

fn draw_step(renderer: &mut Renderer, texture: &mut Texture, framebuffer: &[(u8, u8, u8)]) -> Result<(), Box<Error>> {
    renderer.clear();

    let (width, height) = (160, 144);

    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {

        for y in 0..height {
            for x in 0..width {
                let offset = x * 3 + pitch * y;
                let fb_index = x + (y * height);
                let (r, g, b) = framebuffer[fb_index];

                buffer[offset] = r;
                buffer[offset + 1] = g;
                buffer[offset + 2] = b;
            }
        }
    })?;

    renderer.copy(&texture, None, None)?;
    renderer.present();

    Ok(())
}