use chess::*;
use chess::Piece::*;
use chess::Team::*;

pub fn color(piece: Piece) -> String {
    use chess::Piece::*;
    let color = match piece {
        Empty => "0",
        Pawn(team) | Rook(team) | Knight(team) | Bishop(team) | King(team) | Queen(team) => {
            match team {
                Team::Black => "232",
                Team::White => "7",
            }
        }
    };
    format!("{}[38;5;{}m", 0o33 as char, color)
}

pub fn print(board: [[Piece; 8]; 8]) {
    for (y, row) in board.iter().enumerate() {
        for (x, piece) in row.iter().enumerate() {
            if (x + y) % 2 == 0 {
                print!("{}[48;5;242m", 0o33 as char);
            } else {
                print!(r"{}[48;5;240m", 0o33 as char);
            }
            print!("{}", color(*piece));
            print!("{}", piece.to_string());
        }
        println!("{}[0m", 0o33 as char);
    }
}

fn main() {
    #[rustfmt::skip]
    let old_board: [[Piece; 8]; 8] = [
        [Knight(Black), Rook(Black), Bishop(Black), Queen(Black), King(Black), Bishop(Black), Rook(Black), Knight(Black)],
        [Empty; 8],
        [Empty; 8],
        [Empty; 8],
        [Empty; 8],
        [Empty; 8],
        [Pawn(White), Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Knight(White), Rook(White), Bishop(White), Queen(White), King(White), Bishop(White), Rook(White), Knight(White)],
    ];

    print(old_board);

    #[rustfmt::skip]
    let new_board: [[Piece; 8]; 8] = [
        [Empty, Rook(Black), Bishop(Black), Queen(Black), King(Black), Bishop(Black), Rook(Black), Knight(Black)],
        [Empty; 8],
        [Empty; 8],
        [Empty; 8],
        [Empty; 8],
        [Empty; 8],
        [Knight(Black), Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Knight(White), Rook(White), Bishop(White), Queen(White), King(White), Bishop(White), Rook(White), Knight(White)],
    ];

    println!();
    print(new_board);

    assert_eq!(
        decipher_move(old_board, new_board, Black),
        Ok(Move(Position(0, 0), Position(0, 6)))
    );
}
