#![no_std]

use core::iter::zip;

#[cfg(test)]
mod tests;

type Board = [[Piece; 8]; 8];

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
    Knight(Team),
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
            Knight(..) => "C",
            Bishop(..) => "B",
            King(..) => "K",
            Queen(..) => "Q",
        }
    }

    pub fn team(self) -> Option<Team> {
        match self {
            Empty => None,
            Pawn(team) | Rook(team) | Knight(team) | Bishop(team) | King(team) | Queen(team) => {
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

pub fn decipher_move(previous: Board, new: Board, to_move: Team) -> Result<Move, MoveError> {
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

    let (moved_piece, removed_piece, moved_from, moved_to, leftover_piece) =
        if moving0.0 == moving1.1 {
            (moving1.1, moving1.0, moving0.2, moving1.2, moving0.1)
        } else if moving0.1 == moving1.0 {
            (moving0.1, moving0.0, moving1.2, moving0.2, moving1.1)
        } else {
            return Err(Indecipherable);
        };

    if moved_piece.team() != Some(to_move) {
        return Err(Indecipherable);
    }
    if leftover_piece != Empty {
        return Err(Indecipherable);
    }
    if removed_piece.team() == Some(to_move) {
        return Err(Indecipherable);
    }

    Ok(
        if opposite.is_some() && opposite.unwrap().0 != removed_piece {
            return Err(Indecipherable);
        } else {
            Move(moved_from, moved_to)
        },
    )
}
