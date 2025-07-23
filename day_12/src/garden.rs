#[derive(Debug)]
pub struct Garden {
    pub(crate) width: usize,
    pub(crate) height: usize,
    cells: Vec<char>,
}

impl Garden {
    pub fn new(garden: String) -> Self {
        let mut length = 0;
        let mut height = 0;
        let mut cells = vec![];
        for c in garden.chars() {
            if c != '\n' {
                length += 1;
                cells.push(c);
            } else {
                height += 1
            }
        }
        let width = length / height;
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        if x < self.width && y < self.height {
            Some(self.cells[y * self.width + x])
        } else {
            None
        }
    }

    pub fn get_neighbor(&self, x: usize, y: usize, dx: isize, dy: isize) -> Option<char> {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
            self.get(nx as usize, ny as usize)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Plot<'a> {
    garden: &'a Garden,
    pub(crate) x: usize,
    pub(crate) y: usize,
    letter: char,
}

impl<'a> Plot<'a> {
    pub fn new(garden: &'a Garden, x: usize, y: usize) -> Self {
        let letter =  match garden.get(x, y) {
            None => panic!("Implementation error, plot does not exist"),
            Some(l) => l
        };
        Self {
            garden,
            x,
            y,
            letter,
        }
    }

    pub fn extends_top(&self) -> bool {
        match self.garden.get_neighbor(self.x, self.y, 0, -1) {
            None => false,
            Some(other_letter) => self.letter == other_letter,
        }
    }
    pub fn extends_top_right(&self) -> bool {
        match self.garden.get_neighbor(self.x, self.y, 1, -1) {
            None => false,
            Some(other_letter) => self.letter == other_letter,
        }
    }
    pub fn extends_right(&self) -> bool {
        match self.garden.get_neighbor(self.x, self.y, 1, 0) {
            None => false,
            Some(other_letter) => self.letter == other_letter,
        }
    }
    pub fn extends_bottom_right(&self) -> bool {
        match self.garden.get_neighbor(self.x, self.y, 1, 1) {
            None => false,
            Some(other_letter) => self.letter == other_letter,
        }
    }
    pub fn extends_bottom(&self) -> bool {
        match self.garden.get_neighbor(self.x, self.y, 0, 1) {
            None => false,
            Some(other_letter) => self.letter == other_letter,
        }
    }
    pub fn extends_bottom_left(&self) -> bool {
        match self.garden.get_neighbor(self.x, self.y, -1, 1) {
            None => false,
            Some(other_letter) => self.letter == other_letter,
        }
    }
    pub fn extends_left(&self) -> bool {
        match self.garden.get_neighbor(self.x, self.y, -1, 0) {
            None => false,
            Some(other_letter) => self.letter == other_letter,
        }
    }
    pub fn extends_top_left(&self) -> bool {
        match self.garden.get_neighbor(self.x, self.y, -1, -1) {
            None => false,
            Some(other_letter) => self.letter == other_letter,
        }
    }
}
