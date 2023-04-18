use bitvec::prelude::*;
use rustc_hash::FxHashMap as HashMap;
use std::cmp::{self, Reverse};

pub const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq)]
pub struct Valve {
    id: String,
    flow_rate: u32,
    tunnel_ids: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ProcessedValve {
    flow_rate: u32,
    distances: Vec<u32>,
}

pub fn preprocess(mut valves: Vec<Valve>) -> (Vec<ProcessedValve>, Vec<u32>) {
    // Sort the valves by flow rate in descending order.
    // Valves with lower indices have higher flow rates.
    // The Solutions iterator's upper bound calculation
    // relies on this
    valves.sort_by_key(|valve| Reverse(valve.flow_rate));

    let distances = find_all_pairs_shortest_paths(&valves);
    let valves_with_flow = valves
        .iter()
        .map(|valve| valve.flow_rate)
        .take_while(|&flow_rate| flow_rate > 0)
        .enumerate()
        .collect::<Vec<_>>();
    let valves_with_flow_count = valves_with_flow.len();

    // Distances from the starting point to each valve
    let initial_distances = valves
        .iter()
        .position(|valve| valve.id == "AA")
        .map(|i| distances[i][..valves_with_flow_count].to_vec())
        .unwrap();

    let processed_valves = valves_with_flow
        .into_iter()
        .map(|(i, flow_rate)| {
            let distances = distances[i][..valves_with_flow_count].to_vec();
            ProcessedValve {
                flow_rate,
                distances,
            }
        })
        .collect();
    (processed_valves, initial_distances)
}

pub fn part_one(valves: &[ProcessedValve], initial_distances: &[u32]) -> u32 {
    let unreleased_valve_ids = bitvec![1; valves.len()];
    Solutions::new(valves, initial_distances, unreleased_valve_ids, 30)
        .map(|(pressure_released, _)| pressure_released)
        .max()
        .unwrap()
}

pub fn part_two(valves: &[ProcessedValve], initial_distances: &[u32]) -> u32 {
    let unreleased_valve_ids = bitvec![1; valves.len()];
    Solutions::new(valves, initial_distances, unreleased_valve_ids, 26)
        .map(|(own_pressue, remaining_unreleased_valve_ids)| {
            let elephant_pressure = Solutions::new(
                valves,
                initial_distances,
                remaining_unreleased_valve_ids,
                26,
            )
            .map(|(elephant_pressure, _)| elephant_pressure)
            .max()
            .unwrap();
            own_pressue + elephant_pressure
        })
        .max()
        .unwrap()
}

// Floyd-Warshall algorithm
// https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
fn find_all_pairs_shortest_paths(valves: &[Valve]) -> Vec<Vec<u32>> {
    let id_to_index = valves
        .iter()
        .enumerate()
        .map(|(i, valve)| (&valve.id, i))
        .collect::<HashMap<_, _>>();
    let mut distances = vec![vec![u32::MAX; valves.len()]; valves.len()];
    for (i, valve) in valves.iter().enumerate() {
        distances[i][i] = 0;
        for tunnel_id in &valve.tunnel_ids {
            let j = id_to_index[tunnel_id];
            distances[i][j] = 1;
        }
    }
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                distances[i][j] = cmp::min(
                    distances[i][j],
                    distances[i][k].saturating_add(distances[k][j]),
                );
            }
        }
    }
    distances
}

/// Iterator over possible solutions to opening valves
/// within the time limit.
/// Iterates over tuples of:
///   - the pressure released by opening valves in this
///     order
///   - the set of IDs of valves that were not released
///
/// This iterator does not include every possible solution
/// because it uses an upper bound to prune partial
/// solutions that are guaranteed to be worse than the best
/// solution found so far in terms of pressure released.
struct Solutions<'a> {
    valves: &'a [ProcessedValve],
    stack: Vec<State<'a>>,
    current_best: u32,
}

struct State<'a> {
    distances: &'a [u32],
    remaining_minutes: u32,
    unreleased_valve_ids: BitVec,
    released_pressure: u32,
}

impl<'a> Solutions<'a> {
    fn new(
        valves: &'a [ProcessedValve],
        initial_distances: &'a [u32],
        unreleased_valve_ids: BitVec,
        time_limit: u32,
    ) -> Self {
        let initial_state = State {
            distances: initial_distances,
            remaining_minutes: time_limit,
            unreleased_valve_ids,
            released_pressure: 0,
        };
        Self {
            valves,
            stack: vec![initial_state],
            current_best: 0,
        }
    }
}

