#[derive(Debug, Default, Clone)]
pub struct Shape {
    // Points are stored in range [0,size)
    shape: Vec<(i32, i32)>,
    pos: (i32, i32),
    size: (i32, i32),
}

impl Shape {
    fn new_opt(shape: Vec<(i32, i32)>) -> Option<Shape> {
        let x_min = shape.iter().map(|p| p.0).min()?;
        let x_max = shape.iter().map(|p| p.0).max()?;
        let y_min = shape.iter().map(|p| p.1).min()?;
        let y_max = shape.iter().map(|p| p.1).max()?;

        Some(Shape {
            shape: if x_min == 0 && y_min == 0 {
                shape
            } else {
                shape
                    .into_iter()
                    .map(|(x, y)| (x - x_min, y - y_min))
                    .collect()
            },
            pos: (x_min, y_min),
            size: (x_max - x_min, y_max - y_min),
        })
    }
    pub fn new(shape: Vec<(i32, i32)>) -> Shape {
        Shape::new_opt(shape).unwrap_or_default()
    }
}

impl Shape {
    fn center(&self) -> (i32, i32) {
        (self.pos.0 - self.size.0 / 2, self.pos.1 - self.size.1 / 2)
    }
    fn center_at_pos(&self, pos: (i32, i32)) -> (i32, i32) {
        let center = self.center();
        (center.0 + pos.0, center.1 + pos.1)
    }

    pub fn add(&self, board: &mut super::Board, pos: (i32, i32)) -> Option<()> {
        let center = self.center_at_pos(pos);
        self.shape
            .iter()
            .map(|(x, y)| board.set(center.0 + x, center.1 + y, true))
            .collect()
    }
}

pub mod still {
    use super::Shape;

    pub fn block() -> Shape {
        Shape::new(vec![(0, 0), (0, 1), (1, 0), (1, 1)])
    }
    pub fn beehive() -> Shape {
        Shape::new(vec![(0, 1), (1, 0), (1, 2), (2, 0), (2, 2), (3, 1)])
    }
    pub fn loaf() -> Shape {
        Shape::new(vec![(0, 1), (1, 0), (1, 2), (2, 0), (2, 3), (3, 1), (3, 2)])
    }
    pub fn boat() -> Shape {
        Shape::new(vec![(0, 0), (0, 1), (1, 0), (1, 2), (2, 1)])
    }
    pub fn tub() -> Shape {
        Shape::new(vec![(0, 1), (1, 0), (1, 2), (2, 1)])
    }

    pub fn all() -> Vec<Shape> {
        vec![block(), beehive(), loaf(), boat(), tub()]
    }
}

pub mod oscilators {
    use super::Shape;

    pub fn blinker() -> Shape {
        Shape::new(vec![(0, 0), (0, 1), (0, 2)])
    }

    pub fn toad() -> Shape {
        Shape::new(vec![(0, 0), (0, 1), (0, 2), (1, 1), (1, 2), (1, 3)])
    }

    pub fn beacon() -> Shape {
        Shape::new(vec![(0, 0), (0, 1), (1, 0), (2, 3), (3, 2), (3, 3)])
    }

    pub fn pulsar() -> Shape {
        let mut shape = Vec::<(i32, i32)>::default();
        for (x, y) in &[(0, 2), (0, 3), (0, 4), (5, 2), (5, 3), (5, 4)] {
            for (x_move, y_move) in &[(0, 0), (0, 6), (7, 0), (7, 6)] {
                shape.push((x + x_move, y + y_move));
            }
        }
        for (x, y) in &[(2, 0), (3, 0), (4, 0), (2, 5), (3, 5), (4, 5)] {
            for (x_move, y_move) in &[(0, 0), (0, 7), (6, 0), (6, 7)] {
                shape.push((x + x_move, y + y_move));
            }
        }
        Shape::new(shape)
    }

    pub fn penta() -> Shape {
        Shape::new(vec![
            (0, 1),
            (1, 1),
            (2, 0),
            (2, 2),
            (3, 1),
            (4, 1),
            (5, 1),
            (6, 1),
            (7, 0),
            (7, 2),
            (8, 1),
            (9, 1),
        ])
    }

    pub fn all() -> Vec<Shape> {
        vec![blinker(), toad(), beacon(), pulsar(), penta()]
    }
}

pub mod ships {
    use super::Shape;

    pub fn glider() -> Shape {
        Shape::new(vec![(2, 1), (1, 2), (0, 0), (0, 1), (0, 2)])
    }

    pub fn all() -> Vec<Shape> {
        vec![glider()]
    }
}

pub mod curious {
    use super::Shape;

    pub fn r_pentomino() -> Shape {
        Shape::new(vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 0)])
    }

    pub fn diehard() -> Shape {
        Shape::new(vec![(0, 0), (1, 0), (1, 2), (2, 0), (6, 0), (6, 1), (7, 1)])
    }

    pub fn acorn() -> Shape {
        Shape::new(vec![(0, 0), (1, 0), (2, 0), (3, 1), (5, 0), (5, 2), (6, 0)])
    }

    pub fn all() -> Vec<Shape> {
        vec![r_pentomino(), diehard(), acorn()]
    }
}
