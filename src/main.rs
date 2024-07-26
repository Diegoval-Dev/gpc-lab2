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

fn main() {
    let window_width = WIDTH * 8;
    let window_height = HEIGHT * 8;
    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let mut grid = vec![vec![false; WIDTH]; HEIGHT];

    // Patrón inicial
    grid[1][2] = true;
    grid[2][3] = true;
    grid[3][1] = true;
    grid[3][2] = true;
    grid[3][3] = true;

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
