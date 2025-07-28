use super::MoveError::*;
use super::*;

macro_rules! board {
    (@ K) => { King(Black) };
    (@ Q) => { Queen(Black) };
    (@ N) => { Knight(Black) };
    (@ B) => { Bishop(Black) };
    (@ R) => { Rook(Black) };
    (@ P) => { Pawn(Black) };
    (@ k) => { King(White) };
    (@ q) => { Queen(White) };
    (@ n) => { Knight(White) };
    (@ b) => { Bishop(White) };
    (@ r) => { Rook(White) };
    (@ p) => { Pawn(White) };
    (@ .) => { Empty };
    ($($piece:tt)*) => {{
        unsafe { core::mem::transmute::<[Piece; 64], [[Piece; 8]; 8]>([$(board!(@ $piece)),*]) }
    }};
}

fn flip_board(mut board: Board) -> Board {
    for piece in board.iter_mut().flatten() {
        *piece = match piece {
            Empty => Empty,
            Pawn(team) => Pawn(team.other()),
            Rook(team) => Rook(team.other()),
            Knight(team) => Knight(team.other()),
            Bishop(team) => Bishop(team.other()),
            King(team) => King(team.other()),
            Queen(team) => Queen(team.other()),
        };
    }
    board
}

macro_rules! check_move {
    ($to_move:ident to move {$($old:tt)* } { $($new:tt)* } $move:expr) => {{
        let old_board = board! { $($old)* };
        let new_board = board! { $($new)* };
        assert_eq!(decipher_move(old_board, new_board, $to_move), $move);
        assert_eq!(decipher_move(flip_board(old_board), flip_board(new_board), $to_move.other()), $move);
    }};
}

#[test]
fn simple_move() {
    check_move! {
        Black to move
        {
        R N B Q K B N R
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        r n b q k b n r
        }{
        . N B Q K B N R
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        R . . . . . . .
        r n b q k b n r
        }
        Ok(Move(Position(0, 0), Position(0, 6)))
    };
}

#[test]
fn making_move() {
    check_move! {
        Black to move
        {
        R N B Q K B N R
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        r n b q k b n r
        }{
        . N B Q K B N R
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        r n b q k b n r
        }
        Err(MakingMove)
    };
}

#[test]
fn take_piece() {
    check_move! {
        Black to move
        {
        R N B Q K B N R
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        p . . . . . . .
        r n b q k b n r
        }{
        . N B Q K B N R
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        R . . . . . . .
        r n b q k b n r
        }
        Ok(Move(Position(0, 0), Position(0, 6)))
    };
}

#[test]
fn invalid_take_piece() {
    check_move! {
        Black to move
        {
        R N B Q K B N R
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        P . . . . . . .
        r n b q k b n r
        }{
        . N B Q K B N R
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        R . . . . . . .
        r n b q k b n r
        }
        Err(Indecipherable)
    };
}

#[test]
fn no_move() {
    check_move! {
        Black to move
        {
        R N B Q K B N R
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        r n b q k b n r
        }{
        R N B Q K B N R
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        r n b q k b n r
        }
        Err(NoMove)
    };
}

#[test]
fn other_team() {
    check_move! {
        White to move
        {
        R N B Q K B N R
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        r n b q k b n r
        }{
        R n B Q K B N R
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        r . b q k b n r
        }
        Ok(Move(Position(1, 7), Position(1, 0)))
    };
}

#[test]
fn random_nonsense() {
    check_move! {
        White to move
        {
        R N B Q K B N R
        N n r q k B N R
        . . . . p . . .
        . n . . . . . .
        . . . q . . k .
        . p . q . P . .
        . . . P P . . .
        r p b Q k b n r
        }{
        R n B Q K B N R
        . . . . . . . .
        p . . q p . K .
        . r . . . . . .
        . r Q . Q . P .
        . r . . . . . .
        . . . . r . N .
        N r B q R b n r
        }
        Err(Indecipherable)
    };
}
