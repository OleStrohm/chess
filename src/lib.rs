//#![no_std]

use core::iter::zip;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Team {
    Black,
    White,
}

impl Team {
    pub fn other(self) -> Team {
        match self {
            Team::Black => White,
            Team::White => Black,
        }
    }
}

use Team::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Piece {
    Empty,
    Pawn(Team),
    Rook(Team),
    Castle(Team),
    Bishop(Team),
    King(Team),
    Queen(Team),
}
use Piece::*;

impl Piece {
    pub fn to_string(self) -> &'static str {
        match self {
            Empty => " ",
            Pawn(..) => "P",
            Rook(..) => "R",
            Castle(..) => "C",
            Bishop(..) => "B",
            King(..) => "K",
            Queen(..) => "Q",
        }
    }

    pub fn team(self) -> Option<Team> {
        match self {
            Empty => None,
            Pawn(team) | Rook(team) | Castle(team) | Bishop(team) | King(team) | Queen(team) => {
                Some(team)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position(pub usize, pub usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Move(pub Position, pub Position);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveError {
    NoMove,
    MakingMove,
    Indecipherable,
}

pub fn decipher_move(
    previous: [[Piece; 8]; 8],
    new: [[Piece; 8]; 8],
    to_move: Team,
) -> Result<Move, MoveError> {
    use MoveError::*;

    let diff = zip(previous, new)
        .enumerate()
        .flat_map(|(y, (prev_row, new_row))| {
            zip(prev_row, new_row)
                .enumerate()
                .map(move |(x, (old_piece, new_piece))| (old_piece, new_piece, Position(x, y)))
        });

    let mut moving = diff.clone().filter(|&(old, new, _)| {
        old != new && (old.team() == Some(to_move) || new.team() == Some(to_move))
    });

    let opposite = diff.clone().filter(|&(old, new, _)| {
        old != new && (old.team() == Some(to_move.other()) || new.team() == Some(to_move.other()))
    });

    if moving.clone().count() == 0 && opposite.clone().count() == 0 {
        return Err(NoMove);
    }

    if moving.clone().count() == 1 && opposite.clone().count() == 0 {
        let moving0 = moving.next().unwrap();
        if moving0.0.team() == Some(to_move) && moving0.1 == Empty {
            return Err(MakingMove);
        } else {
            return Err(Indecipherable);
        }
    }

    if moving.clone().count() != 2 || opposite.clone().count() > 1 {
        return Err(Indecipherable);
    }

    let moving0 = moving.next().unwrap();
    let moving1 = moving.next().unwrap();
    let opposite = moving.next();

    Ok(if moving0.0 == moving1.1 && moving0.1 == Empty {
        if opposite.is_some() && opposite.unwrap().1 != moving0.0 {
            return Err(Indecipherable);
        } else {
            Move(moving0.2, moving1.2)
        }
    } else if moving0.1 == moving1.0 && moving0.0 == Empty {
        if opposite.is_some() && opposite.unwrap().1 != moving0.1 {
            return Err(Indecipherable);
        } else {
            Move(moving1.2, moving0.2)
        }
    } else {
        return Err(Indecipherable);
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::MoveError::*;

    macro_rules! board {
        (@ K) => { King(Black) };
        (@ Q) => { Queen(Black) };
        (@ C) => { Castle(Black) };
        (@ B) => { Bishop(Black) };
        (@ R) => { Rook(Black) };
        (@ P) => { Pawn(Black) };
        (@ k) => { King(White) };
        (@ q) => { Queen(White) };
        (@ c) => { Castle(White) };
        (@ b) => { Bishop(White) };
        (@ r) => { Rook(White) };
        (@ p) => { Pawn(White) };
        (@ .) => { Empty };
        ($($piece:tt)*) => {{
            unsafe { core::mem::transmute::<[Piece; 64], [[Piece; 8]; 8]>([$(board!(@ $piece)),*]) }
        }};
    }

    macro_rules! check_move {
        ({$($old:tt)* } { $($new:tt)* } $move:expr) => {{
            let old_board = board! { $($old)* };
            let new_board = board! { $($new)* };
            assert_eq!(decipher_move(old_board, new_board, Black), $move);
        }};
    }

    #[test]
    fn simple_move_macro() {
        check_move! {
            {
            C R B Q K B R C
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            c r b q k b r c
            }{
            . R B Q K B R C
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            C . . . . . . .
            c r b q k b r c
            }
            Ok(Move(Position(0, 0), Position(0, 6)))
        };
    }

    #[test]
    fn making_move() {
        check_move!{
            {
            C R B Q K B R C
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            c r b q k b r c
            }{
            . R B Q K B R C
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            c r b q k b r c
            }
            Err(MakingMove)
        };
    }
}
