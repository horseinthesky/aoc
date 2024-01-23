use crate::custom_error::AocError;

#[derive(Clone, Debug)]
pub struct Traversal {
    layout: Vec<u8>,
    rows: usize,
    cols: usize,
}

const UP: u8 = 1;
const RIGHT: u8 = 2;
const DOWN: u8 = 4;
const LEFT: u8 = 8;

const SPLITTER_HORIZ: u8 = 0;
const SPLITTER_VERT: u8 = 1;
const MIRROR_FWD: u8 = 2;
const MIRROR_BWD: u8 = 3;
const OPEN: u8 = 4;

impl Traversal {
    pub fn energized(&mut self) -> usize {
        self.traverse(0, 0, RIGHT)
    }

    #[inline(always)]
    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn step(
        &self,
        row: usize,
        col: usize,
        dir: u8,
    ) -> Option<(usize, usize)> {
        match dir {
            UP if row == 0 => None,
            UP => Some((row - 1, col)),
            RIGHT if col == self.cols - 1 => None,
            RIGHT => Some((row, col + 1)),
            DOWN if row == self.rows - 1 => None,
            DOWN => Some((row + 1, col)),
            LEFT if col == 0 => None,
            LEFT => Some((row, col - 1)),
            _ => panic!("Invalid direction"),
        }
    }

    pub fn traverse(
        &mut self,
        mut row: usize,
        mut col: usize,
        mut dir: u8,
    ) -> usize {
        let mut energized = 0;
        loop {
            let idx = self.idx(row, col);
            let curr = self.layout[idx] & 0xF;
            let visited = self.layout[idx] >> 4;
            if dir & visited != 0 {
                break;
            }
            if visited == 0 {
                energized += 1;
            }
            self.layout[idx] |= dir << 4;
            match curr {
                SPLITTER_HORIZ => {
                    if dir == UP || dir == DOWN {
                        if let Some((r, c)) =
                            self.step(row, col, LEFT)
                        {
                            energized +=
                                self.traverse(r, c, LEFT);
                        }
                        dir = RIGHT;
                    }
                }
                SPLITTER_VERT => {
                    if dir == LEFT || dir == RIGHT {
                        if let Some((r, c)) =
                            self.step(row, col, UP)
                        {
                            energized +=
                                self.traverse(r, c, UP);
                        }
                        dir = DOWN;
                    }
                }
                MIRROR_FWD => match dir {
                    UP => dir = RIGHT,
                    RIGHT => dir = UP,
                    DOWN => dir = LEFT,
                    LEFT => dir = DOWN,
                    _ => panic!("Invalid dir"),
                },
                MIRROR_BWD => match dir {
                    UP => dir = LEFT,
                    RIGHT => dir = DOWN,
                    DOWN => dir = RIGHT,
                    LEFT => dir = UP,
                    _ => panic!("Invalid dir"),
                },
                _ => {}
            }
            if let Some((r, c)) = self.step(row, col, dir) {
                row = r;
                col = c;
            } else {
                break;
            }
        }
        energized
    }
}

pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let cols = input.find('\n').unwrap();
    let rows = (input.len() / cols) - 1;

    let layout = input
        .as_bytes()
        .iter()
        .filter(|b| **b != b'\n')
        .map(|b| match *b {
            b'-' => SPLITTER_HORIZ,
            b'|' => SPLITTER_VERT,
            b'/' => MIRROR_FWD,
            b'\\' => MIRROR_BWD,
            b'.' => OPEN,
            _ => panic!("Invalid elem"),
        })
        .collect();

    let mut traversal = Traversal { layout, rows, cols };

    Ok(traversal.energized().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!("46", process(input)?);
        Ok(())
    }
}
