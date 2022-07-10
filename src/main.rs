use rand::Rng;

#[derive(Clone, Copy)]
struct Cell {
    is_open: bool,
    is_marked: bool,
    is_bomb: bool,
}

#[derive(Clone)]
struct Field {
    rows: i32,
    cols: i32,
    cells: Vec<Cell>,
    bombs: i32,
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Field {
    pub fn open(&mut self, cursor: Point) {
        let mut cell = self.get_cell_bound(cursor.x, cursor.y);
        if !cell.is_open{
        cell.is_open = true;
        self.change_cell(cursor.x, cursor.y, cell);
            if 0 == self.get_neightbor_of_cell(cursor.x, cursor.y) {
                for dtrows in 0..3 {
                    for dtcols in 0..3 {
                        let drows = dtrows - 1; let dcols = dtcols -1;
                        let mut new_cursor = Point{x: cursor.x + drows,y: cursor.y + dcols};
                        if new_cursor.x < 0 {new_cursor.x = 0}
                        if new_cursor.y < 0 {new_cursor.y = 0}
                        if new_cursor.x >= self.rows {new_cursor.x = self.rows-1}
                        if new_cursor.x >= self.cols {new_cursor.y = self.cols-1}
                        self.open(new_cursor);
                    }
                }
            }
        if cell.is_bomb {
            self.game_over(false)
        }
        }
    }
    pub fn mark(&mut self, cursor: Point) {
        let mut cell = self.get_cell(cursor.x, cursor.y);
        cell.is_marked = !cell.is_marked;
        self.change_cell(cursor.x, cursor.y, cell);
        let mut bomb_count = 0;
        let mut bomb_and_marked_count = 0;
        for i in 0..self.rows as usize {
            for j in 0..self.cols as usize { 
                let temp_cell = self.get_cell(i as i32, j as i32);
                if temp_cell.is_bomb {
                    bomb_count += 1
                }
                if temp_cell.is_bomb && temp_cell.is_marked {
                    bomb_and_marked_count += 1
                }
            }
        }
        if bomb_and_marked_count == bomb_count {
            self.game_over(true)
        }
    }
    pub fn resize(&mut self, x: i32, y: i32) {
       self.rows = x;
       self.cols = y;
       self.init_cells();
    }
    fn init_cells(&mut self) {
        for _ in 0..(self.rows * self.cols) {
            self.cells.push(Cell{is_marked: false, is_open: false, is_bomb: false})
        }
    }
    pub fn print(&mut self, cursor: Point){
        for i in 0..self.rows {
            for j in 0..self.cols {
                if i == cursor.x && j == cursor.y {
                    print!("[")
                } else {print!(" ")}
                let cell = self.get_cell(i, j);
                if cell.is_open {
                    if cell.is_bomb {
                        print!("¤")
                    } else {
                        let neightbor = self.get_neightbor_of_cell(i, j);
                        if neightbor > 0 {print!("{}", neightbor)}
                        else {print!(" ");}
                    }  
                } else {
                    if cell.is_marked {
                        print!("F");
                    } else {
                        print!(".");
                    }
                }
                if i == cursor.x && j == cursor.y {
                    print!("]")
                } else {print!(" ")}

            }
            print!("\n");
        }
    }
    fn get_neightbor_of_cell(&mut self, row: i32, col: i32) -> i32 {
       let mut neightbors = 0;
       for delta_row in 0..3 {
          for delta_col in 0..3 {
              let drow = delta_row -1;
              let dcol = delta_col -1;
                  let cell = self.get_cell_bound(row+drow, col+dcol);
                  if cell.is_bomb {neightbors +=1 };
              }
          }
       return neightbors;
    }
    fn get_cell_bound(&mut self, x: i32, y: i32) -> Cell {
        if 0 <= x && x < self.rows && 0 <= y && y < self.cols {
            let res = self.get_cell(x, y);
            return res
        }
        else {return Cell{is_bomb: false, is_open: true, is_marked: false}}
    }
    fn get_cell(&mut self, x: i32, y: i32) -> Cell {
        self.cells[(x * self.cols + y) as usize]
    }
    fn change_cell(&mut self, x: i32, y:i32, val: Cell) {
        self.cells[(x * self.cols + y) as usize] = val
    }
    pub fn put_bombs(&mut self, bombs: i32) {
        self.bombs = bombs;
        let mut rng = rand::thread_rng();
        if bombs < self.rows*self.cols {
            let mut i = 0;
            while i < bombs {
               let x = rng.gen_range(0..self.rows);
               let y = rng.gen_range(0..self.cols);
               if !self.get_cell(x, y).is_bomb {
                   self.change_cell(x, y, Cell{is_bomb: true, is_open: false, is_marked: false}); 
                   i += 1;
               }
            }
        }
    }
   
    fn print_all_bombs(&mut self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let mut cell = self.get_cell(i, j);
                if cell.is_bomb {
                    cell.is_open = true;
                    self.change_cell(i, j, cell);
                }
            }
        }
        self.print(Point{x: self.rows/2, y: self.cols/2});
    }
    
    fn game_over(&mut self, won: bool) {
        if won {
            println!("You won !")
        } else {
            self.print_all_bombs();
            println!("You loose ...")
        }
        std::process::exit(0)
    }


}

use console;
use std;
fn main() {
   let mut field: Field = Field {bombs: 0, rows: 0, cols: 0, cells: vec![Cell{is_bomb: false, is_open: false, is_marked: false};0]};
    let mut rows = 10; let mut cols = 10; let mut bombs = 10;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        rows = args[1].parse().unwrap();
    }
    if args.len() > 2 {
        cols = args[2].parse().unwrap();
    }
    if args.len() > 3 {
        bombs = args[3].parse().unwrap();
        if bombs > rows*cols {
            bombs = (((rows * cols) as f32) * 0.10) as i32
        }
    }
    field.resize(rows, cols);
    field.put_bombs(bombs);
    let mut cursor = Point {x: rows/2, y: cols/2};
    
    let stdout = console::Term::buffered_stdout();
    
    field.print(cursor);
   'game_loop: loop {
        print!("\x1B[{}A", rows);
        print!("\x1B[{}D", cols);
        field.print(cursor);
        if let Ok(k) = stdout.read_key() {
            match k{
                console::Key::ArrowLeft => {if cursor.y > 0 {cursor.y -= 1}},
                console::Key::ArrowRight => {if cursor.y < field.cols-1 {cursor.y += 1}},
                console::Key::ArrowUp => {if cursor.x > 0 {cursor.x -= 1}},
                console::Key::ArrowDown => {if cursor.x < field.rows-1 {cursor.x += 1}},
                console::Key::Char(' ') => field.open(cursor),
                console::Key::Char('f') => field.mark(cursor),
                console::Key::Char('q') => break 'game_loop,
                _ => (),
            }
        }
    }          
}


