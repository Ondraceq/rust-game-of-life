pub trait Rule {
    fn next_board(&mut self, board: &super::Board) -> super::Board;
}

pub trait BasicRule {
    fn rule(board: &super::Board, cell: &bool, pos: (i32, i32)) -> bool;
}

impl<T: BasicRule> Rule for T {
    fn next_board(&mut self, board: &super::Board) -> super::Board {
        assert_eq!(board.cells.len() % board.height, 0);
        let rule = |(cell, pos)| Self::rule(board, cell, pos);
        super::Board {
            height: board.height,
            cells: board.indexed_iter().map(rule).collect(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Empty {}

impl Rule for Empty {
    fn next_board(&mut self, board: &super::Board) -> super::Board {
        board.clone()
    }
}

#[derive(Debug, Default)]
pub struct Conway {}

impl BasicRule for Conway {
    fn rule(board: &super::Board, cell: &bool, pos: (i32, i32)) -> bool {
        let neighbours_count = get_neighbours_count::<false>(&board, pos);
        matches!((cell, neighbours_count), (true, 2..=3) | (false, 3))
    }
}

#[derive(Debug, Default)]
pub struct ConwayWrapped {}

impl BasicRule for ConwayWrapped {
    fn rule(board: &super::Board, cell: &bool, pos: (i32, i32)) -> bool {
        let neighbours_count = get_neighbours_count::<true>(&board, pos);
        matches!((cell, neighbours_count), (true, 2..=3) | (false, 3))
    }
}

fn get_neighbours_count<const WRAPPED: bool>(board: &super::Board, (x, y): (i32, i32)) -> u8 {
    let width = board.width() as i32;
    let height = board.height() as i32;
    assert!(x >= 0);
    assert!(y >= 0);
    assert!(x < width);
    assert!(y < height);

    let mut result = 0;
    for &i in &[-1, 0, 1] {
        for &j in &[-1, 0, 1] {
            if (i, j) == (0, 0) {
                continue;
            }
            let &cell = if WRAPPED {
                let x = (x + i + width) % width;
                let y = (y + j + height) % height;
                board.get(x, y).expect("Internal logic error")
            } else {
                let x = x + i;
                let y = y + j;
                board.get(x, y).unwrap_or(&false)
            };
            if cell {
                result += 1;
            }
        }
    }
    result
}
