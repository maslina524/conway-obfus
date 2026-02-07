use std::{thread, time};

fn main() {
    const ROWS: usize = 16;
    const COLS: usize = 8;
    let mut front = [['.'; ROWS]; COLS];
    
    // Glider pattern
    front[0][1] = '#';
    front[1][2] = '#';
    front[2][0] = '#';
    front[2][1] = '#';
    front[2][2] = '#';

    loop {
        // Clear screen
        print!("\x1b[2J\x1b[H");
        
        // Print current state
        for y in 0..COLS {
            for x in 0..ROWS {
                print!("{} ", front[y][x]);
            }
            println!();
        }
        
        // Calculate next generation
        front = next(front);
        
        // Delay
        thread::sleep(time::Duration::from_millis(100));
    }
}

fn next(current: [[char; 16]; 8]) -> [[char; 16]; 8] {
    let mut next_gen = current.clone();
    
    for y in 0..8 {
        for x in 0..16 {
            let mut live_neighbors = 0;
            
            // Check all 8 neighbors with wrap-around
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    
                    let ny = (y as isize + dy).rem_euclid(8) as usize;
                    let nx = (x as isize + dx).rem_euclid(16) as usize;
                    
                    if current[ny][nx] == '#' {
                        live_neighbors += 1;
                    }
                }
            }
            
            // Apply Conway's Game of Life rules
            next_gen[y][x] = match (current[y][x], live_neighbors) {
                ('#', 2) | ('#', 3) => '#',
                ('.', 3) => '#',
                _ => '.'
            };
        }
    }
    
    next_gen
}