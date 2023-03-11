// TODO: Maybe change Location into a struct and have board a struct too so that the code looks
// more intuitive

type Location = (usize, usize);
type Board = [[Option<Piece>; 8]; 8];

#[derive(Clone, Debug, PartialEq)]
enum Color {
    Black, White
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Self::Black => "Black",
            Self::White => "White",
        }.to_string()
    }
}

impl Color {
    fn color(self: &Self, input: &str) -> String {
        format!("{}{}\x1b[0m",
            match self {
                Self::Black => "\x1b[30;1m",
                Self::White => "\x1b[37;1m",
            },
            input,
        )
    }
}

#[derive(Clone, Debug)]
enum PieceKind {
    Pawn, Knight, Bishop, Rook, Queen, King
}

impl ToString for PieceKind {
    fn to_string(&self) -> String {
        match self {
            PieceKind::Pawn => 'P',
            PieceKind::Knight => 'N',
            PieceKind::Bishop => 'B',
            PieceKind::Rook => 'R',
            PieceKind::Queen => 'Q',
            PieceKind::King => 'K',
        }.to_string()
    }
}


fn get_moves(loc: Location, board: &Board) -> Vec<(i8, i8)> {
    let piece = board[loc.1][loc.0].clone().unwrap();
    match piece.kind {
        PieceKind::Pawn => {
            match piece.color {
                Color::Black => vec![(0,1)],
                Color::White => vec![(0,-1)],
            }
        }
        PieceKind::Knight => {
            let mut rvec = Vec::new();
            for tile in [(-1,-2),(1,-2),(2,-1),(2,1),(1,2),(-1,2),(-2,1),(-2,-1)] {
                let new_loc = (loc.0 as i8 + tile.0, loc.1 as i8 + tile.1);
                if !is_out_of_bounds(new_loc) && (board[new_loc.1 as usize][new_loc.0 as usize].is_none()
                    || board[new_loc.1 as usize][new_loc.0 as usize].clone().unwrap().color != piece.color) {
                    rvec.push(tile);
                }
            }
            rvec
        }
        PieceKind::Bishop => {
            let mut rvec = Vec::new();
            for diag in [(-1,-1),(1,-1),(1,1),(-1,1)] {
                let mut change = diag;
                let mut new_loc = (loc.0 as i8 + change.0, loc.1 as i8 + change.1);
                while !is_out_of_bounds(new_loc) && board[new_loc.1 as usize][new_loc.0 as usize].is_none() {
                    rvec.push(change);
                    change = (change.0 + diag.0, change.1 + diag.1);
                    new_loc = (loc.0 as i8 + change.0, loc.1 as i8 + change.1);
                }
                // If it is the opposite color
                if !is_out_of_bounds(new_loc) && board[new_loc.1 as usize][new_loc.1 as usize].clone().unwrap().color != piece.color {
                    rvec.push(change);
                }
            }
            rvec
        }
        PieceKind::Rook => {
            let mut rvec = Vec::new();
            for dir in [(-1,0),(0,-1),(1,0),(0,1)] {
                let mut change = dir;
                let mut new_loc = (loc.0 as i8 + change.0, loc.1 as i8 + change.1); 
                while !is_out_of_bounds(new_loc) && board[new_loc.1 as usize][new_loc.0 as usize].is_none() {
                    rvec.push(change);
                    change = (change.0 + dir.0, change.1 + dir.1);
                    new_loc = (loc.0 as i8 + change.0, loc.1 as i8 + change.1);
                }
                // If it is the opposite color
                if !is_out_of_bounds(new_loc) && board[new_loc.1 as usize][new_loc.1 as usize].clone().unwrap().color != piece.color {
                    rvec.push(change);
                }
            }
            rvec
        }
        PieceKind::Queen => {
            let mut rvec = Vec::new();
            // Basically both a rook and a bishop
            for diag in [(-1,-1),(1,-1),(1,1),(-1,1)] {
                let mut change = diag;
                let mut new_loc = (loc.0 as i8 + change.0, loc.1 as i8 + change.1);
                while !is_out_of_bounds(new_loc) && board[new_loc.1 as usize][new_loc.0 as usize].is_none() {
                    rvec.push(change);
                    change = (change.0 + diag.0, change.1 + diag.1);
                    new_loc = (loc.0 as i8 + change.0, loc.1 as i8 + change.1);
                }
                // If it is the opposite color
                if !is_out_of_bounds(new_loc) && board[new_loc.1 as usize][new_loc.1 as usize].clone().unwrap().color != piece.color {
                    rvec.push(change);
                }
            }
            for dir in [(-1,0),(0,-1),(1,0),(0,1)] {
                let mut change = dir;
                let mut new_loc = (loc.0 as i8 + change.0, loc.1 as i8 + change.1); 
                while !is_out_of_bounds(new_loc) && board[new_loc.1 as usize][new_loc.0 as usize].is_none() {
                    rvec.push(change);
                    change = (change.0 + dir.0, change.1 + dir.1);
                    new_loc = (loc.0 as i8 + change.0, loc.1 as i8 + change.1);
                }
                // If it is the opposite color
                if !is_out_of_bounds(new_loc) && board[new_loc.1 as usize][new_loc.1 as usize].clone().unwrap().color != piece.color {
                    rvec.push(change);
                }
            }
            rvec
        }
        PieceKind::King => {
            let mut rvec = Vec::new();
            for tile in [(-1,0),(-1,-1),(0,-1),(1,-1),(1,0),(1,1),(0,1),(-1,1)] {
                let new_loc = (loc.0 as i8 + tile.0, loc.1 as i8 + tile.1);
                if !is_out_of_bounds(new_loc) && (board[new_loc.1 as usize][new_loc.0 as usize].is_none()
                    || board[new_loc.1 as usize][new_loc.0 as usize].clone().unwrap().color != piece.color) {
                    rvec.push(tile);
                }

            }
            rvec
        }
    }
}