impl<'a> Iterator for Solutions<'a> {
    type Item = (u32, BitVec);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(state) = self.stack.pop() {
            let upper_bound = calculate_remaining_pressure_upper_bound(self.valves, &state);
            if upper_bound <= self.current_best {
                continue;
            }
            let mut is_solution = true;
            for id in state.unreleased_valve_ids.iter_ones() {
                let minutes_to_release = state.distances[id] + 1;
                if state.remaining_minutes > minutes_to_release {
                    let remaining_minutes = state.remaining_minutes - minutes_to_release;
                    let ProcessedValve {
                        flow_rate,
                        ref distances,
                    } = self.valves[id];
                    let released_pressure = state.released_pressure + remaining_minutes * flow_rate;
                    let mut unreleased_valve_ids = state.unreleased_valve_ids.clone();
                    unreleased_valve_ids.set(id, false);
                    let next_state = State {
                        distances,
                        remaining_minutes,
                        unreleased_valve_ids,
                        released_pressure,
                    };
                    self.stack.push(next_state);
                    is_solution = false;
                }
            }
            if is_solution {
                self.current_best = cmp::max(self.current_best, state.released_pressure);
                return Some((state.released_pressure, state.unreleased_valve_ids));
            }
        }
        None
    }
}

fn calculate_remaining_pressure_upper_bound(valves: &[ProcessedValve], state: &State) -> u32 {
    let release_times = (0..=state.remaining_minutes).rev().step_by(2).skip(1);
    // flow_rates are sorted in descending order because
    // valves were sorted in the preprocess function
    let flow_rates = state
        .unreleased_valve_ids
        .iter_ones()
        .map(|id| valves[id].flow_rate);
    let max_remaining_pressure_release = release_times
        .zip(flow_rates)
        .map(|(t, f)| t * f)
        .sum::<u32>();
    state.released_pressure + max_remaining_pressure_release
}

pub mod parser {
    use super::Valve;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        character::complete::{line_ending, u32},
        combinator::map,
        multi::{many1, separated_list1},
        sequence::{preceded, terminated, Tuple},
        Finish, IResult,
    };

    pub fn parse(input: &str) -> Result<Vec<Valve>> {
        let (rest, valves) = valves(input)
            .finish()
            .map_err(|err| anyhow!(err.to_string()))?;
        if !rest.is_empty() {
            return Err(anyhow!("Unparsed input: {}", rest));
        }
        Ok(valves)
    }

    fn valve(input: &str) -> IResult<&str, Valve> {
        let valve_id = |input| map(take(2usize), String::from)(input);
        let id = preceded(tag("Valve "), valve_id);
        let flow_rate = preceded(tag(" has flow rate="), u32);
        let single_tunnel_id = preceded(
            tag("; tunnel leads to valve "),
            map(valve_id, |id| vec![id]),
        );
        let multiple_tunnel_ids = preceded(
            tag("; tunnels lead to valves "),
            separated_list1(tag(", "), valve_id),
        );
        let tunnel_ids = alt((single_tunnel_id, multiple_tunnel_ids));
        let (input, (id, flow_rate, tunnel_ids)) = (id, flow_rate, tunnel_ids).parse(input)?;
        let valve = Valve {
            id,
            flow_rate,
            tunnel_ids,
        };
        Ok((input, valve))
    }

    fn valves(input: &str) -> IResult<&str, Vec<Valve>> {
        let line = terminated(valve, line_ending);
        many1(line)(input)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_valve() {
            let input = "Valve EF has flow rate=22; tunnels lead to valves FK, HT, DE";
            let expected = Valve {
                id: "EF".to_string(),
                flow_rate: 22,
                tunnel_ids: vec!["FK".to_string(), "HT".to_string(), "DE".to_string()],
            };
            assert_eq!(valve(input), Ok(("", expected)));
            let input = "Valve AA has flow rate=22; tunnel leads to valve FK";
            let expected = Valve {
                id: "AA".to_string(),
                flow_rate: 22,
                tunnel_ids: vec!["FK".to_string()],
            };
            assert_eq!(valve(input), Ok(("", expected)));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let valves = parser::parse(INPUT).unwrap();
        let (processed_valves, initial_distances) = preprocess(valves);
        assert_eq!(part_one(&processed_valves, &initial_distances), 2320);
    }

    #[test]
    fn test_part_two() {
        let valves = parser::parse(INPUT).unwrap();
        let (processed_valves, initial_distances) = preprocess(valves);
        assert_eq!(part_two(&processed_valves, &initial_distances), 2967);
    }
}
