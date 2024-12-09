use std::collections::HashSet;

#[path = "../input.rs"]
mod input;

type Position = (i32, i32);
type Direction = (i32, i32);

#[derive(Clone)]
struct Map {
    data: Vec<Vec<char>>,
}

fn main() {
    let map = read_map(&input::read_stdin());
    println!("explored positions: {}", sum_explored_positions(&map));
    println!(
        "number of obstacles that would cause a loop: {}",
        sum_loop_obstacles(&map)
    );
}

fn sum_explored_positions(map: &Map) -> u32 {
    let mut visited: HashSet<Position> = HashSet::new();
    let (mut pos, mut dir) = get_start_pos(map);
    visited.insert(pos);

    let mut sum = 1;
    loop {
        let coord_in_front = coord_in_front(pos, dir);
        let char_in_front = char_in_front(coord_in_front, map);
        match char_in_front {
            Some('#') => dir = turn(dir),
            Some(_) => {
                pos = coord_in_front;
            }
            None => break,
        }
        if !visited.contains(&pos) {
            visited.insert(pos);
            sum += 1;
        }
    }
    sum
}

fn sum_loop_obstacles(map: &Map) -> u32 {
    let mut sum = 0;
    let mut new_map = map.clone();
    for (i, line) in map.data.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            new_map.data[i][j] = '#';
            if will_escape(&new_map) {
                sum += 1
            }
            new_map.data[i][j] = *char;
        }
    }
    sum
}

fn will_escape(
    map: &Map,
) -> bool {
    let mut visited: HashSet<(Position, Direction)> = HashSet::new();
    let (mut pos, mut dir) = get_start_pos(map);
    visited.insert((pos, dir));

    loop {
        let coord_in_front = coord_in_front(pos, dir);
        let char_in_front = char_in_front(coord_in_front, map);
        match char_in_front {
            Some('#') => dir = turn(dir),
            Some(_) => {
                pos = coord_in_front;
            }
            None => break,
        }
        let coordinate = (pos, dir);
        if visited.contains(&coordinate) {
            return true;
        } else {
            visited.insert(coordinate);
        }
    }
    false
}

fn turn(dir: Direction) -> Direction {
    match dir {
        (0, 1) => (1, 0),
        (0, -1) => (-1, 0),
        (1, 0) => (0, -1),
        (-1, 0) => (0, 1),
        _ => (1, 0),
    }
}

fn get_start_pos(map: &Map) -> (Position, Direction) {
    let mut pos = (0, 0);
    let mut dir = (1, 0);
    for (i, line) in map.data.iter().enumerate() {
        for (j, grid) in line.into_iter().enumerate() {
            let char = *grid;
            if char == 'v' || char == '^' || char == '>' || char == '<' {
                pos = (i as i32, j as i32);
                dir = match char {
                    'v' => (1, 0),
                    '^' => (-1, 0),
                    '>' => (0, 1),
                    '<' => (0, -1),
                    _ => dir,
                };
                break;
            }
        }
    }
    (pos, dir)
}

fn coord_in_front(pos: Position, dir: Direction) -> Position {
    (pos.0 + dir.0, pos.1 + dir.1)
}

fn char_in_front(coord_in_front: (i32, i32), map: &Map) -> Option<char> {
    let char_in_front: Option<char> = {
        if coord_in_front.0 < 0 || coord_in_front.1 < 0 {
            None
        } else {
            map.data
                .get(coord_in_front.0 as usize)
                .and_then(|x| x.get(coord_in_front.1 as usize).and_then(|v| Some(*v)))
        }
    };
    char_in_front
}

fn read_map(input: &str) -> Map {
    let data: Vec<Vec<char>> = input
        .lines()
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    Map { data }
}
