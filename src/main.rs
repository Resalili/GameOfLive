
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
            alive: true,
        }
    } 
}

struct Grid{
    cells:Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid{
        fn new(width: usize, height: usize) -> Self{
        let mut buf: Vec<Cell> = Vec::new(); 
        for y in 0..height+1{
            for x in 0..width+1 {
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

    fn neighbours(&self, cell: Cell) -> Vec<&Cell> {
        let mut buf = Vec::new();
        let (x,y) = (cell.x, cell.y); 
        let offsets = [
            (-1,1), (0,1), (1,1), 
            (-1,0),        (1,0),
            (-1,-1),(0,-1), (1,-1),
        ];
        for (dx,dy) in offsets{
            if let Some(neighbor) = self.get(x as isize + dx, y as isize + dy){
                buf.push(neighbor); 
            }
        }
        buf
    } 
}





const WIDTH: usize = 20;
const HEIGHT: usize = 20;

fn main() {
    let grid = Grid::new(WIDTH,HEIGHT); 
    
}

