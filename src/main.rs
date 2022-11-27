use std::env;

#[derive(Debug)]
struct BoardPosition {
    row: usize,
    column: usize,
}
fn is_game_over(board: [[&str; 3]; 3]) -> bool {
    let board_positions_to_check = [
        // rows
        [
            BoardPosition { row: 0, column: 0 },
            BoardPosition { row: 0, column: 1 },
            BoardPosition { row: 0, column: 2 },
        ],
        [
            BoardPosition { row: 1, column: 0 },
            BoardPosition { row: 1, column: 1 },
            BoardPosition { row: 1, column: 2 },
        ],
        [
            BoardPosition { row: 2, column: 0 },
            BoardPosition { row: 2, column: 1 },
            BoardPosition { row: 2, column: 2 },
        ],
        // columns
        [
            BoardPosition { row: 0, column: 0 },
            BoardPosition { row: 1, column: 0 },
            BoardPosition { row: 2, column: 0 },
        ],
        [
            BoardPosition { row: 0, column: 1 },
            BoardPosition { row: 1, column: 1 },
            BoardPosition { row: 2, column: 1 },
        ],
        [
            BoardPosition { row: 0, column: 2 },
            BoardPosition { row: 1, column: 2 },
            BoardPosition { row: 2, column: 2 },
        ],
        // diagonals
        [
            BoardPosition { row: 0, column: 0 },
            BoardPosition { row: 1, column: 1 },
            BoardPosition { row: 2, column: 2 },
        ],
        [
            BoardPosition { row: 0, column: 2 },
            BoardPosition { row: 1, column: 1 },
            BoardPosition { row: 2, column: 0 },
        ],
    ];
    for i in 0..board_positions_to_check.len() {
        let current_positions = &board_positions_to_check[i];
        let mut last_value = "";
        for j in 0..current_positions.len() {
            let current_board_position = &current_positions[j];
            let current_board_value =
                board[current_board_position.row][current_board_position.column];
            if current_board_value.len() <= 0 {
                break;
            }
            if last_value.len() <= 0 {
                last_value = current_board_value;
            } else {
                if last_value == current_board_value {
                    if j == current_positions.len() - 1 {
                        return true;
                    }
                    last_value = current_board_value;
                } else {
                    break;
                }
            }
        }
    }

    return false;
}

fn ai_choose_move(board: &mut [[&str; 3]; 3]) -> () {
    for i in 0..board.len() {
        let row = board[i];
        for j in 0..row.len() {
            let column = row[j];
            if column.len() <= 0 {
                board[i][j] = "O";
                return;
            }
        }
    }
}
fn main() {
    let mut tic_tac_toe_board = [["", "", ""], ["", "", ""], ["", "", ""]];
    println!("{}", is_game_over(tic_tac_toe_board));
    loop {
        if is_game_over(tic_tac_toe_board) {
            break;
        }
        let args: Vec<String> = env::args().collect();
        let user_input_row = args[1].parse::<usize>().unwrap();
        let user_input_column = args[2].parse::<usize>().unwrap();

        for i in 0..tic_tac_toe_board.len() {
            let row = tic_tac_toe_board[i];
            for j in 0..row.len() {
                let column = row[j];
                if j == user_input_column && i == user_input_row {
                    tic_tac_toe_board[i][j] = "X"
                }
            }
        }
        ai_choose_move(&mut tic_tac_toe_board);
        println!("{:?}", tic_tac_toe_board);
    }
}
