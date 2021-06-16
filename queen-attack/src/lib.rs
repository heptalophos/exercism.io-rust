#[derive(Debug)]
pub struct ChessPosition{
    rank: i8,
    file: i8,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition 
}

impl ChessPosition {
    pub fn new(rank: i8, file: i8) -> Option<Self> {
        match (rank, file) {
            (x, y) if (0..=7).contains(&x) && 
                      (0..=7).contains(&y) => 
                Some(Self {rank, file}),
            (_, _) => 
                None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let drank = 
            i8::abs(self.position.rank - other.position.rank);
        let dfile = 
            i8::abs(self.position.file - other.position.file);
        match (drank, dfile) {
            (0, 0) => panic!("This is MY square honey!"),
            (0, _) => true,
            (_, 0) => true,
            (x, y) => x == y
        }
    }
}
