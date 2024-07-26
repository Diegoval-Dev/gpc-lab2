use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::thread;
use crate::framebuffer::Framebuffer;
use crate::color::Color;

mod framebuffer;
mod color;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn render(framebuffer: &mut Framebuffer, grid: &Vec<Vec<bool>>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if grid[y][x] {
                framebuffer.set_current_color(Color::new(255, 255, 255));
            } else {
                framebuffer.set_current_color(Color::new(0, 0, 0));
            }
            framebuffer.point(x as isize, y as isize);
        }
    }
}

fn count_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dy in [-1, 0, 1].iter().cloned() {
        for dx in [-1, 0, 1].iter().cloned() {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as isize + dx).rem_euclid(WIDTH as isize) as usize;
            let ny = (y as isize + dy).rem_euclid(HEIGHT as isize) as usize;
            if grid[ny][nx] {
                count += 1;
            }
        }
    }
    count
}

fn update(grid: &mut Vec<Vec<bool>>) {
    let mut new_grid = grid.clone();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let neighbors = count_neighbors(grid, x, y);
            if grid[y][x] {
                if neighbors < 2 || neighbors > 3 {
                    new_grid[y][x] = false;
                }
            } else {
                if neighbors == 3 {
                    new_grid[y][x] = true;
                }
            }
        }
    }
    *grid = new_grid;
}

fn initialize_pattern(grid: &mut Vec<Vec<bool>>) {
    // Gosper Glider Gun pattern
    let pattern = vec![
        (1, 5), (1, 6), (2, 5), (2, 6), (11, 5), (11, 6), (11, 7), (12, 4),
        (12, 8), (13, 3), (13, 9), (14, 3), (14, 9), (15, 6), (16, 4), (16, 8),
        (17, 5), (17, 6), (17, 7), (18, 6), (21, 3), (21, 4), (21, 5), (22, 3),
        (22, 4), (22, 5), (23, 2), (23, 6), (25, 1), (25, 2), (25, 6), (25, 7),
        (35, 3), (35, 4), (36, 3), (36, 4)
    ];

    for &(x, y) in pattern.iter() {
        grid[y][x] = true;
    }
}

fn main() {
    let window_width = WIDTH * 8;
    let window_height = HEIGHT * 8;
    let frame_delay = Duration::from_millis(100); // Aproximadamente 10 FPS

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let mut grid = vec![vec![false; WIDTH]; HEIGHT];

    // Inicializar el patrón
    initialize_pattern(&mut grid);

    while window.is_open() {
        // Listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Render
        render(&mut framebuffer, &grid);

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(framebuffer.get_buffer(), WIDTH, HEIGHT)
            .unwrap();

        // Update game state
        update(&mut grid);

        // Sleep to maintain a consistent framerate
        thread::sleep(frame_delay);
    }
}
