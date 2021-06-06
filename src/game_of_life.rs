pub mod board;
pub use board::Board;

pub mod shapes;
pub use shapes::Shape;

pub mod rules;

pub struct Game<'a> {
    pub board: Board,
    pub rule: Box<dyn 'a + rules::Rule>,
}

impl<'a> Game<'a> {
    pub fn create<R>(width: usize, height: usize, rule: R) -> Self
    where
        R: 'a + rules::Rule,
    {
        Self {
            board: Board::create(width, height),
            rule: Box::new(rule),
        }
    }

    pub fn create_random<R>(width: usize, height: usize, rule: R) -> Self
    where
        R: 'a + rules::Rule,
    {
        let mut game = Game::create(width, height, rule);
        game.randomize();
        game
    }
}

impl Game<'_> {
    pub fn clear(&mut self) {
        self.board.clear()
    }

    pub fn randomize(&mut self) {
        self.board.randomize()
    }
}

impl Game<'_> {
    pub fn step(&mut self) -> Board {
        let new_board = self.rule.next_board(&self.board);
        std::mem::replace(&mut self.board, new_board)
    }
}
