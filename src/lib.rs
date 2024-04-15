use std::collections::HashSet;

type BoardIndex = usize;
type Vector = i8;
const BOARD_SIZE:BoardIndex = 8;

#[derive(Eq,Hash,Clone,PartialEq,Debug)]
struct SquareAddress{
    row : BoardIndex,
    col : BoardIndex,
}

#[derive(Clone,Eq,Hash,PartialEq,Debug)]
struct Move{
    from : SquareAddress,
    to : SquareAddress,
}

type Moves = HashSet<Move>;

#[derive(Debug)]
struct ReversibleMove{
    mv : Move,
    org : Square,  // records occupant of mv.to
}

type ReversibleMoves = Vec<ReversibleMove>;

struct MoveVector{
    row_delta : Vector,
    col_delta : Vector,
}


#[derive(Debug,Copy,Clone,PartialEq)]
enum Player{
    White,
    Black,
}

#[derive(Debug,Copy,Clone,PartialEq)]
enum Piece{
    King,
}

#[derive(PartialEq,Debug,Copy,Clone)]
struct PlayerPiece{
    player:Player,
    piece:Piece,
}

struct Board{
    squares: [[Square;BOARD_SIZE];BOARD_SIZE],
    player : Player,
    history: ReversibleMoves,
}

#[derive(PartialEq,Debug,Copy,Clone)]
enum Square{
    Empty,
    PlayerPiece(PlayerPiece),
}

