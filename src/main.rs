#[derive(Copy,Clone)]
struct Cell{
    alive: bool,
    x:usize,
    y:usize,
}

impl Cell{
    fn new(x:usize, y:usize) -> Self{
        Self{
            x,
            y,
            alive: false,
        }
    } 
    fn dye(&mut self){
        self.alive = false;
    }
    fn  born(&mut self){
        self.alive = true;
    }    
    fn neighbours<'a>(&self, grid: &'a Grid) -> Vec<&'a Cell> {
        let mut buf = Vec::new();
        let (x,y) = (self.x, self.y); 
        let offsets = [
            (-1,1), (0,1), (1,1), 
            (-1,0),        (1,0),
            (-1,-1),(0,-1), (1,-1),
        ];
        for (dx,dy) in offsets{
            if let Some(neighbor) = grid.get(x as isize + dx, y as isize + dy){
                buf.push(neighbor); 
            }
        }
        buf
    } 

}

#[derive(Clone)]
struct Grid{
    cells:Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid{
        fn new(width: usize, height: usize) -> Self{
        let mut buf: Vec<Cell> = Vec::new(); 
        for y in 0..height {
            for x in 0..width {
                buf.push(Cell::new(x,y))
            }
        }
        Self{    
            width,
            height,
            cells: buf,
        }
    }

    fn get(&self, x:isize,y:isize) -> Option<&Cell>{
        if x < 0 || y < 0 ||
            x >= self.width as isize ||
            y >= self.height as isize {
                return None;
        }
        Some(&self.cells[y as usize * self.width + x as usize])
    }
    fn seed_glider(mut self) -> Self {
        for (x, y) in [(1,0), (2,1), (0,2), (1,2), (2,2)] {
            let idx = y * self.width + x;
            self.cells[idx].born();
        }
        self
    }

}





const WIDTH: usize = 20;
const HEIGHT: usize = 20;

fn main() {
    let mut grid = Grid::new(WIDTH,HEIGHT).seed_glider(); 
    let mut first = true;
    loop{
        draw(&grid,first);
        first = false;
        update(&mut grid);
        std::thread::sleep(std::time::Duration::from_millis(150));
    }
}

fn draw(grid: &Grid, first: bool){
    if !first {
        print!("\x1b[{}A", grid.height);
    }
    for y in 0..grid.height {
        for x in 0..grid.width {
            match grid.get(x as isize,y as isize){
                Some(cell) => { print!("{}",if cell.alive {"X "} else {". "}) }
                None => {}
            }
        }
        println!()
    }
}

fn update(grid: &mut Grid) {
    // считаем соседей по снимку прошлого состояния,
    // чтобы не мешать "новых" и "старых" соседей в одном шаге
    let old = grid.clone();
    for cell in &mut grid.cells {
        let count = cell.neighbours(&old).iter().filter(|n| n.alive).count();
        if count < 2 || count > 3 {
            cell.dye();
        } else if count == 3 {
            cell.born();
        }
    }
}
