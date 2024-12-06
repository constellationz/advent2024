#[path = "../input.rs"]
mod input;

type Crossword = Vec<Vec<char>>;
type Window = Vec<Vec<char>>;

fn main() {
    let crossword = read_crossword(&input::read_stdin());
    println!(
        "number of occurences: {}",
        num_occurences(&crossword, "XMAS")
    );
    println!("number of x occurences: {}", num_x_occurences(&crossword));
}

fn num_occurences(crossword: &Crossword, word: &str) -> u32 {
    let cell_iter = (1..grid.r - 1)
        .map(|i| (1..grid.c - 1).map(move |j| (i, j)))
        .flatten();
    let cell_iter2 = crossword
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().enumerate().map(|(j, _col)| (i, j)))
        .flatten();
    crossword.iter().enumerate().fold(0, |sum, (i, row)| {
        sum + row.iter().enumerate().fold(0, |row_sum, (j, _col)| {
            row_sum + occurences_at(crossword, i as i32, j as i32, word)
        })
    })
}

fn num_x_occurences(crossword: &Crossword) -> u32 {
    x_windows().iter().fold(0, |sum, window| {
        sum + num_window_occurences(crossword, window)
    })
}

fn x_windows() -> Vec<Vec<Vec<char>>> {
    vec![
        vec![
            vec!['M', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'S'],
        ],
        vec![
            vec!['M', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'S'],
        ],
        vec![
            vec!['S', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'M'],
        ],
        vec![
            vec!['S', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'M'],
        ],
    ]
}

fn num_window_occurences(crossword: &Crossword, window: &Vec<Vec<char>>) -> u32 {
    let (crossword_width, crossword_height) = dims(crossword);
    let (window_width, window_height) = dims(window);
    (0..=crossword_width - window_width).fold(0, |sum, i| {
        sum + (0..=crossword_height - window_height).fold(0, |inner_sum, j| {
            inner_sum
                + if window_matches(crossword, window, i, j) {
                    1
                } else {
                    0
                }
        })
    })
}

fn dims<T>(arr2d: &[Vec<T>]) -> (usize, usize) {
    (arr2d.len(), arr2d[0].len())
}

fn window_matches(crossword: &Crossword, window: &Window, i: usize, j: usize) -> bool {
    window.iter().enumerate().all(|(window_i, window_row)| {
        window_row
            .iter()
            .enumerate()
            .all(|(window_j, window_char)| {
                *window_char == '.' || *window_char == crossword[i + window_i][j + window_j]
            })
    })
}

// Uses flood-fill style algorithm to find all words
// Probably should've used a flood-fill style algorithm, but oh well!
fn occurences_at(crossword: &Crossword, i: i32, j: i32, word: &str) -> u32 {
    if crossword_at(crossword, i, j) == word.chars().next() {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .fold(0, |sum, direction| {
            sum + occurences_descent(crossword, i, j, direction.0, direction.1, word, 1)
        })
    } else {
        0
    }
}

// A recursive descent for following a lead on a word
fn occurences_descent(
    crossword: &Crossword,
    i: i32,
    j: i32,
    i_dir: i32,
    j_dir: i32,
    word: &str,
    word_pos: usize,
) -> u32 {
    if word_pos == word.len() {
        return 1;
    }
    let new_i: i32 = i + i_dir;
    let new_j: i32 = j + j_dir;
    if crossword_at(crossword, new_i, new_j) == word.chars().nth(word_pos) {
        occurences_descent(crossword, new_i, new_j, i_dir, j_dir, word, word_pos + 1)
    } else {
        0
    }
}

fn crossword_at(crossword: &Crossword, i: i32, j: i32) -> Option<char> {
    let i_index = i as usize;
    let j_index = j as usize;
    if i >= 0 && j >= 0 && i_index < crossword.len() && j_index < crossword[0].len() {
        Some(crossword[i_index][j_index])
    } else {
        None
    }
}

fn read_crossword(input: &str) -> Crossword {
    input.lines().map(|line| line.chars().collect()).collect()
}