#[derive(Clone, Debug)]
struct Piece {
    kind: PieceKind,
    color: Color,
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        self.color.color(&self.kind.to_string())
    }
}

impl Piece {
    fn is_valid_move(self: &Self, from: Location, to: Location, board: &Board) -> bool {
        // Check if the location you want to go to, is not occupied by your own piece
        if let Some(el) = &board[to.1][to.0] {
            if el.color == self.color {
                println!("\x1b[31;1mMoveError\x1b[0m: \x1b[34;1mCannot move to occupied tile \x1b[33;1m{}\x1b[0m\x1b[0m", loc2move(to));
                return false;
            }
        }

        let diff = (to.0 as i8 - from.0 as i8, to.1 as i8 - from.1 as i8);
        let moves = get_moves(from, board);
        moves.contains(&diff)
    }
}

static BOARD: Board = [
    [ Some(Piece { kind: PieceKind::Rook, color: Color::Black }), Some(Piece { kind: PieceKind::Knight, color: Color::Black }), Some(Piece { kind: PieceKind::Bishop, color: Color::Black }), Some(Piece { kind: PieceKind::Queen, color: Color::Black }), Some(Piece { kind: PieceKind::King, color: Color::Black }), Some(Piece { kind: PieceKind::Bishop, color: Color::Black }), Some(Piece { kind: PieceKind::Knight, color: Color::Black }), Some(Piece { kind: PieceKind::Rook, color: Color::Black }) ],
    [ Some(Piece { kind: PieceKind::Pawn, color: Color::Black }), Some(Piece { kind: PieceKind::Pawn, color: Color::Black }), Some(Piece { kind: PieceKind::Pawn, color: Color::Black }), Some(Piece { kind: PieceKind::Pawn, color: Color::Black }), Some(Piece { kind: PieceKind::Pawn, color: Color::Black }), Some(Piece { kind: PieceKind::Pawn, color: Color::Black }), Some(Piece { kind: PieceKind::Pawn, color: Color::Black }), Some(Piece { kind: PieceKind::Pawn, color: Color::Black }), ],
    [ None, None, None, None, None, None, None, None ],
    [ None, None, None, None, None, None, None, None ],
    [ None, None, None, None, None, None, None, None ],
    [ None, None, None, None, None, None, None, None ],
    [ Some(Piece { kind: PieceKind::Pawn, color: Color::White }), Some(Piece { kind: PieceKind::Pawn, color: Color::White }), Some(Piece { kind: PieceKind::Pawn, color: Color::White }), Some(Piece { kind: PieceKind::Pawn, color: Color::White }), Some(Piece { kind: PieceKind::Pawn, color: Color::White }), Some(Piece { kind: PieceKind::Pawn, color: Color::White }), Some(Piece { kind: PieceKind::Pawn, color: Color::White }), Some(Piece { kind: PieceKind::Pawn, color: Color::White }), ],
    [ Some(Piece { kind: PieceKind::Rook, color: Color::White }), Some(Piece { kind: PieceKind::Knight, color: Color::White }), Some(Piece { kind: PieceKind::Bishop, color: Color::White }), Some(Piece { kind: PieceKind::Queen, color: Color::White }), Some(Piece { kind: PieceKind::King, color: Color::White }), Some(Piece { kind: PieceKind::Bishop, color: Color::White }), Some(Piece { kind: PieceKind::Knight, color: Color::White }), Some(Piece { kind: PieceKind::Rook, color: Color::White }) ],
];

