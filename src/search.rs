use crate::board::*;
use crate::heuristics::*;
use crate::min_heap::*;
use std::collections::*;
use std::time::Duration;

/// Statistics of the search, used to evaluate the performance of the search algorithms.
/// Feel free to add more fields to this struct if you need them.
pub struct Stats {
    /// Numbers of states expanded during search
    pub expanded: usize,
    /// Total runtime spend in the search.
    ///
    /// ```rust
    /// let start_time: Instant = std::time::Instant::now();
    /// // do something
    /// let runtime: Duration = start_time.elapsed();
    /// ```
    pub runtime: Duration,
}

impl Stats {
    /// Creates a new `Stats` instance with the given expanded states count and runtime.
    pub fn new(expanded: usize, runtime: Duration) -> Stats {
        Stats { expanded, runtime }
    }
}

fn backtrack_moves_rec(current_state: Board, predecessors: &HashMap<Board, (Board, Direction)>, mut moves: Vec<Direction>) -> Vec<Direction> {
    match predecessors.get(&current_state) {
        None => {return moves;}
        Some ((parent_state, m)) => {
            moves.push(*m);
            println!("nouvelle valeur de moves:");
            for direction in moves.clone() {
                println!("{}", direction);
            }
            return backtrack_moves_rec(*parent_state, predecessors, moves);
        }
    };
}

fn backtrack_moves(state: Board, predecessors: &HashMap<Board, (Board, Direction)>) -> Vec<Direction> {
    println!("backtracking...");
    let mut path = backtrack_moves_rec(state, predecessors, Vec::new());
    println!("path len: {}", path.len());
    for direction in path.clone() {
        println!("{}", direction);
    }
    path.reverse();
    return path;
}

fn check_end_search(state: Board) -> bool {
    return state.is_valid_plan(&[]);
}

pub fn search(init_state: Board) -> (Option<Vec<Direction>>, Stats) {
    // record the start time when starting the search (so we can later the time that elapsed since)
    let start = std::time::Instant::now();
    if (check_end_search(init_state)) {
        // here is an example to measure the runtime and returns the statistics
        let runtime = start.elapsed();
        // example to construct a Stats instance
        let stats = Stats::new(0, runtime);
        // return the results and associated stats
        return (Some(Vec::new()), stats);
    }

    // frontier: MinHeap provide allows to store the states to explore, with associated priority
    let mut heap: MinHeap<Board> = MinHeap::new();

    // the standard library provides a HashMap, that can be used to store the cost and predecessors of each state
    // assocaciates each state on the frontier to the best cost to reach it
    let mut path_costs: HashMap<Board, u32> = HashMap::new();
    // assocaciates each state on the frontier to the its best parent state and the action to it (parent, action)
    let mut predecessors: HashMap<Board, (Board, Direction)> = HashMap::new();

    // keeps track all states that have been expanded
    let mut expanded: HashSet<Board> = HashSet::new();

    // search algorithm implementation
    let mut result = None;

    heap.insert(init_state, 1);
    path_costs.insert(init_state, 1);

    while !heap.is_empty() {
        let state = heap.pop().unwrap();
        if expanded.contains(&state) { continue };
        if (check_end_search(state)) {
            result = Some(Vec::new());
            break;
        }
        let cost = *path_costs.get(&state).unwrap();

        for direction in [Direction::Up, Direction::Right, Direction::Down, Direction::Left] {
            let apply_result = state.apply(direction);
            match apply_result {
                None => /*invalid direction for state*/{},
                Some (new_state) => {
                    path_costs.insert(new_state, cost+1);
                    predecessors.insert(new_state, (state, direction));
                    heap.insert(new_state, 1);
                    if (check_end_search(new_state)) {
                        result = Some(backtrack_moves(new_state, &predecessors));
                        break;
                    }
                }
            }
        }
        expanded.insert(state);
    }

    // here is an example to measure the runtime and returns the statistics
    let runtime = start.elapsed();
    // example to construct a Stats instance
    let stats = Stats::new(expanded.len(), runtime);
    // return the results and associated stats
    (result, stats)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_search() {
        use super::*;

        // validates that search does return the optimal plan on the first 20 isntances

        for (expected_cost, init) in &INSTANCES[0..20] {
            let (path, stats) = search(*init);
            let path = path.expect("no plan");
            assert!(init.is_valid_plan(&path));
            assert_eq!(path.len(), *expected_cost as usize);
        }
    }
}
