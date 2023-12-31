use std::{
    fs::File,
    io::Read,
    iter::zip,
    path::{Path, PathBuf},
    str::FromStr,
    time::Duration,
};

use n2t::{pixels_from_bitplane, HackEmulator};

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 512;
const HEIGHT: usize = 256;

fn main() {
    let mut emu = HackEmulator::new(
        r"G:\Coding and Programming\My Projects\VSC\nand_2_tetris\test_files\ch 11\Square".into(),
    );

    emu.cpu.run_until(54, false, false);

    for _ in 0..10_000_000 {
        emu.cpu.step(false, false);
        if emu.cpu.pc == 40283 {
            break;
        }
    }
    emu.cpu.run_until(600000, false, false);

    let blank = emu.get_screen().iter().enumerate().filter(|(_, x)| **x != 0).collect::<Vec<_>>();
    println!("{:?}", blank);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut buffer: Vec<u32> = vec![u32::MAX; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        emu.cpu.run_exact(1_000_000, false, false);
        let screen = &mut emu.cpu.ram[0x4000..0x6000];
        pixels_from_bitplane(screen, &mut buffer);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
