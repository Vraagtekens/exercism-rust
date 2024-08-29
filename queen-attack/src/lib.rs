// const CHESSBOARD: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

#[derive(Debug, PartialEq)]
pub struct ChessPosition{
    // array: Vec<char>
    rank: i32, // rank begins where the white piece starts
    file: i32,
}

#[derive(Debug)]
pub struct Queen{
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        // match (rank, file) {
        //     (0..=7, 0..=7) => Some(Self { rank, file }),
        //     _ => None,
        // }

        if rank < 0 || file < 0 || rank > 7 || file > 7{
            return  None;
        }

        Some(ChessPosition{
            rank,
            file
        })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self{
            position
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        // todo!("Determine if this Queen can attack the other Queen {other:?}");
        if  self.position.rank == other.position.rank ||
            self.position.file == other.position.file ||
            self.position.rank == other.position.file ||
            self.position.file == other.position.rank {
                return true;
        }

        for i in 1..8 {
            let rank = self.position.rank;
            let file = self.position.file;

            // Create possible diagonal positions
            let positions = [
                ChessPosition::new(rank + i, file + i),
                ChessPosition::new(rank + i, file - i),
                ChessPosition::new(rank - i, file - i),
                ChessPosition::new(rank - i, file + i),
            ];

            // Check if any of these positions match the other queen's position
            // By using flatten(), we only compare valid ChessPosition objects, 
            // avoiding any unwrap() calls that could cause panics.
            for pos in positions.iter().flatten() {
                if *pos == other.position {
                    return true;
                }
            }
        }

        false
    }
}
