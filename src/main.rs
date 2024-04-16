use std::io::{self, Write};

const PLAYERX: char = 'X';
const PLAYERO: char = 'O';
const BOARD_SIZE: usize = 9;

fn main() {
    let mut board: [char; BOARD_SIZE] = ['.'; BOARD_SIZE];

    let mut current_player = false; // can be either X or O
    let mut num_plays = 0;

    // clear the screen
    print!("\x1b[H\x1b[J");
    io::stdout().flush().expect("couldn't flush stdout!");
    draw_board(board);

    loop {
        let player = if current_player {PLAYERX} else {PLAYERO};

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("couldn't read from stdin");

        let guess: usize = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("type a valid number!");
                continue
            }
        };

        if !is_a_valid_guess(guess, board) {
            println!("try a different place!");
            continue
        }

        board[guess-1] = player;

        // clear the screen
        print!("\x1b[H\x1b[J");
        io::stdout().flush().expect("couldn't flush stdout!");

        draw_board(board);

        if player_won(board) {
            println!("{player} won!");
            break
        }

        current_player = !current_player;
        num_plays += 1;

        if num_plays == 9 {
            println!("draw!");
            break
        }
    }
}


fn is_a_valid_guess(guess: usize, board: [char; BOARD_SIZE]) -> bool {
    if guess < 1 || guess > 9 {return false}

    let char_at_index = board[guess-1]; // rust checks for index out of bounds
    char_at_index != PLAYERX && char_at_index != PLAYERO
}


fn draw_board(board: [char; BOARD_SIZE]) {
    for (index, element) in board.iter().enumerate() {
        if (index+1) % 3 == 0 {
            println!("{}", element);
            continue
        }
        print!("{} ", element);
    }
}


fn player_won(board: [char; BOARD_SIZE]) -> bool {
    let mut win: bool = false;
    for i in 0..3 {
        if !is_player(board[i]) {continue};
        win |= (board[i] == board[i+3] && board[i] == board[i+6]) || 
        ({
            let i = i*3;
            board[i] == board[i+1] && board[i] == board[i+2] && is_player(board[i])
        });
        if win {return true};
    }
    if !is_player(board[4]) {return win}

    win |= (board[0] == board[4] && board[0] == board[8]) || 
        (board[2] == board[4] && board[2] == board[6]);
    win
}

fn is_player(place: char) -> bool {
    place == PLAYERX || place == PLAYERO
}