fn print_board(board: &Board) {
    for (i, row) in board.iter().enumerate() {
        print!("{} ", 8 - i);
        for el in row {
            match el {
                Some(x) => {
                    print!("{}", x.to_string());
                },
                None => print!(" "),
            }
        }
        println!();
    }
    print!("  ");
    for i in 0..8 {
        print!("{}", ('a' as u8 + i) as char);
    }
    println!();
}

fn show_moves(from: Location, to: Location, board: &Board) {
    let possible_moves = get_moves(from, board);
    if possible_moves.is_empty() {
        println!("\x1b[34;1mThere are no available moves for \x1b[0m{}\x1b[34;1m at \x1b[35;1m{}\x1b[0m",
            board[from.1][from.0].clone().unwrap().to_string(),
            loc2move(from),
        );
    }

    for (i, row) in board.iter().enumerate() {
        print!("{} ", 8 - i);
        for (j, el) in row.iter().enumerate() {
            if (j, i) == from {
                print!("\x1b[34;1m{}\x1b[0m", el.clone().unwrap().kind.to_string());
            } else {
                match el {
                    Some(x) => {
                        if possible_moves.contains(&(j as i8 - from.0 as i8, i as i8 - from.1 as i8)) {
                            print!("\x1b[36;1m{}\x1b[0m", x.kind.to_string());
                        } else if (j, i) == to {
                            print!("\x1b[31;1m{}\x1b[0m", x.kind.to_string());
                        } else {
                            print!("{}", x.to_string());
                        }
                    },
                    None => {
                        if possible_moves.contains(&(j as i8 - from.0 as i8, i as i8 - from.1 as i8)) {
                            print!("\x1b[34;1m*\x1b[0m");
                        } else if (j, i) == to {
                            print!("\x1b[31;1mx\x1b[0m");
                        } else {
                            print!(" ")
                        }
                    },
                }
            }
        }
        println!();
    }
    print!("  ");
    for i in 0..8 {
        print!("{}", ('a' as u8 + i) as char);
    }
    println!();
}

fn move_to(from: Location, to: Location, board: &mut Board) {
    if board[to.1][to.0].is_some() {
        println!("{}\x1b[36;1m has been captured by \x1b[0m{} \x1b[36;1mat \x1b[33;1m{}\x1b[0m",
            board[to.1][to.0].clone().unwrap().to_string(),
            board[from.1][from.0].clone().unwrap().to_string(),
            loc2move(to),
        );
    }

    board[to.1][to.0] = board[from.1][from.0].clone();
    board[from.1][from.0] = None;
    println!("{}\x1b[36;1m was moved from \x1b[33;1m{}\x1b[36;1m to \x1b[33;1m{}\x1b[0m", board[to.1][to.0].clone().unwrap().to_string(), loc2move(from), loc2move(to));
}

