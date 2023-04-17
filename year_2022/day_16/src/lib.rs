use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::cmp::{self, Reverse};

pub const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq)]
pub struct Valve {
    flow_rate: u32,
    tunnel_ids: Vec<String>,
}

type Valves = HashMap<String, Valve>;
type Distances<'a> = HashMap<(&'a str, &'a str), u32>;

struct State<'a> {
    position: &'a str,
    remaining_minutes: u32,
    releasable_valve_ids: HashSet<&'a str>,
    released_valve_ids: HashSet<&'a str>,
    released_pressure: u32,
}

struct Solutions<'a> {
    valves: &'a Valves,
    distances: &'a Distances<'a>,
    stack: Vec<State<'a>>,
}

impl<'a> Solutions<'a> {
    fn new(
        valves: &'a Valves,
        releasable_valve_ids: &'a HashSet<&'a str>,
        distances: &'a Distances,
        time_limit: u32,
    ) -> Self {
        let initial_state = State {
            position: "AA",
            remaining_minutes: time_limit,
            releasable_valve_ids: releasable_valve_ids.clone(),
            released_valve_ids: HashSet::default(),
            released_pressure: 0,
        };
        Self {
            valves,
            distances,
            stack: vec![initial_state],
        }
    }
}

impl<'a> Iterator for Solutions<'a> {
    type Item = (u32, HashSet<&'a str>);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(state) = self.stack.pop() {
            let mut is_solution = true;
            for &id in state.releasable_valve_ids.iter() {
                let distance = self.distances[&(state.position, id)];
                let minutes_to_release = distance + 1;
                if state.remaining_minutes > minutes_to_release {
                    let remaining_minutes = state.remaining_minutes - minutes_to_release;
                    let released_pressure =
                        state.released_pressure + remaining_minutes * self.valves[id].flow_rate;
                    let mut released_valve_ids = state.released_valve_ids.clone();
                    released_valve_ids.insert(id);
                    let mut releasable_valve_ids = state.releasable_valve_ids.clone();
                    releasable_valve_ids.remove(id);
                    let next_state = State {
                        position: id,
                        remaining_minutes,
                        releasable_valve_ids,
                        released_valve_ids,
                        released_pressure,
                    };
                    self.stack.push(next_state);
                    is_solution = false;
                }
            }
            if is_solution {
                return Some((state.released_pressure, state.released_valve_ids));
            }
        }
        None
    }
}

pub fn preprocess(valves: &Valves) -> (Distances, HashSet<&str>) {
    let distances = find_all_pairs_shortest_paths(valves);
    let releasable_valve_ids = valves
        .iter()
        .filter(|(_, valve)| valve.flow_rate > 0)
        .map(|(id, _)| id.as_str())
        .collect::<HashSet<_>>();
    (distances, releasable_valve_ids)
}

pub fn part_one(
    valves: &Valves,
    distances: &Distances,
    releasable_valve_ids: &HashSet<&str>,
) -> u32 {
    Solutions::new(valves, releasable_valve_ids, distances, 30)
        .map(|(pressure_released, _)| pressure_released)
        .max()
        .unwrap()
}

pub fn part_two(
    valves: &Valves,
    distances: &Distances,
    releasable_valve_ids: &HashSet<&str>,
) -> u32 {
    let mut solutions =
        Solutions::new(valves, releasable_valve_ids, distances, 26).collect::<Vec<_>>();
    solutions.sort_by_key(|(pressure_released, _)| Reverse(*pressure_released));
    let mut max = 0;
    for (i, (own_pressure, own_valves)) in solutions.iter().enumerate() {
        let total_pressure = &solutions[i + 1..]
            .iter()
            .map(|(p, valves)| (p + own_pressure, valves))
            .take_while(|(p, _)| *p > max)
            .find(|(_, valves)| own_valves.is_disjoint(valves))
            .map(|(p, _)| p);
        if let Some(p) = total_pressure {
            max = cmp::max(max, *p);
        }
    }
    max
}

// Floyd-Warshall algorithm
// https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
pub fn find_all_pairs_shortest_paths(valves: &Valves) -> HashMap<(&str, &str), u32> {
    let mut distances = HashMap::default();
    for (valve_id, valve) in valves {
        distances.insert((valve_id.as_str(), valve_id.as_str()), 0);
        for tunnel_id in valve.tunnel_ids.iter() {
            distances.insert((valve_id.as_str(), tunnel_id.as_str()), 1);
        }
    }
    let valve_strs = valves.keys().map(String::as_str).collect::<Vec<_>>();
    for &v in valve_strs.iter() {
        for &source in valve_strs.iter() {
            for &destination in valve_strs.iter() {
                let prev_distance = distances.get(&(source, destination)).copied();
                let new_distance = distances
                    .get(&(source, v))
                    .and_then(|d| distances.get(&(v, destination)).map(|dd| d + dd));
                match (prev_distance, new_distance) {
                    (None, Some(new)) => {
                        distances.insert((source, destination), new);
                    }
                    (Some(old), Some(new)) if new < old => {
                        distances.insert((source, destination), new);
                    }
                    _ => {}
                }
            }
        }
    }
    distances
}

pub mod parser {
    use super::{Valve, Valves};
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

    pub fn parse(input: &str) -> Result<Valves> {
        let (rest, valves) = valves(input)
            .finish()
            .map_err(|err| anyhow!(err.to_string()))?;
        if !rest.is_empty() {
            return Err(anyhow!("Unparsed input: {}", rest));
        }
        Ok(valves)
    }

    fn valve(input: &str) -> IResult<&str, (String, Valve)> {
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
            flow_rate,
            tunnel_ids,
        };
        Ok((input, (id, valve)))
    }

    fn valves(input: &str) -> IResult<&str, Valves> {
        let line = terminated(valve, line_ending);
        let mut iter = iterator(input, line);
        let valves = iter.collect::<Valves>();
        let (input, _) = iter.finish()?;
        Ok((input, valves))
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_valve() {
            let input = "Valve EF has flow rate=22; tunnels lead to valves FK, HT, DE";
            let expected_valve = Valve {
                flow_rate: 22,
                tunnel_ids: vec!["FK".to_string(), "HT".to_string(), "DE".to_string()],
            };
            assert_eq!(
                valve(input).unwrap(),
                ("", ("EF".to_string(), expected_valve))
            );
            let input = "Valve AA has flow rate=22; tunnel leads to valve FK";
            let expected_valve = Valve {
                flow_rate: 22,
                tunnel_ids: vec!["FK".to_string()],
            };
            assert_eq!(
                valve(input).unwrap(),
                ("", ("AA".to_string(), expected_valve))
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let valves = parser::parse(INPUT).unwrap();
        let (distances, releasable_valve_ids) = preprocess(&valves);
        assert_eq!(part_one(&valves, &distances, &releasable_valve_ids), 2320);
    }

    #[test]
    fn test_part_two() {
        let valves = parser::parse(INPUT).unwrap();
        let (distances, releasable_valve_ids) = preprocess(&valves);
        assert_eq!(part_two(&valves, &distances, &releasable_valve_ids), 2967);
    }
}
