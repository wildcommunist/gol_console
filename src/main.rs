use crossterm::{cursor, execute, terminal};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use crossterm::terminal::ClearType;

const WIDTH: usize = 140;
const HEIGHT: usize = 75;

fn main() {
    let mut game = Game::new();

    // create a simple methuselah
    game.set_cell(50, 50, true);
    game.set_cell(50, 51, true);
    game.set_cell(49, 51, true);
    game.set_cell(50, 52, true);
    game.set_cell(51, 52, true);

    let sleep_time = Duration::from_millis(125);

    loop {
        // Update the game state and render it to the screen
        game.tick();
        render(&game);
        sleep(sleep_time);
    }
}

struct Game {
    cells: [[bool; WIDTH]; HEIGHT],
}

impl Game {
    fn new() -> Game {
        Game {
            cells: [[false; WIDTH]; HEIGHT],
        }
    }

    fn tick(&mut self) {
        let mut next_cells = [[false; WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let neighbors = self.count_neighbors(x, y);

                next_cells[y][x] = match (self.cells[y][x], neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }

        self.cells = next_cells;
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let x = (x as i32 + dx) as usize;
                let y = (y as i32 + dy) as usize;

                if x < WIDTH && y < HEIGHT && self.cells[y][x] {
                    count += 1;
                }
            }
        }

        count
    }

    fn set_cell(&mut self, x: usize, y: usize, value: bool) {
        self.cells[y][x] = value;
    }
}

fn render(game: &Game) {
    // Clear the screen
    execute!(stdout(), terminal::Clear(ClearType::All)).unwrap();

    // Move the cursor to the top left corner
    execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();

    // Print the game state to the screen
    for bx in 0..=WIDTH {
        write!(stdout(), "#").unwrap();
    }
    writeln!(stdout()).unwrap();
    for row in &game.cells {
        write!(stdout(), "#").unwrap();
        for &cell in row.iter() {
            let ch = if cell { '▓' } else { '░' };
            write!(stdout(), "{}", ch).unwrap();
        }
        write!(stdout(), "#").unwrap();
        writeln!(stdout()).unwrap();
    }
    for bx in 0..=WIDTH {
        write!(stdout(), "#").unwrap();
    }
    writeln!(stdout()).unwrap();

    // Flush the output buffer
    stdout().flush().unwrap();
}