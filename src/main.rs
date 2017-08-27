use std::fmt;
use std::{thread, time};
use std::io;
use std::io::Write;
// Game of life, implementation by Loan.
// Rules :
// A populated cell with zero or one living cells die
// A populated cell with two or three living cells continue to live
// A dead cell with exactly three cells generate

const GRID_X: usize = 70;
const GRID_Y: usize = 60;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

impl fmt::Display for Cell {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::Alive => write!(f, "O"),
            Cell::Dead => write!(f, " "),
        }
    }
}

struct Game {
   grid: [[Cell; GRID_X]; GRID_Y],

}

impl Game{
    fn new_default() -> Game{
        Game { grid: [[Cell::Dead; GRID_X]; GRID_Y] }
    }

    fn show(&self) {
        for line in self.grid.iter() {
           for column in line.iter() {
               print!("{}", column);
               io::stdout().flush().ok().expect("Could not flush stdout");
           }
           println!();
        }
    }

    fn process(&mut self){
        let mut next = [[Cell::Dead; GRID_X]; GRID_Y];
        for (i, line) in self.grid.iter().enumerate() {
            for (j, column) in line.iter().enumerate() {
                match column {
                    &Cell::Dead => {
                        if self.check_alive(i, j) == 3 {
                            next[i][j] = Cell::Alive;
                        }
                    },

                    &Cell::Alive => {
                        let count = self.check_alive(i, j);
                        match count {
                            0 => next[i][j] = Cell::Dead,
                            1 => next[i][j] = Cell::Dead,
                            2 => next[i][j] = Cell::Alive,
                            3 => next[i][j] = Cell::Alive,
                            4 => next[i][j] = Cell::Dead,
                            _ => next[i][j] = Cell::Dead, // More than 4, surpopulation
                        }
                    },
                    _ => (),
                }
            }
        }
        self.grid = next;
    }

    fn check_alive(&self, line: usize, column: usize) -> u8{
        let mut count:u8 = 0;
        let top_end = GRID_Y-1;
        let top_left = GRID_X-1;

        let option_line : [usize;3] = match line{
            0 => [GRID_Y-1, line, line+1],
            a if a == top_end => [line-1, line, 0],
            _ => [line-1, line, line+1],
        };

       let option_column : [usize;3]= match column {
            0 => [GRID_X-1, column, column+1],
           a if a == top_left => [column-1, column, 0],
           _ => [column-1, column, column+1],
        };

        for (i, checkline) in option_line.iter().enumerate(){
            for (j, checkcolumn) in option_column.iter().enumerate(){
                if i == 1 && j == 1 { // Himself, do not check !
                }else {
                    if self.grid[*checkline][*checkcolumn] == Cell::Alive {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

fn main(){
    let mut game = Game::new_default();
    game.grid[5][3] = Cell::Alive; // The glider
    game.grid[6][3] = Cell::Alive;
    game.grid[7][3] = Cell::Alive;
    game.grid[7][2] = Cell::Alive;
    game.grid[6][1] = Cell::Alive;
    game.show();
    start(game);
}

fn start(mut game: Game){
    let ten_millis = time::Duration::from_millis(99);
    loop{
        thread::sleep(ten_millis);
        game.show();
        game.process(); // make one turn
    }
}
