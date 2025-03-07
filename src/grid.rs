#[derive(PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Position {
    x: usize,
    y: usize,
}

// Abstract this out into lib
#[derive(Clone)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>,
    pos: Position,
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Grid {
            grid: Vec::<Vec<T>>::new(),
            pos: Position { x: 0, y: 0 },
        }
    }

    pub fn move_pos(&mut self, dir: &Direction, steps: Option<usize>) -> Result<(), &'static str> {
        // Assert grid has been initialized
        assert!(
            !(self.grid.len() == 0) || !(self.grid[0].len() == 0),
            "Grid is of size 0"
        );

        // Default to 1 step
        let steps = match steps {
            Some(i) => i,
            None => 1,
        };

        let result = match dir {
            Direction::Up => {
                if self.pos.y as i64 - steps as i64 >= 0 {
                    self.pos.y = self.pos.y - steps;
                    Ok(())
                } else {
                    Err("Out of bounds")
                }
            }
            Direction::Down => {
                if self.pos.y + steps < self.grid.len() {
                    self.pos.y = self.pos.y + steps;
                    Ok(())
                } else {
                    Err("Out of bounds")
                }
            }
            Direction::Left => {
                if self.pos.x as i64 - steps as i64 >= 0 {
                    self.pos.x = self.pos.x - steps;
                    Ok(())
                } else {
                    Err("Out of bounds")
                }
            }
            Direction::Right => {
                if self.pos.x + steps < self.grid[0].len() {
                    self.pos.x = self.pos.x + steps;
                    Ok(())
                } else {
                    Err("Out of bounds")
                }
            }
        };

        result
    }

    pub fn peek<'a>(
        &'a mut self,
        dir: &Direction,
        steps: Option<usize>,
    ) -> Result<&'a T, &'static str> {
        // Assert grid has been initialized
        assert!(
            !(self.grid.len() == 0) || !(self.grid[0].len() == 0),
            "Grid is of size 0"
        );

        // Default to 1 step
        let steps = match steps {
            Some(i) => i,
            None => 1,
        };

        let result = match dir {
            Direction::Up => {
                if self.pos.y as i64 - steps as i64 >= 0 {
                    Ok(&self.grid[self.pos.y - steps][self.pos.x])
                } else {
                    Err("Out of bounds")
                }
            }
            Direction::Down => {
                if self.pos.y + steps < self.grid.len() {
                    Ok(&self.grid[self.pos.y + steps][self.pos.x])
                } else {
                    Err("Out of bounds")
                }
            }
            Direction::Left => {
                if self.pos.x as i64 - steps as i64 >= 0 {
                    Ok(&self.grid[self.pos.y][self.pos.x - steps])
                } else {
                    Err("Out of bounds")
                }
            }
            Direction::Right => {
                if self.pos.x + steps < self.grid[0].len() {
                    Ok(&self.grid[self.pos.y][self.pos.x + steps])
                } else {
                    Err("Out of bounds")
                }
            }
        };

        result
    }

    pub fn print_grid<F>(&self, f: F)
    where
        F: Fn(&T) -> char,
    {
        for row in self.grid.iter() {
            let line: Vec<char> = row.iter().map(|x| f(x)).collect();
            let line: String = line.into_iter().collect();
            println!("{line}");
        }
    }
}
