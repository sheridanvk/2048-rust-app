#![recursion_limit = "256"]
extern crate console_error_panic_hook;

use rand::Rng;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::keyboard::{KeyListenerHandle, KeyboardService};
use yew::utils::document;

type Board = Vec<Vec<i32>>;

#[derive(Debug, PartialEq)]
pub enum GameState {
    Active,
    Won,
    Lost,
    WonActive,
}

struct Index {
    board: Board,
    // This key_listener needs to be kept around so that the handler is not removed, but it doesn't get referenced in code,
    // so it appears unused
    key_listener: KeyListenerHandle,
    state: GameState,
}

enum Msg {
    Move(Direction),
    Noop,
}

#[derive(PartialEq)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

fn check_game_state(board: &mut Board, state: &mut GameState) {
    // Scan all the cells at this point in the game.
    // If any of the cells are 2048, the player has won and can choose to start again or keep going
    // If there's no more moves available, the game is lost
    if state == &mut GameState::Active {
        for row in board.iter() {
            for cell in row.iter() {
                if cell == &16i32 {
                    return *state = GameState::Won;
                }
            }
        }
    }
    let mut copied_board = board.clone();
    if maybe_move_tiles(Direction::Left, &mut copied_board)
        | maybe_move_tiles(Direction::Right, &mut copied_board)
        | maybe_move_tiles(Direction::Up, &mut copied_board)
        | maybe_move_tiles(Direction::Down, &mut copied_board)
    {
        if matches!(state, &mut GameState::Won | &mut GameState::WonActive) {
            return *state = GameState::WonActive;
        } else {
            return *state = GameState::Active;
        }
    }
    return *state = GameState::Lost;
}

fn place_starter_value(board: &mut Board) -> bool {
    // Scan all the cells at this point in the game; find the empty ones where a starter value can be placed
    let mut free_cells = Vec::new();
    for (i, row) in board.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell == &0i32 {
                free_cells.push([i, j]);
            }
        }
    }
    let mut rng = rand::thread_rng();
    // Generate a random number, either 2 or 4
    let starter_value = rng.gen_range(1..3) * 2;

    // Generate a random index into the free cell positions, between 0 and and free_cells.length
    let free_cells_index = rng.gen_range(0..free_cells.len());
    let cell_position = free_cells[free_cells_index];

    // set the selected cell to the randomly generated starter value
    board[cell_position[0]][cell_position[1]] = starter_value;
    return true;
}

// This function assumes a left swipe/keypress, as it's the easiest to calculate conceptially (given that all tile values combine
// at the end of the swipe direction, which here is the beginning of a row).
// Other directions can use this function by transforming the board into this direction before the calculations are done.
fn collect_tiles_to_the_left(transformed_board: &mut Board) {
    let mut row = 0;
    let mut column = 0;
    loop {
        let mut curr_row: Vec<i32> = transformed_board[row]
            .iter()
            .cloned()
            .filter(|item| item != &0i32)
            .collect();
        loop {
            if column + 1 >= curr_row.len() {
                column = 0;
                break;
            }
            if curr_row[column] == curr_row[column + 1] {
                curr_row[column] = curr_row[column] * 2;
                curr_row.remove(column + 1);
                column += 1;
            } else {
                column += 1;
            }
        }
        loop {
            if curr_row.len() < 4 {
                curr_row.push(0);
            } else {
                break;
            }
        }
        transformed_board[row] = curr_row;
        row += 1;
        if row > 3 {
            break;
        }
    }
}

fn transpose_board(board: &mut Board) {
    let mut transformed_board = vec![];
    let mut column = 0;
    loop {
        let mut transformed_column = vec![];
        board
            .iter()
            .for_each(|row| transformed_column.push(row[column]));
        transformed_board.push(transformed_column);
        column += 1;
        if column == 4 {
            break;
        }
    }
    *board = transformed_board;
}

fn reverse_board(board: &mut Board) {
    for row in board.iter_mut() {
        row.reverse();
    }
}

// TODO: set game state to lost when it's lost
fn maybe_move_tiles(direction: Direction, board: &mut Board) -> bool {
    let old_board = &mut board.clone();
    match direction {
        Direction::Left => {
            // This direction matches the function, so we don't have to transform the board
            collect_tiles_to_the_left(board);
        }
        Direction::Right => {
            // We need to flip the board along the rows
            reverse_board(board);
            collect_tiles_to_the_left(board);
            reverse_board(board);
        }
        Direction::Up => {
            // Here we need to create new vectors from the columns of the board, and pass those in as the new board
            transpose_board(board);
            collect_tiles_to_the_left(board);
            transpose_board(board);
        }
        Direction::Down => {
            // Here we create new vectors from the columns of the board
            // We also have to reverse them.
            transpose_board(board);
            reverse_board(board);
            collect_tiles_to_the_left(board);
            // Repeat the steps backwards to reset the board
            reverse_board(board);
            transpose_board(board);
        }
    }
    if old_board == board {
        return false;
    } else {
        return true;
    }
}

pub fn make_move(direction: Direction, board: &mut Board, state: &mut GameState) -> bool {
    if maybe_move_tiles(direction, board) {
        place_starter_value(board);
        check_game_state(board, state);
        ConsoleService::log(&format!("{:?}", state));
        return true;
    }
    return false;
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        fn initialise_board() -> Board {
            let row = vec![0; 4];
            let mut new_board = vec![row.clone(); 4];
            place_starter_value(&mut new_board);
            place_starter_value(&mut new_board);
            return new_board;
        }

        let board = initialise_board();

        // Create a callback from a component link to handle keypresses
        // Keep it in state so that it doesn't get garbage collected until this component gets removed
        let key_listener = KeyboardService::register_key_down(
            &document(),
            link.callback(|event: KeyboardEvent| match event.key_code() {
                37 => Msg::Move(Direction::Left),
                38 => Msg::Move(Direction::Up),
                39 => Msg::Move(Direction::Right),
                40 => Msg::Move(Direction::Down),
                _ => Msg::Noop,
            }),
        );
        Self {
            // link,
            board,
            key_listener,
            state: GameState::Active,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Move(direction) => return make_move(direction, &mut self.board, &mut self.state),
            Msg::Noop => {}
        }
        return false;
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }
    fn view(&self) -> Html {
        html! {
            <div>
                <div id="board">
                {
                    for self.board.iter().map(|row| {
                        html! {
                                {row.iter().map(|item| html!{<div class={format!("cell cell-{}", item)}>{format!("{}", item)}</div>}).collect::<Html>()}
                        }
                    })
                }
                </div>
                {
                    match self.state {
                        GameState::Lost => html!{<p>{"Sorry you've lost! Refresh to try again"}</p>},
                        GameState::Won => html!{<p>{"You won the game! Keep going or refresh to start again"}</p>},
                        GameState::Active | GameState::WonActive => html!{}
                    }
                }
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    console_error_panic_hook::set_once();
    App::<Index>::new().mount_to_body();
}
