use bitvec::prelude::*;
use rustc_hash::FxHashMap as HashMap;
use std::cmp::{self, Reverse};

pub const INPUT: &str = include_str!("../input.txt");

type Valves = Vec<(String, u32, Vec<String>)>;

type ProcessedValves = Vec<(u32, Vec<u32>)>;

pub fn preprocess(mut valves: Valves) -> (ProcessedValves, Vec<u32>) {
    valves.sort_by_key(|&(_, flow_rate, _)| Reverse(flow_rate));
    let distances = find_all_pairs_shortest_paths(&valves);
    let valves_with_flow = valves
        .iter()
        .map(|&(_, flow_rate, _)| flow_rate)
        .take_while(|&flow_rate| flow_rate > 0)
        .enumerate()
        .collect::<Vec<_>>();
    let valves_with_flow_count = valves_with_flow.len();
    let initial_distances = valves
        .iter()
        .position(|(id, _, _)| id == "AA")
        .map(|i| distances[i][..valves_with_flow_count].to_vec())
        .unwrap();
    let processed_valves = valves_with_flow
        .into_iter()
        .map(|(i, flow_rate)| {
            let distances = distances[i][..valves_with_flow_count].to_vec();
            (flow_rate, distances)
        })
        .collect();
    (processed_valves, initial_distances)
}

pub fn part_one(valves: &ProcessedValves, initial_distances: &[u32]) -> u32 {
    let unreleased_valve_ids = bitvec![1; valves.len()];
    Solutions::new(valves, initial_distances, unreleased_valve_ids, 30)
        .map(|(pressure_released, _)| pressure_released)
        .max()
        .unwrap()
}

pub fn part_two(valves: &ProcessedValves, initial_distances: &[u32]) -> u32 {
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
fn find_all_pairs_shortest_paths(valves: &[(String, u32, Vec<String>)]) -> Vec<Vec<u32>> {
    let id_to_index = valves
        .iter()
        .enumerate()
        .map(|(i, (id, _, _))| (id.as_str(), i))
        .collect::<HashMap<_, _>>();
    let mut distances = vec![vec![u32::MAX; valves.len()]; valves.len()];
    for (i, (_, _, tunnel_ids)) in valves.iter().enumerate() {
        distances[i][i] = 0;
        for tunnel_id in tunnel_ids {
            let j = id_to_index[tunnel_id.as_str()];
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

struct State<'a> {
    distances: &'a [u32],
    remaining_minutes: u32,
    unreleased_valve_ids: BitVec,
    released_pressure: u32,
}

struct Solutions<'a> {
    valves: &'a ProcessedValves,
    stack: Vec<State<'a>>,
    current_best: u32,
}

impl<'a> Solutions<'a> {
    fn new(
        valves: &'a ProcessedValves,
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
            let release_times = (0..=state.remaining_minutes).rev().step_by(2).skip(1);
            let flow_rates = state
                .unreleased_valve_ids
                .iter_ones()
                .map(|id| self.valves[id].0);
            let max_remaining_pressure_release = release_times
                .zip(flow_rates)
                .map(|(t, f)| t * f)
                .sum::<u32>();
            let upper_bound = state.released_pressure + max_remaining_pressure_release;
            if upper_bound <= self.current_best {
                continue;
            }
            let mut is_solution = true;
            for id in state.unreleased_valve_ids.iter_ones() {
                let minutes_to_release = state.distances[id] + 1;
                if state.remaining_minutes > minutes_to_release {
                    let remaining_minutes = state.remaining_minutes - minutes_to_release;
                    let (flow_rate, ref distances) = self.valves[id];
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

pub mod parser {
    use super::Valves;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        character::complete::{line_ending, u32},
        combinator::{iterator, map},
        multi::separated_list1,
        sequence::{preceded, terminated, Tuple},
        Finish, IResult,
    };

    pub fn parse(input: &str) -> Result<Vec<(String, u32, Vec<String>)>> {
        let (rest, valves) = valves(input)
            .finish()
            .map_err(|err| anyhow!(err.to_string()))?;
        if !rest.is_empty() {
            return Err(anyhow!("Unparsed input: {}", rest));
        }
        Ok(valves)
    }

    fn valve(input: &str) -> IResult<&str, (String, u32, Vec<String>)> {
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
        let (input, valve) = (id, flow_rate, tunnel_ids).parse(input)?;
        Ok((input, valve))
    }

    fn valves(input: &str) -> IResult<&str, Valves> {
        let line = terminated(valve, line_ending);
        let mut iter = iterator(input, line);
        let valves = iter.collect();
        let (input, _) = iter.finish()?;
        Ok((input, valves))
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_valve() {
            let input = "Valve EF has flow rate=22; tunnels lead to valves FK, HT, DE";
            let expected = (
                "EF".to_string(),
                22,
                vec!["FK".to_string(), "HT".to_string(), "DE".to_string()],
            );
            assert_eq!(valve(input), Ok(("", expected)));
            let input = "Valve AA has flow rate=22; tunnel leads to valve FK";
            let expected = ("AA".to_string(), 22, vec!["FK".to_string()]);
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
