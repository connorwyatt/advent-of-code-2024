use std::str::FromStr;

pub(crate) struct Cursor {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
pub(crate) struct OutOfGridError;

pub(crate) struct Grid {
    pub(crate) value: Vec<Vec<char>>,
    pub(crate) cursor: Cursor,
    pub(crate) rows: usize,
    pub(crate) columns: usize,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let row_length = value.len();
        let column_length = value.first().unwrap().len();

        Ok(Grid {
            value,
            cursor: Cursor { x: 0, y: 0 },
            rows: row_length,
            columns: column_length,
        })
    }
}

impl Grid {
    pub(crate) fn get(&self) -> char {
        *self
            .value
            .get(self.cursor.y)
            .unwrap()
            .get(self.cursor.x)
            .unwrap()
    }

    pub(crate) fn set_cursor(&mut self, cursor: Cursor) -> Result<(), OutOfGridError> {
        if cursor.x > (self.columns - 1) || cursor.y > (self.rows - 1) {
            return Err(OutOfGridError);
        }

        self.cursor = cursor;

        Ok(())
    }

    pub(crate) fn move_cursor(&mut self, direction: Direction) -> Result<(), OutOfGridError> {
        let new_cursor = match direction {
            Direction::Up => match self.cursor.y.checked_sub(1) {
                Some(new_y) => Some(Cursor {
                    x: self.cursor.x,
                    y: new_y,
                }),
                _ => None,
            },
            Direction::Right => {
                let new_x = self.cursor.x + 1;
                if new_x > (self.columns - 1) {
                    None
                } else {
                    Some(Cursor {
                        x: new_x,
                        y: self.cursor.y,
                    })
                }
            }
            Direction::Down => {
                let new_y = self.cursor.y + 1;
                if new_y > (self.rows - 1) {
                    None
                } else {
                    Some(Cursor {
                        x: self.cursor.x,
                        y: new_y,
                    })
                }
            }
            Direction::Left => match self.cursor.x.checked_sub(1) {
                Some(new_x) => Some(Cursor {
                    x: new_x,
                    y: self.cursor.y,
                }),
                _ => None,
            },
        };

        let Some(cursor) = new_cursor else {
            return Err(OutOfGridError);
        };

        self.cursor = cursor;

        Ok(())
    }
}
