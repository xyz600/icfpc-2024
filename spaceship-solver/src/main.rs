use core::tsp::{
    array_solution::ArraySolution,
    distance::DistanceFunction,
    lkh::{self, LKHConfig},
    opt3,
    solution::Solution,
};
use std::{
    io::{self, BufRead},
    path::PathBuf,
    str::FromStr,
};

fn read_input() -> Result<Vec<(i64, i64)>, anyhow::Error> {
    let stdin = io::stdin();
    let mut grid: Vec<(i64, i64)> = Vec::new();
    grid.push((0, 0));

    for line in stdin.lock().lines() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let nums = line
            .split_whitespace()
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;

        grid.push((nums[0], nums[1]));
    }
    Ok(grid)
}

struct Problem {
    point_list: Vec<(i64, i64)>,
    name: String,
}

impl Problem {
    pub fn new(point_list: Vec<(i64, i64)>, name: String) -> Problem {
        Problem { point_list, name }
    }
}

impl DistanceFunction for Problem {
    fn distance(&self, id1: u32, id2: u32) -> i64 {
        let dy = self.point_list[id1 as usize].0 - self.point_list[id2 as usize].0;
        let dx = self.point_list[id1 as usize].1 - self.point_list[id2 as usize].1;
        ((dy * dy + dx * dx) as f64).sqrt().round() as i64
    }

    fn dimension(&self) -> u32 {
        self.point_list.len() as u32
    }

    fn name(&self) -> String {
        self.name.to_string()
    }
}

fn tsp(problem: &Problem) -> Vec<usize> {
    let path = "spaceship_cache";

    let init_solution = ArraySolution::new(problem.dimension() as usize);
    let init_solution = opt3::solve(
        problem,
        init_solution,
        opt3::Opt3Config {
            use_neighbor_cache: false,
            debug: false,
            cache_filepath: PathBuf::from_str(path).unwrap(),
        },
    );

    let final_solution = lkh::solve(
        problem,
        init_solution,
        LKHConfig {
            use_neighbor_cache: false,
            cache_filepath: PathBuf::from_str(path).unwrap(),
            debug: false,
            time_ms: 10_000,
            start_kick_step: 5,
            kick_step_diff: 10,
            end_kick_step: problem.dimension() as usize / 10,
            fail_count_threashold: 50,
            max_depth: 6,
        },
    );

    let mut ret = vec![];
    let mut start = 0;
    for _ in 0..problem.dimension() {
        ret.push(start as usize);
        start = final_solution.next(start as u32) as usize;
    }
    ret
}

fn main() -> Result<(), anyhow::Error> {
    // ユークリッド距離で TSP を解く
    // この順序で訪れることを強く前提に置いて、ビームサーチで手順を求める
    let coords = read_input()?;
    let problem = Problem::new(coords, "spaceship".to_string());

    let final_solution = tsp(&problem);
    eprintln!("{:?}", final_solution);

    Ok(())
}
