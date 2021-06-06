use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Board {
    pub(super) cells: Vec<bool>,
    pub(super) height: usize,
}

impl Board {
    pub fn create(width: usize, height: usize) -> Board {
        assert!(
            i32::try_from(width).is_ok(),
            "Board size has to be representable by i32"
        );
        assert!(
            i32::try_from(height).is_ok(),
            "Board size has to be representable by i32"
        );
        Board {
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

    pub fn create_random(width: usize, height: usize) -> Board {
        let mut board = Board::create(width, height);
        board.randomize();
        board
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

impl<'a> IntoIterator for &'a Board {
    type Item = &'a [bool];
    type IntoIter = std::slice::ChunksExact<'a, bool>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.as_slice().chunks_exact(self.height)
    }
}

impl<'a> IntoIterator for &'a mut Board {
    type Item = &'a mut [bool];
    type IntoIter = std::slice::ChunksExactMut<'a, bool>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.as_mut_slice().chunks_exact_mut(self.height)
    }
}

impl Board {
    pub fn iter(&self) -> <&Self as std::iter::IntoIterator>::IntoIter {
        self.into_iter()
    }
    pub fn iter_mut(&mut self) -> <&mut Self as std::iter::IntoIterator>::IntoIter {
        self.into_iter()
    }
    pub fn indexed_iter(&self) -> impl '_ + Iterator<Item = (&bool, (i32, i32))> {
        self.iter()
            .zip(0..)
            .map(|(row, x)| row.iter().zip((0..).map(move |y| (x, y))))
            .flatten()
    }
    pub fn indexed_iter_mut(&mut self) -> impl '_ + Iterator<Item = (&mut bool, (i32, i32))> {
        self.iter_mut()
            .zip(0..)
            .map(|(row, x)| row.iter_mut().zip((0..).map(move |y| (x, y))))
            .flatten()
    }
}