impl Player{
    fn next_player(&self)->Self{
        match self{
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}

impl PlayerPiece{
    fn new(player:Player,piece:Piece)->Self{
        Self{player:player,piece:piece}
    }

    fn is_piece(&self,piece:Piece)->bool{
        self.piece == piece
    }

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

impl Move{
    fn new(from:SquareAddress,to:SquareAddress)->Self{
        Self{from:from,to:to}
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
        let player = Player::White; 
        let history = vec![];
        Board{squares:squares,player:player,history:history}
    }

    fn all_addresses()->Vec<SquareAddress>{
        let mut r = vec![];
        for row in 0..BOARD_SIZE{
            for col in 0..BOARD_SIZE{
                let sa = SquareAddress::new(row,col);
                r.push(sa);
            }
        }
        r
    }

    fn make_move(&mut self,m:Move){
        println!("make_move  {:?}",m);
        let pp = self.get_square(&m.from).get_player_piece().unwrap();

        let org = self.get_square(&m.to);
        let rm = ReversibleMove{mv:m.clone(),org:org};
        self.history.push(rm);

        let s = Square::PlayerPiece(pp);
        self.set_square(&m.to,s);

        let s = Square::Empty;
        self.set_square(&m.from,s);

        self.player = self.player.next_player();
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
        for sa in Self::all_addresses(){
            let sq = self.get_square(&sa);
            if sq.is_player(self.player){
                r.push(sa);
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

    fn unmake_move(&mut self){
        let rm = self.history.pop().unwrap();
        println!("Unmake move {:?}",rm);
        // move piece back to original square
        self.set_square(&rm.mv.from,self.get_square(&rm.mv.to));
        // restore square that was moved to
        self.set_square(&rm.mv.to,rm.org);
    }

    fn moves_into_check(&mut self,m:Move)->bool{
        // does this move place us in check ?
        self.make_move(m);
        // look 1 move ahead for next player -- avoid recursive check
        // checking by passing "false"
        for m in self.get_moves_check(false){
            // if m.to has a king in it, we have moved into check
            if self.get_square(&m.to).is_king(){
                self.unmake_move();
                return true
            }
        }
        self.unmake_move();  
        false
    }

    fn get_moves(&mut self)->Moves{
        // get valid moves, by checking we don't move into check
        self.get_moves_check(true)
    }

    fn get_moves_check(&mut self,check:bool)->Moves{
        // get moves, optionally checking for check 
        let mut moves = HashSet::new();
        // first find squares to move from 
        let from_squares = self.find_from_squares();
        for sa in from_squares{
            for move_vector in Self::move_vectors(){
                let t = sa.add_vector(&move_vector);
                if let Some(ta) = t{
                    let m = Move::new(sa.clone(),ta.clone());
                    if !check || !self.moves_into_check(m){
                        moves.insert(Move::new(sa.clone(),ta));
                    }
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

    fn is_player(&self,player:Player)->bool{
        match self{
            Self::Empty => false,
            Self::PlayerPiece(pp) => pp.player == player,
            }
    }

    fn get_player_piece(&self)->Option<PlayerPiece>{
        match self{
            Self::Empty => None,
            Self::PlayerPiece(pp) => Some(*pp),
        }
    }

    fn is_king(&self)->bool{
        match self{
            Self::Empty => false,
            Self::PlayerPiece(pp) => pp.is_piece(Piece::King),
        }
    }

    fn is_white_king(&self)->bool{
        self.is_player(Player::White) &&
        self.get_player_piece().unwrap().is_piece(Piece::King)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_board(){
        let b = Board::new();
        for sa in Board::all_addresses(){
            assert!(b.get_square(&sa).is_empty());
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
        let wk = PlayerPiece::new(Player::White,Piece::King);
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
        let wk = PlayerPiece::new(Player::White,Piece::King);
        let sa = SquareAddress::new(0,0);
        let s = Square::new_player_piece(wk);
        b.set_square(&sa,s);
        let mut expected = HashSet::new();
        expected.insert(Move::new(sa.clone(),SquareAddress::new(1,1)));
        expected.insert(Move::new(sa.clone(),SquareAddress::new(0,1)));
        expected.insert(Move::new(sa.clone(),SquareAddress::new(1,0)));
        assert_eq!(b.get_moves(),expected);
    }

    fn king_moves_from_7_0()->Moves{
        let mut m = HashSet::new();
        let sa = SquareAddress::new(7,0);
        m.insert(Move::new(sa.clone(),SquareAddress::new(7,1)));
        m.insert(Move::new(sa.clone(),SquareAddress::new(6,1)));
        m.insert(Move::new(sa.clone(),SquareAddress::new(6,0)));
        m
    }

    fn test_king_moves_at_7_0(pp:PlayerPiece){
        let mut b = Board::new();
        b.player = pp.player;
        let s = Square::new_player_piece(pp);
        let sa = SquareAddress::new(7,0);
        b.set_square(&sa,s);
        let expected = king_moves_from_7_0();
        assert_eq!(b.get_moves(),expected);
    }

    #[test]
    fn test_white_king_at_7_0_moves(){
        let wk = PlayerPiece::new(Player::White,Piece::King);
        test_king_moves_at_7_0(wk);
    }

    #[test]
    fn test_black_king_at_7_0_moves(){
        let bk = PlayerPiece::new(Player::Black,Piece::King);
        test_king_moves_at_7_0(bk);
    }

    #[test]
    fn test_make_move(){
        let mut b = Board::new();
        let wk = PlayerPiece::new(Player::White,Piece::King);
        let sa1 = SquareAddress::new(0,0);
        let s1 = Square::PlayerPiece(wk);
        b.set_square(&sa1,s1);
        let m = Move::new(sa1.clone(),SquareAddress::new(1,1));
        b.make_move(m);
        assert!(b.get_square(&sa1).is_empty());
        assert!(b.get_square(&SquareAddress::new(1,1)).is_white_king());
        assert_eq!(b.player,Player::Black);
    }

    #[test]
    fn test_unmake_move(){
        let mut b = Board::new();
    
        let wk = PlayerPiece::new(Player::White,Piece::King);
        let sa1 = SquareAddress::new(0,0);
        let s1 = Square::PlayerPiece(wk);
        b.set_square(&sa1,s1);

        let sa2 = SquareAddress::new(1,1);
        let bk = PlayerPiece::new(Player::Black,Piece::King);
        let s2 = Square::PlayerPiece(bk);
        b.set_square(&sa2,s2);

        let m = Move::new(sa1.clone(),sa2.clone());
        b.make_move(m);
        b.unmake_move();
        // black king should be restored on sa2
        // white king should be back to sa1

        assert_eq!(b.get_square(&sa2),s2);
        assert_eq!(b.get_square(&sa1),s1);
    }
    
    #[test]
    fn test_kings_cannot_move_into_check(){
        let mut b = Board::new();
        let wk = PlayerPiece::new(Player::White,Piece::King);
        let sa1 = SquareAddress::new(0,0);
        let s1 = Square::PlayerPiece(wk);
        b.set_square(&sa1,s1);


        let bk = PlayerPiece::new(Player::Black,Piece::King);
        let sa2 = SquareAddress::new(2,0);
        let s2 = Square::PlayerPiece(bk);
        b.set_square(&sa2,s2);

        let mut expected = HashSet::new();
        expected.insert(Move::new(sa1.clone(),SquareAddress::new(0,1)));

        assert_eq!(b.get_moves(),expected);

    }
}
