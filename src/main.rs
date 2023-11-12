use rand::prelude::*;
use raylib::prelude::*;

struct Board {
    cells: Vec<i32>,
    size: u8,
    solved: bool,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<i32> for Direction {
    fn from(direction: i32) -> Self {
        match direction {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

pub const BACKGROUND_DARKER: Color = Color {
    r: 11,
    g: 11,
    b: 11,
    a: 255,
};
pub const BACKGROUND: Color = Color {
    r: 22,
    g: 22,
    b: 22,
    a: 255,
};
pub const BACKGROUND_LIGHTER: Color = Color {
    r: 55,
    g: 55,
    b: 55,
    a: 255,
};
pub const TEXT: Color = Color {
    r: 240,
    g: 240,
    b: 240,
    a: 255,
};
pub const BORDER: Color = Color {
    r: 230,
    g: 230,
    b: 230,
    a: 255,
};
pub const MESSAGE_WINDOW_BOUNDS: Rectangle = Rectangle::new(40.0, 140.0, 400.0, 200.0);

impl Board {
    fn new(cells: Vec<i32>, size: u8) -> Board {
        let mut board = Board {
            cells,
            size,
            solved: false,
        };
        board.check_solved();
        board
    }

    fn scramble(&mut self) {
        let mut rng = rand::thread_rng();
        let cells: Vec<i32> = (1..self.size as i32 * self.size as i32 + 1).collect();
        self.cells = cells;

        for _i in 0..20 {
            // to create a random board, we generate a solved board 
            // and then we performance a random number of legal moves
            // if we accidentally create a solved board, we try again (limited to 20 attempts)
            let move_count = rng.gen_range(20..100);

            for _ in 0..move_count {
                self.move_empty(Direction::from(rng.gen_range(0..4)));
            }

            // if we didn't create a solved board, exit
            if !self.solved {
                break;
            }
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        let cell_width = 120;
        let cell_height = 120;

        for (i, cell) in self.cells.iter().enumerate() {
            let x = (i % self.size as usize) as i32 * cell_width;
            let y = (i / self.size as usize) as i32 * cell_height;

            let cell_color = if *cell == 16 {
                BACKGROUND_DARKER
            } else {
                BACKGROUND
            };

            d.draw_rectangle(x, y, cell_width, cell_height, cell_color);
            d.draw_rectangle_lines(x, y, cell_width, cell_height, BORDER);

            if *cell == 16 {
                continue;
            }

            d.draw_text(
                &cell.to_string(),
                x + cell_width / 2 - 10,
                y + cell_height / 2 - 10,
                20,
                TEXT,
            );
        }
    }

    fn check_solved(&mut self) {
        let mut solved = true;

        for (i, cell) in self.cells.iter().enumerate() {
            if *cell != i as i32 + 1 {
                solved = false;
                break;
            }
        }

        self.solved = solved;
    }

    fn get_empty_index(&self) -> usize {
        self.cells.iter().position(|cell| *cell == 16).unwrap()
    }

    fn get_neighbor_index(&self, index: usize, direction: Direction) -> Option<usize> {
        let row = index / self.size as usize;
        let col = index % self.size as usize;

        match direction {
            Direction::Up => {
                if row == 0 {
                    None
                } else {
                    Some((row - 1) * self.size as usize + col)
                }
            }
            Direction::Down => {
                if row == self.size as usize - 1 {
                    None
                } else {
                    Some((row + 1) * self.size as usize + col)
                }
            }
            Direction::Left => {
                if col == 0 {
                    None
                } else {
                    Some(row * self.size as usize + col - 1)
                }
            }
            Direction::Right => {
                if col == self.size as usize - 1 {
                    None
                } else {
                    Some(row * self.size as usize + col + 1)
                }
            }
        }
    }

    fn move_empty(&mut self, direction: Direction) {
        let empty_index = self.get_empty_index();
        let neighbor_index = self.get_neighbor_index(empty_index, direction);

        if let Some(neighbor_index) = neighbor_index {
            self.cells.swap(empty_index, neighbor_index);
        }

        self.check_solved();
    }
}

fn format_window_title(level_index: i32) -> String {
    "15 Puzzle - Level ".to_owned() + &(level_index + 1).to_string()
}

fn main() {
    let mut completed_level_count: i32 = 0;
    let mut board = Board::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16].to_vec(), 4);
    board.scramble();

    let (mut rl, thread) = raylib::init()
        .size(480, 480)
        .title(&format_window_title(completed_level_count))
        .build();

    rl.set_target_fps(30);

    while !rl.window_should_close() {
        // user input
        if board.solved {
            if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                completed_level_count += 1;
                rl.set_window_title(&thread, &format_window_title(completed_level_count));
                board.scramble();
            }
        } else {
            if rl.is_key_pressed(KeyboardKey::KEY_UP) {
                board.move_empty(Direction::Up);
            }

            if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
                board.move_empty(Direction::Down);
            }

            if rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
                board.move_empty(Direction::Left);
            }

            if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
                board.move_empty(Direction::Right);
            }
        }

        // draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BACKGROUND_DARKER);
        board.draw(&mut d);

        if board.solved {
            d.draw_rectangle_rec(MESSAGE_WINDOW_BOUNDS, BACKGROUND_LIGHTER);
            d.draw_rectangle_lines_ex(MESSAGE_WINDOW_BOUNDS, 2.0, BORDER);

            d.draw_text(
                "You win!\nPress [SPACE] to continue",
                MESSAGE_WINDOW_BOUNDS.x as i32 + 10,
                MESSAGE_WINDOW_BOUNDS.y as i32 + 10,
                28,
                Color::WHITE,
            );
        }
    }
}
