use std::collections::HashSet;

type BoardIndex = usize;
type Vector = i8;
const BOARD_SIZE:BoardIndex = 8;

#[derive(Eq,Hash,Clone,PartialEq,Debug)]
struct SquareAddress{
    row : BoardIndex,
    col : BoardIndex,
}

#[derive(Eq,Hash,PartialEq,Debug)]
struct Move{
    from : SquareAddress,
    to : SquareAddress,
}

type Moves = HashSet<Move>;

struct MoveVector{
    row_delta : Vector,
    col_delta : Vector,
}



#[derive(Copy,Clone,PartialEq)]
enum Player{
    White,
    Black,
}

#[derive(Copy,Clone,PartialEq)]
enum Piece{
    King,
}

#[derive(Copy,Clone)]
struct PlayerPiece{
    player:Player,
    piece:Piece,
}

struct Board{
    squares: [[Square;BOARD_SIZE];BOARD_SIZE],
}

#[derive(Copy,Clone)]
enum Square{
    Empty,
    PlayerPiece(PlayerPiece),
}

impl SquareAddress{
    fn new(row:BoardIndex,col:BoardIndex)->Self{
        Self{row:row,col:col}
    }

    fn is_valid(&self)->bool{
        Self::is_valid_index(self.row as Vector) &&
        Self::is_valid_index(self.col as Vector)
    }

    fn is_valid_index(i:Vector)->bool{
        i >=0 && i<BOARD_SIZE as Vector
    }

    fn add_vector(&self,v:&MoveVector)->Option<SquareAddress>{
        let row = self.row as Vector + v.row_delta;
        let col = self.col as Vector + v.col_delta;
        if Self::is_valid_index(row) && Self::is_valid_index(col){
            Some(SquareAddress{row:row as BoardIndex,col:col as BoardIndex})
        } else {
            None
        }
    }
}

impl MoveVector{
    fn new(row_delta:Vector,col_delta:Vector)->Self{
        Self{row_delta:row_delta,col_delta:col_delta}
    }
}

impl Board{
    fn new()->Self{
        let mut squares = [[Square::Empty;BOARD_SIZE];BOARD_SIZE];
        Board{squares:squares}
    }

    fn get_square(&self,sa:&SquareAddress)->Square{
        assert!(sa.is_valid());
        self.squares[sa.row][sa.col]
    }

    fn set_square(&mut self,sa:&SquareAddress,s:Square){
        self.squares[sa.row][sa.col] = s;
    }

    fn find_from_squares(&self)->Vec<SquareAddress>{
        let mut r = vec![];
        for row in 0..BOARD_SIZE{
            for col in 0..BOARD_SIZE{
                let sa = SquareAddress::new(row,col);
                let sq = self.get_square(&sa);
                if sq.is_white_king(){
                    r.push(sa);
                }
            }
        }
        r
    }

    fn move_vectors()->Vec<MoveVector>{
        let mut r = vec![];
        r.push(MoveVector::new(0,1));
        r.push(MoveVector::new(0,-1));
        r.push(MoveVector::new(1,1));
        r.push(MoveVector::new(1,-1));
        r.push(MoveVector::new(1,0));
        r.push(MoveVector::new(-1,1));
        r.push(MoveVector::new(-1,0));
        r.push(MoveVector::new(-1,-1));
        r
    }

    fn get_moves(&self)->Moves{ 
        let mut moves = HashSet::new();
        // first find squares to move from 
        let from_squares = self.find_from_squares();
        for sa in from_squares{
            for move_vector in Self::move_vectors(){
                let t = sa.add_vector(&move_vector);
                if let Some(ta) = t{
                    moves.insert(Move{from:sa.clone(),to:ta});
                }
            }
        }
        moves
    }
}

impl Square{
    fn new_empty()->Self{
        Self::Empty
    }

    fn new_player_piece(p:PlayerPiece)->Self{
        Self::PlayerPiece(p)
    }
    
    fn is_empty(&self)->bool{
        match self{
            Self::Empty => true,
            _ => false,
        }
    }

    fn is_white_king(&self)->bool{
        match self{
            Self::Empty => false,
            Self::PlayerPiece(p) => {
                p.player == Player::White && p.piece == Piece::King
            },
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_board(){
        let b = Board::new();
        for row in 0..BOARD_SIZE{
            for col in 0..BOARD_SIZE{
                let sa = SquareAddress::new(row,col);
                assert!(b.get_square(&sa).is_empty());
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_illegal_row_index(){
        let b = Board::new();
        let sa = SquareAddress::new(8,0);
        b.get_square(&sa);
    }


    #[test]
    #[should_panic]
    fn test_illegal_col_index(){
        let b = Board::new();
        let sa = SquareAddress::new(0,8);
        b.get_square(&sa);
    }

    #[test]
    fn test_white_king_at_0_0(){
        let mut b = Board::new();
        let wk = PlayerPiece{player:Player::White,piece:Piece::King};
        let sa = SquareAddress::new(0,0);
        assert!(!b.get_square(&sa).is_white_king());
        let s = Square::new_player_piece(wk);
        b.set_square(&sa,s);
        assert!(!b.get_square(&sa).is_empty());
        assert!(b.get_square(&sa).is_white_king());
    }

    #[test]
    fn test_white_king_at_0_0_moves(){
        let mut b = Board::new();
        let wk = PlayerPiece{player:Player::White,piece:Piece::King};
        let sa = SquareAddress::new(0,0);
        let s = Square::new_player_piece(wk);
        b.set_square(&sa,s);
        let mut expected = HashSet::new();
        expected.insert(Move{from:sa.clone(),to:SquareAddress::new(1,1)});
        expected.insert(Move{from:sa.clone(),to:SquareAddress::new(0,1)});
        expected.insert(Move{from:sa.clone(),to:SquareAddress::new(1,0)});
        assert_eq!(b.get_moves(),expected);
    }

    #[test]
    fn test_white_king_at_7_0_moves(){
        let mut b = Board::new();
        let wk = PlayerPiece{player:Player::White,piece:Piece::King};
        let sa = SquareAddress::new(7,0);
        let s = Square::new_player_piece(wk);
        b.set_square(&sa,s);
        let mut expected = HashSet::new();
        expected.insert(Move{from:sa.clone(),to:SquareAddress::new(7,1)});
        expected.insert(Move{from:sa.clone(),to:SquareAddress::new(6,1)});
        expected.insert(Move{from:sa.clone(),to:SquareAddress::new(6,0)});
        assert_eq!(b.get_moves(),expected);

    }
}
