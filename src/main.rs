use rand::prelude::*;

const BOARD_SIZE: usize = 6;
const BOARD_CELL_COUNT: usize = BOARD_SIZE * BOARD_SIZE;

fn are_indexes_win(board: &[bool], test_indexes: &[usize]) -> bool {
  for i in test_indexes {
    if !board[*i] { return false; }
  }

  true
}

fn is_row_win(board: &[bool], row: usize) -> bool {
  let test_indexes: [usize; BOARD_SIZE] = core::array::from_fn(|i| row * BOARD_SIZE + i);

  are_indexes_win(board, &test_indexes)
}

fn is_column_win(board: &[bool], col: usize) -> bool {
  let test_indexes: [usize; BOARD_SIZE] = core::array::from_fn(|i| i * BOARD_SIZE + col);

  are_indexes_win(board, &test_indexes)
}

fn is_win(board: &[bool]) -> bool {
  for i in 0..BOARD_SIZE-1 {
      if is_row_win(board, i) || is_column_win(board, i) { return true; }
  }

  false
}

fn main() {
    println!("control-c to exit");
    let mut solved_in: [i32; BOARD_CELL_COUNT] = [0; BOARD_CELL_COUNT];
    let mut board_counter = 0;
    loop {
        //fill in slots randomly until we have bingo
        let mut board = [false; BOARD_CELL_COUNT];
        let mut guesses: [usize; BOARD_CELL_COUNT] = core::array::from_fn(|i| i );

        guesses.shuffle(&mut thread_rng());

        for guess_number in 0..guesses.len() {
            let guess = guesses[guess_number];
            board[guess] = true;

            if is_win(&board) {
                solved_in[guess_number] += 1;
                break;
            }
        }

        board_counter += 1;

        if board_counter % 1000 == 0 {
            let percents:  [f32; BOARD_CELL_COUNT] =
              core::array::from_fn(|i| solved_in[i] as f32 * 100.0 / board_counter as f32);

            for val in percents.iter() {
              if *val <= 0.0001 {
                print!("0 ");
              } else {
                print!("{:.1} ", val);
              }
            }
            println!();
        }
    }
}


