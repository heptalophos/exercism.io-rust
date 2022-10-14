#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from(d: u8) -> Direction {
        match (d % 4) as u8 {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => unreachable!("unknown direction")
        }
    }
}

pub struct Robot {
    x:   i32, 
    y:   i32,
    dir: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x: x, y: y, dir: d}
    }

    pub fn turn_right(self) -> Self {
        Robot::new(self.x, self.y, Direction::from(self.dir as u8 + 1))
    }

    pub fn turn_left(self) -> Self {
        Robot::new(self.x, self.y, Direction::from(self.dir as u8 + 3))
    }

    pub fn advance(self) -> Self {
        let (dx, dy) = 
            if self.dir == Direction::from(0) { (0,  1) } else
            if self.dir == Direction::from(1) { (1,  0) } else
            if self.dir == Direction::from(2) { (0, -1) } else
            if self.dir == Direction::from(3) { (-1, 0) } else
            { panic!("can't advance to unknown direction") };
        Robot::new(self.x + dx, self.y + dy, self.dir)
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for instruction in instructions.chars() {
            match instruction {
                'A' => robot = robot.advance(),
                'L' => robot = robot.turn_left(),
                'R' => robot = robot.turn_right(),
                 _  => panic!("Don't know how to do this")
            }
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
