use rand::Rng;
use std::{thread, time, collections::VecDeque};
#[derive(Copy, Clone, Debug)]
struct Life {
    current: bool,
    next: bool,
}
#[derive(Clone, Debug)]
struct LifeRule {
    birth_rule: Vec<u8>,
    survive_rule: Vec<u8>,
    grid_type: u8,
    initial_cells: [i32;2],
}
const ROWS: usize = 62; const COLS: usize = 118; const SIZE: usize = ROWS * COLS; // Board parameters
fn main() {
    let mut game_board: [Life; SIZE]; // Board init
    let mut game_rule: LifeRule = get_rand_rule(); // Choosing random rule from list of predefined rules
    game_board = seed_board(get_rand_cells(&game_rule.initial_cells)); // Seeding board with randomly placed alive cells
    let mut state_array: VecDeque<i32> = VecDeque::with_capacity(50); // State array for stuck/oscillation checking
    let mut iteration_counter: i64 = 1; // Number of iterations
    let mut similar_count = 0; // Amount of similar iterations -- logic below
    let mut color = 17;
    let mut color_iter = 1;
    let mut debug_array_average = 0;
    loop {
        for x in 0..game_board.len() {
            let current_col = x % COLS;
            let current_row = (x - current_col) / COLS;
            let mut count = 0;
            for row_offset in [ROWS - 1, 0, 1].iter().cloned() {
                for col_offset in [COLS - 1, 0, 1].iter().cloned() {
                    // Alter this match statement to achieve different grid types
                    match game_rule.grid_type{
                        0 =>
                        match (current_row, row_offset, col_offset) {
                            (_row, 0, 0) => continue,
                            _ => (),
                        },
                        1 =>
                        match (current_row, row_offset, col_offset) {
                            (_row, 0, 0) => continue,
                            (row, _off_row, col) if row % 2 == 0 && col == COLS - 1 => continue,
                            (row, _off_row, col) if row % 2 == 1 && col == COLS + 1 => continue,
                            _ => (),
                        },
                        _ => panic!(),
                    }
                    let adjacent_row = (current_row + row_offset) % ROWS;
                    let adjacent_col = (current_col + col_offset) % COLS;
                    if game_board[get_array_position(adjacent_row, adjacent_col)].current { count += 1; }
                }
            }
            match (count, game_board[x].current) {
                (cells, false) if game_rule.birth_rule.iter().any(|&x| x == cells) => game_board[x].next = true, // Birth Rule
                (cells, true) if game_rule.survive_rule.iter().any(|&x| x == cells) => game_board[x].next = true, // Survive Rule
                _ => game_board[x].next = false, // Anything else is death
            }
        }  
        if state_array.len() == 50 {
            if state_array.iter().all(|&x|(state_array.iter().all(|&v| v == x))) { similar_count += 10; };
            let mut array_average = 0;
            state_array.iter().for_each(|&x| { array_average += x });
            array_average /= 50;
            debug_array_average = array_average;
            if state_array.iter().all(|&x| { x > (95 * array_average) / 100 && x < (105 * array_average) / 100 }) { similar_count += 1 };
            state_array.pop_front();
        }
        let mut cell_count: i32 = 0;
        (0..game_board.len()).for_each(|x: usize|{ if game_board[x].current { cell_count += 100; } });
        state_array.push_back(cell_count);
        if similar_count >= 100 {
            game_board = seed_board(get_rand_cells(&game_rule.initial_cells)); state_array.clear();
            game_rule = get_rand_rule();
            similar_count = 0; iteration_counter += 1;
            continue;
        } else {
            (0..game_board.len()).for_each(|x| { game_board[x].current = game_board[x].next; } );
        }
        match color {
            17 => {color_iter = 1; color += color_iter;}
            231 => {color_iter = -1; color += color_iter;}
            _ => color += color_iter
        }
        print!("\x1b[38;5;{ }m",color);
        let mut space_counter = 0;
        let mut output_string: String = "".to_owned();
        (0..239).for_each(|_x| {print!("-")}); print!("\n");
        for line in game_board.as_slice().chunks(COLS) {
            output_string.push_str("| ");
            if space_counter % 2 == 0 && game_rule.grid_type == 1 { output_string.push_str(" ") }
            for cell in line {
                match cell.current {
                    true => output_string.push_str("# "),
                    false => output_string.push_str("  "),
                }
            }
            if space_counter % 2 == 1 && game_rule.grid_type == 1 { output_string.push_str(" ") }
            output_string.push_str("|\n");
            space_counter += 1;
        }
        print!("{ }",output_string);
        (0..239).for_each(|_x| {print!("-")}); print!("\n");
        let mut debug_print:String = "".to_owned();
        debug_print.push_str(&format!("Iteration Counter: { } Similarity Counter: { } Color Code: { } Array Average: { } ", iteration_counter, similar_count, color, debug_array_average));
        debug_print.push_str(&format!("Grid type: { } ",game_rule.grid_type));
        debug_print.push_str("Birth Rule: ");
        game_rule.birth_rule.iter().for_each(|x|{debug_print.push_str(&format!("{ } ",x))});
        debug_print.push_str("Survive Rule: ");
        game_rule.survive_rule.iter().for_each(|x|{debug_print.push_str(&format!("{ } ",x))});
        println!("{ }",debug_print);
        thread::sleep(time::Duration::from_millis(100));
        print!("{esc}c", esc = 27 as char);
    }
}
fn seed_board(initial_cells: i32) -> [Life; SIZE]{
    let mut game_board: [Life; SIZE] = [Life {current: false, next: false}; SIZE];
    let mut cell = 0;
    while cell != initial_cells {
        let rand_row = rand::thread_rng().gen_range(0..ROWS);
        let rand_col = rand::thread_rng().gen_range(0..COLS);
        if game_board[get_array_position(rand_row, rand_col)].current == false{
            game_board[get_array_position(rand_row, rand_col)].current = true;
            cell += 1;
        }
    }
    return game_board;
}
fn get_array_position(row: usize, col: usize) -> usize{
    (row * COLS + col) as usize
}
fn get_rand_cells(input_range: &[i32;2]) -> i32{
    return rand::thread_rng().gen_range(input_range[0]..input_range[1]);
}
fn get_rand_rule() -> LifeRule{
    let output_vec = vec![
        LifeRule {birth_rule: vec![3,8], survive_rule: vec![2,3,8], grid_type: 0, initial_cells: [200,1500]},
        LifeRule {birth_rule: vec![3], survive_rule: vec![1,2], grid_type: 0, initial_cells: [200,1500]},
        LifeRule {birth_rule: vec![3], survive_rule: vec![0,2,3], grid_type: 0, initial_cells: [200,1500]},
        LifeRule {birth_rule: vec![3], survive_rule: vec![0,1,2,3,4,5,6,7,8], grid_type: 0, initial_cells: [200,1500]},
        LifeRule {birth_rule: vec![3], survive_rule: vec![0,2,3], grid_type: 0, initial_cells: [200,1500]},
        LifeRule {birth_rule: vec![3], survive_rule: vec![1,2,3,4,5], grid_type: 0, initial_cells: [200,1500]},
        LifeRule {birth_rule: vec![3], survive_rule: vec![1,2,3,4], grid_type: 0, initial_cells: [200,1500]},
        LifeRule {birth_rule: vec![3], survive_rule: vec![2,3], grid_type: 0, initial_cells: [200,1500]},
        LifeRule {birth_rule: vec![3], survive_rule: vec![2,3,8], grid_type: 0, initial_cells: [200,1500]},
        LifeRule {birth_rule: vec![3,4,5,7], survive_rule: vec![4,5,6,8], grid_type: 0, initial_cells: [200,1500]},
        LifeRule {birth_rule: vec![3,6], survive_rule: vec![2,3], grid_type: 0, initial_cells: [200,1500]},
        LifeRule {birth_rule: vec![3,6,7,8], survive_rule: vec![2,3,5,7,8], grid_type: 0, initial_cells: [200,1500]},
        LifeRule {birth_rule: vec![3,6,7,8], survive_rule: vec![2,3,5,7,8], grid_type: 0, initial_cells: [200,1500]},
        LifeRule {birth_rule: vec![3,6,8], survive_rule: vec![2,3,4,5], grid_type: 0, initial_cells: [200,1500]},
        LifeRule {birth_rule: vec![3,8], survive_rule: vec![2,3], grid_type: 0, initial_cells: [200,1500]},
        LifeRule {birth_rule: vec![2,3], survive_rule: vec![1,2,3,5], grid_type: 1, initial_cells: [200,300]}
        ];
    let random_rule = rand::thread_rng().gen_range(0..output_vec.len());
    return output_vec[random_rule].clone();
}
