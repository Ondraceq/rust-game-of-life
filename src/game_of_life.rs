pub mod shapes;
pub use shapes::Shape;

#[derive(Debug, Clone)]
pub struct GameOfLife {
    cells: Vec<bool>,
    height: usize,
}

impl GameOfLife {
    pub fn create(width: usize, height: usize) -> GameOfLife {
        GameOfLife {
            cells: vec![false; width * height],
            height,
        }
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = false;
        }
    }

    pub fn randomize(&mut self) {
        for cell in &mut self.cells {
            *cell = rand::random();
        }
    }

    pub fn create_random(width: usize, height: usize) -> GameOfLife {
        let mut game = GameOfLife::create(width, height);
        game.randomize();
        game
    }

    pub fn width(&self) -> usize {
        if self.height == 0 {
            assert_eq!(self.cells.len(), 0);
            return 0;
        }
        assert_eq!(self.cells.len() % self.height, 0);
        self.cells.len() / self.height
    }
    pub fn height(&self) -> usize {
        self.height
    }

    fn get_index<T>(&self, x: T, y: T) -> Option<usize>
    where
        T: std::convert::TryInto<usize>,
    {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        if x >= self.width() {
            return None;
        }
        if y >= self.height() {
            return None;
        }
        Some(x * self.height + y)
    }

    pub fn get<T>(&self, x: T, y: T) -> Option<&bool>
    where
        T: std::convert::TryInto<usize>,
    {
        let index = self.get_index(x, y)?;
        self.cells.get(index)
    }

    pub fn get_mut<T>(&mut self, x: T, y: T) -> Option<&mut bool>
    where
        T: std::convert::TryInto<usize>,
    {
        let index = self.get_index(x, y)?;
        self.cells.get_mut(index)
    }

    pub fn set<T>(&mut self, x: T, y: T, value: bool) -> Option<()>
    where
        T: std::convert::TryInto<usize>,
    {
        *self.get_mut(x, y)? = value;
        Some(())
    }

    pub fn toggle<T>(&mut self, x: T, y: T) -> Option<()>
    where
        T: std::convert::TryInto<usize>,
    {
        let cell = self.get_mut(x, y)?;
        *cell = !*cell;
        Some(())
    }
}

impl<'a> IntoIterator for &'a GameOfLife {
    type Item = &'a [bool];
    type IntoIter = std::slice::ChunksExact<'a, bool>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.as_slice().chunks_exact(self.height)
    }
}

impl<'a> IntoIterator for &'a mut GameOfLife {
    type Item = &'a mut [bool];
    type IntoIter = std::slice::ChunksExactMut<'a, bool>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.as_mut_slice().chunks_exact_mut(self.height)
    }
}

impl GameOfLife {
    pub fn iter(&self) -> <&Self as std::iter::IntoIterator>::IntoIter {
        self.into_iter()
    }
    pub fn iter_mut(&mut self) -> <&mut Self as std::iter::IntoIterator>::IntoIter {
        self.into_iter()
    }
}

fn get_neighbours_count(game: &GameOfLife, x: i32, y: i32) -> u8 {
    assert!(x >= 0);
    assert!(y >= 0);
    assert!((x as usize) < game.width());
    assert!((y as usize) < game.height());

    let mut result = 0;
    for &i in &[-1, 0, 1] {
        for &j in &[-1, 0, 1] {
            if (i, j) == (0, 0) {
                continue;
            }
            if matches!(game.get(x + i, y + j), Some(true)) {
                result += 1;
            }
        }
    }
    result
}

impl GameOfLife {
    pub fn step(&mut self) {
        assert_eq!(self.cells.len() % self.height, 0);
        self.cells = self
            .iter()
            .zip(0..)
            .map(|(row, x)| row.iter().zip((0..).map(move |y| (x, y))))
            .flatten()
            .map(|(value, (x, y))| {
                let neighbours_count = get_neighbours_count(self, x, y);
                matches!((*value, neighbours_count), (true, 2..=3) | (false, 3))
            })
            .collect();
    }
}
