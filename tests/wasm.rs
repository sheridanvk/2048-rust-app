use wasm_bindgen_test::*;
// use yew::services::console::ConsoleService;
use yew_app::{make_move, Direction, GameState};

#[wasm_bindgen_test]
fn test_move() {
    let mut test_board = vec![vec!(4, 4, 8, 0); 4];
    make_move(Direction::Left, &mut test_board, &mut GameState::Active);
    assert_eq!(test_board[0][0], 8);
    assert_eq!(test_board[0][1], 8);
    assert!(matches!(test_board[0][2], 0 | 2 | 4));
}

#[wasm_bindgen_test]
fn test_move_no_changes() {
    let mut test_board = vec![vec!(2, 4, 8, 0); 4];
    let board_snapshot = test_board.clone();
    // This board should not be able to make a move to the left, as no tiles would move in that direction
    make_move(Direction::Left, &mut test_board, &mut GameState::Active);
    assert_eq!(test_board, board_snapshot);
    // A move to the right should work as normal (the first row will move all the way to the right; we don't know whether a
    // new tile will pop into that row, so we only check the last 3 elements
    make_move(Direction::Right, &mut test_board, &mut GameState::Active);
    assert_eq!(test_board[0].split_off(1), vec!(2, 4, 8));
}

#[wasm_bindgen_test]
fn test_adds_starter_value() {
    let mut test_board = vec![
        vec![2, 4, 8, 0],
        vec![2, 4, 8, 16],
        vec![2, 4, 8, 16],
        vec![2, 4, 8, 16],
    ];
    make_move(Direction::Right, &mut test_board, &mut GameState::Active);
    // ConsoleService::log(&format!("{:?}", test_board));
    assert!(matches!(test_board[0][0], 2 | 4));
}

#[wasm_bindgen_test]
fn test_game_won() {}

#[wasm_bindgen_test]
fn test_game_lost() {
    // let mut test_board = vec![
    //     vec![8, 16, 8, 16],
    //     vec![16, 8, 16, 8],
    //     vec![8, 16, 8, 16],
    //     vec![16, 8, 16, 8],
    // ];
    // check_game_state();
}
