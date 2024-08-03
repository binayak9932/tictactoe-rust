use std::io;

#[derive(Debug, PartialEq, Clone, Copy)] // Add Copy here
enum Player {
    X,
    O,
}

type Board = [[Option<Player>; 3]; 3];

fn create_board() -> Board {
    [[None, None, None], [None, None, None], [None, None, None]]
}

fn print_board(board: &Board) {
    for row in board {
        for cell in row {
            print!(
                "{} ",
                match cell {
                    Some(Player::X) => "X",
                    Some(Player::O) => "O",
                    None => ".",
                }
            );
        }
        println!();
    }
}

fn get_player_move() -> (usize, usize) {
    println!("Enter your move (row, col) separated by a comma (e.g., 0,1):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let coords: Vec<usize> = input
        .trim()
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    (coords[0], coords[1])
}

fn make_move(board: &mut Board, player: Player, (row, col): (usize, usize)) -> bool {
    if board[row][col].is_none() {
        board[row][col] = Some(player);
        true
    } else {
        false
    }
}

fn check_win(board: &Board, player: Player) -> bool {
    // Check rows
    for row in 0..3 {
        if board[row][0] == Some(player)
            && board[row][1] == Some(player)
            && board[row][2] == Some(player)
        {
            return true;
        }
    }

    // Check columns
    for col in 0..3 {
        if board[0][col] == Some(player)
            && board[1][col] == Some(player)
            && board[2][col] == Some(player)
        {
            return true;
        }
    }

    // Check diagonals
    if (board[0][0] == Some(player) && board[1][1] == Some(player) && board[2][2] == Some(player))
        || (board[0][2] == Some(player)
            && board[1][1] == Some(player)
            && board[2][0] == Some(player))
    {
        return true;
    }

    false
}

fn is_draw(board: &Board) -> bool {
    for row in 0..3 {
        for col in 0..3 {
            if board[row][col].is_none() {
                return false; // There are still empty cells
            }
        }
    }
    true // No empty cells left
}

fn main() {
    let mut board = create_board();
    let mut current_player = Player::X;

    loop {
        print_board(&board);

        let (row, col) = get_player_move();

        if !make_move(&mut board, current_player, (row, col)) {
            println!("Invalid move! Try again.");
            continue;
        }

        if check_win(&board, current_player) {
            println!("Player {:?} wins!", current_player);
            break;
        }

        if is_draw(&board) {
            println!("It's a draw!");
            break;
        }

        current_player = match current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }
}