fn move2loc(input: &str) -> (i8, i8) {
    (
        input.chars().nth(0).unwrap() as i8 - 'a' as i8,
        8 - (input.chars().nth(1).unwrap() as i8 - '0' as i8),
    )
}

fn debugloc2move(loc: (i8, i8)) -> String {
    format!("{}{}",
        ('a' as u8 + loc.0 as u8) as char,
        8 - loc.1,
    )
}

fn loc2move(loc: Location) -> String {
    format!("{}{}",
        ('a' as u8 + loc.0 as u8) as char,
        8 - loc.1,
    )
}

fn is_out_of_bounds(loc: (i8, i8)) -> bool {
    loc.0 < 0 || loc.1 < 0 || loc.0 > 7 || loc.1 > 7
}

fn main() {
    let mut board = BOARD.clone();
    let mut curr_color = Color::White;
    loop {
        println!("\x1b[35;1m{}\x1b[34;1m is playing right now.\x1b[0m", curr_color.to_string());
        print_board(&board);

        // Get the input
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let comm: Vec<_> = line.split(' ').collect();

        // Check if the supplied arguments are correct
        if comm.len() != 2 {
            println!("Incorrect input! Supplied: {}", line);
            continue;
        }

        // Special commands
        if comm[0] == "help" {
            let loc = move2loc(comm[1]);
            if is_out_of_bounds(loc) {
                println!("\x1b[31;1mInvalidLocationError\x1b[0m: \x1b[34;1mSupplied \x1b[33;1m{}\x1b[34;1m which is outside of the board\x1b[0m", debugloc2move(loc));
                continue;
            }
            show_moves((loc.0 as usize, loc.1 as usize), (9, 9), &board);
            continue;
        }

        // Check if the moves are on the board
        let (from, to) = (move2loc(comm[0]), move2loc(comm[1]));
        if is_out_of_bounds(from) || is_out_of_bounds(to) {
            println!("\x1b[31;1mInvalidLocationError\x1b[0m: \x1b[34;1mSupplied \x1b[33;1m{} \x1b[34;1mto \x1b[33;1m{}, \x1b[34;1mWhich is outside of the board\x1b[0m", debugloc2move(from), debugloc2move(to));
            continue;
        }
        let (from, to) = ((from.0 as usize, from.1 as usize), (to.0 as usize, to.1 as usize));

        // Get the piece on the location if it exists
        let piece = match &board[from.1][from.0] {
            Some(x) => x,
            None => {
                println!("\x1b[31;1mLocationError\x1b[0m: \x1b[34;1mLocation \x1b[33;1m{}\x1b[34;1m Has no piece on it\x1b[0m", loc2move(from));
                continue;
            },
        };

        // Check if the piece is of your own color
        if piece.color != curr_color {
            println!("\x1b[31;1mPlayerError\x1b[0m: \x1b[35;1m{}\x1b[34;1m Is playing right now, thus cannot move \x1b[35;1m{}\x1b[34;1m Piece\x1b[0m",
                curr_color.to_string(),
                piece.color.to_string(),
            );
            continue;
        }

        // Maybe give back why it cant happen later, and not a boolean
        if !piece.is_valid_move(from, to, &board) {
            println!("\x1b[31;1mInvalidMoveError\x1b[0m:\x1b[34;1m Displaying tried move, and all possible moves from this piece\x1b[0m.");
            show_moves(from, to, &board);
            continue;
        }

        // Move the piece at last
        move_to(from, to, &mut board);

        // Change the player that is playing
        curr_color = match curr_color {
            Color::Black => Color::White,
            Color::White => Color::Black,
        };
    }
}
