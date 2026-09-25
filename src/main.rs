use crossterm::{cursor, terminal, style::{self,Color}, ExecutableCommand, QueueableCommand};
use std::io::{stdout, Write};
#[derive(Copy, Clone, PartialEq)]
enum CellType{
    Empty,
    Sand,
}

impl CellType{
    fn is_alive(&self) -> bool {
        !matches!(self, CellType::Empty)
    }

    fn density(&self) -> u8 {
        match self {
            CellType::Sand => 3,
            CellType::Empty => 0,
        }
    }
    
    fn can_displace(&self, other: &CellType) -> bool {
        self.density() > other.density()
    }

}

#[derive(Clone)]
struct Grid{
    cells:Vec<CellType>,
    width: usize,
    height: usize,
}

impl Grid{
    fn get(&self, x:usize,y:usize) -> Option<&CellType>{
        if  x >= self.width || y >= self.height {
                return None;
        }
        Some(&self.cells[y * self.width + x])
    }

}

fn draw(grid: &Grid) -> std::io::Result<()> {
    let mut out = stdout();

    for y in 0..grid.height {
        out.queue(cursor::MoveTo(0, y as u16))?;
        for x in 0..grid.width {
            let (ch, color) = match grid.get(x, y) {
                Some(CellType::Sand) => ('S', Color::Yellow),
                Some(CellType::Empty) => ('.', Color::DarkGrey),
                None => (' ', Color::Reset),
            };
            out.queue(style::SetForegroundColor(color))?;
            out.queue(style::Print(ch))?;
        }
    }

    out.flush()?;
    Ok(())
}

fn update(grid: &mut Grid) {
    let mut visited = vec![false; grid.width * grid.height];
    for y in 0..grid.height {
        for x in 0..grid.width {
            if visited[y * grid.width + x] { continue; }

            if let Some(cell) = grid.get(x,y) {
                match cell {
                    CellType::Sand => update_sand(grid,&mut visited, x, y),
                    _ => {}
                }
            }
        }
    }
}

fn update_sand(grid: &mut Grid, visited:&mut Vec<bool>, x:usize, y:usize){
    visited[y * grid.width + x] = true;
    let disp = [ (0,1), (-1,1), (1,1) ];
    for (dx, dy) in disp {
        let nx = x as isize + dx;  
        let ny = y as isize + dy;

        if nx < 0 || ny < 0 { continue; }
        let (nx, ny) = (nx as usize, ny as usize);

        if let Some(target) = grid.get(nx,ny) {
            if CellType::Sand.can_displace(target){
                grid.cells[ny * grid.width + nx] = CellType::Sand;
                grid.cells[y * grid.width + x] = CellType::Empty;
                visited[ny * grid.width + nx] = true;
                return;
            }
        }
    }
}

fn spawn_sand(grid: &mut Grid, x: usize, y: usize) {
    if let Some(CellType::Empty) = grid.get(x, y) {
        grid.cells[y * grid.width + x] = CellType::Sand;
    }
}

const WIDTH: usize = 20;
const HEIGHT: usize = 20;

fn main() -> std::io::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    stdout().execute(cursor::Hide)?;
    stdout().execute(terminal::Clear(terminal::ClearType::All))?;

    let mut grid = Grid {
        cells: vec![CellType::Empty; WIDTH * HEIGHT],
        width: WIDTH,
        height: HEIGHT,
    };
    let mut cursor_x : usize = WIDTH / 2;
    let mut cursor_y : usize = 0;


    loop{
        if let Ok(true) = crossterm::event::poll(std::time::Duration::from_millis(0)) {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    crossterm::event::KeyCode::Left => cursor_x = cursor_x.saturating_sub(1),
                    crossterm::event::KeyCode::Right => cursor_x = (cursor_x + 1).min(grid.width - 1),
                    crossterm::event::KeyCode::Up => cursor_y = cursor_y.saturating_sub(1),
                    crossterm::event::KeyCode::Down => cursor_y = (cursor_y + 1).min(grid.height - 1),
                    crossterm::event::KeyCode::Char('s') => spawn_sand(&mut grid, cursor_x, cursor_y),
                    crossterm::event::KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }
        update(&mut grid);
        draw(&grid)?;


        std::thread::sleep(std::time::Duration::from_millis(150));
    }

    crossterm::terminal::disable_raw_mode()?;
    stdout().execute(cursor::Show)?;
    Ok(())
}


