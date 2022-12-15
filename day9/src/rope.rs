use std::collections::HashSet;

#[derive(Debug)]
pub struct Rope {
    head: Position,
    tail: Position,
    visited_positions: HashSet<Position>,
}

impl Rope {
    pub fn new() -> Rope {
        let mut ret = Rope {
            head: Position { x: 0, y: 0 },
            tail: Position { x: 0, y: 0 },
            visited_positions: HashSet::<Position>::new(),
        };

        ret.visited_positions.insert(Position { x: 0, y: 0 });

        ret
    }

    pub fn move_rope(&mut self, direction: Direction, magnitude: u32) {
        // First, compute the new head position
        let mut ii: u32 = 0;

        while ii < magnitude {
            let new_head: Position = match direction {
                Direction::Up => Position {
                    x: self.head.x,
                    y: self.head.y + 1,
                },
                Direction::Down => Position {
                    x: self.head.x,
                    y: self.head.y - 1,
                },
                Direction::Left => Position {
                    x: self.head.x - 1,
                    y: self.head.y,
                },
                Direction::Right => Position {
                    x: self.head.x + 1,
                    y: self.head.y,
                },
            };

            // println!("New head position: ");
            // (&new_head);
            self.head = new_head;

            // We only need to move the tail if it is not adjacent anymore
            if !self.is_adjacent() {
                // We can compute the new tail position by drawing a straight line between the head and the tail, and then moving the tail along that line to the grid point next to the new head position.
                // y = mx + b
                // b = y - mx
                let slope = (self.head.y as f32 - self.tail.y as f32)
                    / (self.head.x as f32 - self.tail.x as f32);
                let y_intercept = self.head.y as f32 - slope * self.head.x as f32;

                // We need to move the tail along the line until it is adjacent to the new head position
                let mut new_tail: Position = self.tail.clone();
                while !self.is_adjacent() {
                    if slope.is_infinite() {
                        println!("Slope is infinite");
                        if self.head.y > self.tail.y {
                            new_tail.y += 1;
                        } else {
                            new_tail.y -= 1;
                        }
                    } else {
                        println!("Slope is finite");
                        // Move the tail along the line until it is adjacent to the new head position
                        if self.head.x > self.tail.x {
                            // Move the tail to the right
                            new_tail.x += 1;
                        } else {
                            // Move the tail to the left
                            new_tail.x -= 1;
                        }

                        // Compute the new y position
                        new_tail.y = (slope * new_tail.x as f32 + y_intercept).round() as i32;
                    }

                    self.tail = new_tail.clone();
                    self.visited_positions.insert(self.tail.clone());
                }

                self.visited_positions.insert(self.tail.clone());
            }

            ii += 1;
        }
    }

    pub fn get_visited_positions_count(self) -> usize {
        self.visited_positions.len()
    }

    fn is_adjacent(&self) -> bool {
        self.compute_distance() <= 1
    }

    fn compute_distance(&self) -> i32 {
        let sqrt: i32 = ((self.head.x - self.tail.x).pow(2) as f32
            + (self.head.y - self.tail.y).pow(2) as f32)
            .sqrt()
            .floor() as i32;

        sqrt
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Position {
    x: i32,
    y: i32,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Test compute_distance
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_distance_2() {
        let rope = Rope {
            head: Position { x: 0, y: 0 },
            tail: Position { x: 0, y: 0 },
            visited_positions: HashSet::<Position>::new(),
        };

        assert_eq!(rope.compute_distance(), 0);
    }

    #[test]
    fn test_compute_distance_3() {
        let rope = Rope {
            head: Position { x: 0, y: 0 },
            tail: Position { x: 0, y: 1 },
            visited_positions: HashSet::<Position>::new(),
        };

        assert_eq!(rope.compute_distance(), 1);
    }

    #[test]
    fn test_compute_distance_4() {
        let rope = Rope {
            head: Position { x: 0, y: 0 },
            tail: Position { x: 1, y: 0 },
            visited_positions: HashSet::<Position>::new(),
        };

        assert_eq!(rope.compute_distance(), 1);
    }

    #[test]
    fn test_compute_distance_5() {
        let rope = Rope {
            head: Position { x: 0, y: 0 },
            tail: Position { x: 1, y: 1 },
            visited_positions: HashSet::<Position>::new(),
        };

        assert_eq!(rope.compute_distance(), 1);
    }

    #[test]
    fn test_compute_distance_diagonal() {
        let rope = Rope {
            head: Position { x: 0, y: 0 },
            tail: Position { x: -1, y: -1 },
            visited_positions: HashSet::<Position>::new(),
        };

        assert_eq!(rope.compute_distance(), 1);
    }
}
