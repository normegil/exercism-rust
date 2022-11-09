#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || file < 0 || rank > 7 || file > 7 {
            return Option::None;
        }
        Option::Some(ChessPosition { rank, file })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let self_file = self.position.file;
        let other_file = other.position.file; 
        let self_rank = self.position.rank;
        let other_rank = other.position.rank; 
        if self_file == other_file || self_rank == other_rank {
            true
        } else {
            let file_diff = self_file - other_file;
            if self_rank == other_rank + file_diff || self_rank == other_rank - file_diff {
                true
            } else {
                false
            }
        }
    }
}
