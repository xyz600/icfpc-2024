use core::tsp::{
    array_solution::ArraySolution,
    distance::DistanceFunction,
    lkh::{self, LKHConfig},
    opt3,
    solution::Solution,
};
use std::{
    collections::HashSet,
    io::{self, BufRead},
    path::PathBuf,
    str::FromStr,
};

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}

fn read_input() -> Result<Vec<Point>, anyhow::Error> {
    let stdin = io::stdin();
    let mut grid: Vec<Point> = Vec::new();
    grid.push(Point::new(0, 0));

    for line in stdin.lock().lines() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let nums = line
            .split_whitespace()
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;

        grid.push(Point::new(nums[0], nums[1]));
    }
    Ok(grid)
}

struct Problem {
    point_list: Vec<Point>,
    name: String,
}

impl Problem {
    pub fn new(point_list: Vec<Point>, name: String) -> Problem {
        Problem { point_list, name }
    }
}

impl DistanceFunction for Problem {
    fn distance(&self, id1: u32, id2: u32) -> i64 {
        let dy = self.point_list[id1 as usize].y - self.point_list[id2 as usize].y;
        let dx = self.point_list[id1 as usize].x - self.point_list[id2 as usize].x;
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

#[derive(Debug, Clone)]
struct State {
    node_index: usize,
    vy: i64,
    vx: i64,
    y: i64,
    x: i64,
    action_buffer: Vec<u8>,
}

impl State {
    fn apply_action(&mut self, action: usize, problem: &Problem, coord_order: &Vec<usize>) {
        let (dy, dx) = ACTION_LIST[action];
        self.vy += dy;
        self.vx += dx;
        self.y += self.vy;
        self.x += self.vx;
        self.action_buffer.push((action + 1) as u8);

        if self.node_index == problem.point_list.len() {
            return;
        }
        let mut target_index = coord_order[self.node_index];
        while self.node_index < problem.point_list.len()
            && problem.point_list[target_index].x == self.x
            && problem.point_list[target_index].y == self.y
        {
            self.node_index += 1;
            if self.node_index < problem.point_list.len() {
                target_index = coord_order[self.node_index];
            }
        }
    }
}

fn evaluate(problem: &Problem, state: &State) -> (usize, i64) {
    if state.node_index == problem.point_list.len() {
        (0, 0)
    } else {
        let dy = problem.point_list[state.node_index].y - state.y;
        let dx = problem.point_list[state.node_index].x - state.x;
        let dist2 = dy * dy + dx * dx;

        (problem.point_list.len() + 1 - state.node_index, dist2)
    }
}

const ACTION_LIST: [(i64, i64); 9] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Clone, Copy)]
struct StateDiff {
    state_index: usize,
    // 0 - 8
    action: usize,

    score: (usize, i64),
}

fn main() -> Result<(), anyhow::Error> {
    // ユークリッド距離で TSP を解く
    // この順序で訪れることを強く前提に置いて、ビームサーチで手順を求める
    let coords = read_input()?;
    let problem = Problem::new(coords, "spaceship".to_string());

    let coord_order = tsp(&problem);

    // beam search
    let mut state_buffer = [
        vec![State {
            node_index: 1,
            vy: 0,
            vx: 0,
            y: 0,
            x: 0,
            action_buffer: vec![],
        }],
        vec![],
    ];

    let beam_width = 1000;
    let mut state_diff: Vec<StateDiff> = vec![];
    let mut state_table = HashSet::<(usize, i64, i64, i64, i64)>::new();
    for iter in 0.. {
        eprintln!(
            "iter: {}, node_index: {}",
            iter, state_buffer[0][0].node_index
        );

        state_diff.clear();
        state_table.clear();

        for (si, s) in state_buffer[0].iter().enumerate() {
            for action in 0..9 {
                let mut state = s.clone();
                state.apply_action(action, &problem, &coord_order);
                let (score, dist2) = evaluate(&problem, &state);
                let diff = StateDiff {
                    state_index: si,
                    action,
                    score: (score, dist2),
                };
                if state_table.insert((state.node_index, state.y, state.x, state.vy, state.vx)) {
                    state_diff.push(diff);
                }
            }
        }

        state_diff.sort_by_key(|v| v.score);
        state_diff.truncate(beam_width);

        for diff in state_diff.iter() {
            let state = state_buffer[0][diff.state_index].clone();
            let mut state = state.clone();
            state.apply_action(diff.action, &problem, &coord_order);
            state_buffer[1].push(state);
        }

        state_buffer.swap(0, 1);
        state_buffer[1].clear();

        if state_buffer[0][0].node_index == problem.point_list.len() {
            break;
        }
    }

    for action in state_buffer[0][0].action_buffer.iter() {
        print!("{}", action);
    }
    println!("");

    Ok(())
}
