use core::tsp::{
    array_solution::ArraySolution,
    distance::DistanceFunction,
    lkh::{self, LKHConfig},
    opt3,
    solution::Solution,
};
use std::{
    collections::VecDeque,
    io::{self, BufRead},
    path::PathBuf,
    str::FromStr,
};

fn read_input() -> Result<Vec<Vec<char>>, anyhow::Error> {
    let stdin = io::stdin();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        grid.push(line.chars().collect());
    }
    Ok(grid)
}

fn create_wall(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = vec![vec!['#'; grid[0].len() + 2]; grid.len() + 2];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            new_grid[i + 1][j + 1] = grid[i][j];
        }
    }
    new_grid
}

struct Problem {
    grid: Vec<Vec<char>>,
    id_table: Vec<Vec<usize>>,
    width: usize,
    height: usize,
    coords: Vec<(usize, usize)>,
    distance_table: Vec<Vec<i64>>,
    start: usize,
}

const DY: [i64; 4] = [0, 1, 0, -1];
const DX: [i64; 4] = [1, 0, -1, 0];
const DIRS: [char; 4] = ['R', 'D', 'L', 'U'];

impl Problem {
    fn bfs(&mut self, start: usize) {
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        self.distance_table[start][start] = 0;

        while let Some((id, distance)) = queue.pop_front() {
            for i in 0..4 {
                let (y, x) = self.coords[id];
                let ny = y as i64 + DY[i];
                let nx = x as i64 + DX[i];
                if nx < 0
                    || ny < 0
                    || ny >= self.height as i64
                    || nx >= self.width as i64
                    || self.grid[ny as usize][nx as usize] == '#'
                {
                    continue;
                }
                let next_id = self.id_table[ny as usize][nx as usize];
                if next_id == std::usize::MAX {
                    continue;
                }
                if self.distance_table[start][next_id] != std::i64::MAX {
                    continue;
                }
                self.distance_table[start][next_id] = distance + 1;

                queue.push_back((next_id, distance + 1));
            }
        }
    }

    fn new(grid: Vec<Vec<char>>) -> Self {
        let width = grid[0].len();
        let height: usize = grid.len();
        let mut id_table = vec![vec![std::usize::MAX; width]; height];
        let mut coords = vec![];
        let mut id = 0;
        let mut start = std::usize::MAX;

        for i in 0..height {
            for j in 0..width {
                if grid[i][j] != '#' {
                    id_table[i][j] = id;
                    coords.push((i, j));
                    if grid[i][j] == 'L' {
                        start = id;
                    }
                    id += 1;
                }
            }
        }
        // ハミルトン路を計算するために、距離0の頂点を挿入する
        let distance_table = vec![vec![std::i64::MAX; id]; id];

        let mut problem = Problem {
            grid,
            id_table,
            width,
            height,
            coords,
            distance_table,
            start,
        };

        for i in 0..id {
            problem.bfs(i);
        }

        problem
    }
}

impl DistanceFunction for Problem {
    fn distance(&self, id1: u32, id2: u32) -> i64 {
        self.distance_table[id1 as usize][id2 as usize]
    }

    fn dimension(&self) -> u32 {
        self.coords.len() as u32
    }

    fn name(&self) -> String {
        "lambdaman".to_string()
    }
}

fn bfs(problem: &Problem, start: usize, goal: usize) -> String {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let mut recur_table = vec![std::usize::MAX; problem.dimension() as usize];

    while let Some((id, distance)) = queue.pop_front() {
        if id == goal {
            let mut command_buffer = vec![];
            let c = problem.coords[goal];
            let mut c = (c.0 as i64, c.1 as i64);
            let target = problem.coords[start];
            let target = (target.0 as i64, target.1 as i64);

            while target != c {
                let id = problem.id_table[c.0 as usize][c.1 as usize];
                let dir = recur_table[id];
                command_buffer.push(DIRS[(dir + 2) % 4]);
                c.0 += DY[dir];
                c.1 += DX[dir];
            }
            command_buffer.reverse();
            return String::from_iter(command_buffer);
        }

        for dir in 0..4 {
            let (y, x) = problem.coords[id];
            let ny = y as i64 + DY[dir];
            let nx = x as i64 + DX[dir];
            if nx < 0
                || ny < 0
                || ny >= problem.height as i64
                || nx >= problem.width as i64
                || problem.grid[ny as usize][nx as usize] == '#'
            {
                continue;
            }
            let next_id = problem.id_table[ny as usize][nx as usize];
            if next_id == std::usize::MAX {
                continue;
            }
            if recur_table[next_id] != std::usize::MAX {
                continue;
            }
            recur_table[next_id] = (dir + 2) % 4;
            queue.push_back((next_id, distance + 1));
        }
    }
    unreachable!("cannot find target id");
}

fn reconstruct_path(problem: &Problem, solution: &ArraySolution) -> String {
    // L から始めて、最短経路を通っては復元するのを繰り返す
    let mut buffer = String::new();
    let mut start = problem.start;

    for _iter in 0..problem.dimension() - 1 {
        let next = solution.next(start as u32) as usize;
        let path = bfs(problem, start, next);
        buffer.push_str(path.as_str());
        start = next;
    }
    buffer
}

fn main() -> Result<(), anyhow::Error> {
    let table = read_input()?;
    let table = create_wall(table);

    let problem = Problem::new(table);
    if false {
        for y in 0..problem.dimension() {
            for x in 0..problem.dimension() {
                eprint!("{:5}, ", problem.distance(y, x));
            }
            eprintln!();
        }
    }

    let solution = ArraySolution::new(problem.dimension() as usize);
    let path = "lambdaman.txt";

    eprintln!("dimension: {}", problem.dimension());

    let init_solution = opt3::solve(
        &problem,
        solution,
        opt3::Opt3Config {
            use_neighbor_cache: false,
            debug: false,
            cache_filepath: PathBuf::from_str(path).unwrap(),
        },
    );

    let final_solution = lkh::solve(
        &problem,
        init_solution,
        LKHConfig {
            use_neighbor_cache: false,
            cache_filepath: PathBuf::from_str(path).unwrap(),
            debug: false,
            time_ms: 1_000,
            start_kick_step: 5,
            kick_step_diff: 10,
            end_kick_step: problem.dimension() as usize / 10,
            fail_count_threashold: 50,
            max_depth: 2,
        },
    );

    // パスの復元
    let path_all = reconstruct_path(&problem, &final_solution);
    print!("{}", path_all);

    Ok(())
}
