use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::cmp;
use std::rc::Rc;

pub const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq)]
pub struct Valve {
    flow_rate: u32,
    tunnel_ids: Vec<String>,
}

type Valves = HashMap<String, Valve>;

pub fn part_one(valves: &Valves) -> u32 {
    struct State<'a> {
        position: &'a str,
        remaining_minutes: u32,
        releasable_valve_ids: Rc<HashSet<&'a str>>,
        released_pressure: u32,
    }
    let distances = find_all_pairs_shortest_paths(valves);
    let releasable_valve_ids = Rc::new(
        valves
            .iter()
            .filter(|(_, valve)| valve.flow_rate > 0)
            .map(|(id, _)| id.as_str())
            .collect::<HashSet<_>>(),
    );
    let mut max_released_pressure = 0;
    let initial_state = State {
        position: "AA",
        remaining_minutes: 30,
        releasable_valve_ids,
        released_pressure: 0,
    };
    let mut stack = vec![initial_state];
    while let Some(mut state) = stack.pop() {
        let mut is_solution = true;
        Rc::make_mut(&mut state.releasable_valve_ids).remove(state.position);
        for &id in state.releasable_valve_ids.iter() {
            let distance = distances[&(state.position, id)];
            let minutes_to_release = distance + 1;
            if state.remaining_minutes > minutes_to_release {
                let remaining_minutes = state.remaining_minutes - minutes_to_release;
                let released_pressure =
                    state.released_pressure + remaining_minutes * valves[id].flow_rate;
                let next_state = State {
                    position: id,
                    remaining_minutes,
                    releasable_valve_ids: Rc::clone(&state.releasable_valve_ids),
                    released_pressure,
                };
                stack.push(next_state);
                is_solution = false;
            }
        }
        if is_solution {
            max_released_pressure = cmp::max(max_released_pressure, state.released_pressure);
        }
    }
    max_released_pressure
}

pub fn part_two(valves: &Valves) -> u32 {
    struct State<'a> {
        own_position: &'a str,
        own_remaining_minutes: u32,
        elephant_position: &'a str,
        elephant_remaining_minutes: u32,
        releasable_valve_ids: Rc<HashSet<&'a str>>,
        released_pressure: u32,
    }
    let distances = find_all_pairs_shortest_paths(valves);
    let releasable_valve_ids = Rc::new(
        valves
            .iter()
            .filter(|(_, valve)| valve.flow_rate > 0)
            .map(|(id, _)| id.as_str())
            .collect::<HashSet<_>>(),
    );
    let mut max_released_pressure = 0;
    let initial_state = State {
        own_position: "AA",
        own_remaining_minutes: 26,
        elephant_position: "AA",
        elephant_remaining_minutes: 26,
        releasable_valve_ids,
        released_pressure: 0,
    };
    let mut stack = vec![initial_state];
    while let Some(mut state) = stack.pop() {
        let mut is_solution = true;
        Rc::make_mut(&mut state.releasable_valve_ids).remove(state.own_position);
        Rc::make_mut(&mut state.releasable_valve_ids).remove(state.elephant_position);
        for &id in state.releasable_valve_ids.iter() {
            let distance = distances[&(state.own_position, id)];
            let minutes_to_release = distance + 1;
            if state.own_remaining_minutes > minutes_to_release {
                let remaining_minutes = state.own_remaining_minutes - minutes_to_release;
                let released_pressure =
                    state.released_pressure + remaining_minutes * valves[id].flow_rate;
                let next_state = State {
                    own_position: id,
                    own_remaining_minutes: remaining_minutes,
                    elephant_position: state.elephant_position,
                    elephant_remaining_minutes: state.elephant_remaining_minutes,
                    releasable_valve_ids: Rc::clone(&state.releasable_valve_ids),
                    released_pressure,
                };
                stack.push(next_state);
                is_solution = false;
            }
            let distance = distances[&(state.elephant_position, id)];
            let minutes_to_release = distance + 1;
            if state.elephant_remaining_minutes > minutes_to_release {
                let remaining_minutes = state.elephant_remaining_minutes - minutes_to_release;
                let released_pressure =
                    state.released_pressure + remaining_minutes * valves[id].flow_rate;
                let next_state = State {
                    own_position: state.own_position,
                    own_remaining_minutes: state.own_remaining_minutes,
                    elephant_position: id,
                    elephant_remaining_minutes: remaining_minutes,
                    releasable_valve_ids: Rc::clone(&state.releasable_valve_ids),
                    released_pressure,
                };
                stack.push(next_state);
                is_solution = false;
            }
        }
        if is_solution {
            max_released_pressure = cmp::max(max_released_pressure, state.released_pressure);
        }
    }
    max_released_pressure
}

// Floyd-Warshall algorithm
// https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
// This could be replaced with BFS from each valve to every other valve to make it O(v^2 + ve)
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
        assert_eq!(part_one(&valves), 2320);
    }

    #[test]
    fn test_part_two() {
        let valves = parser::parse(INPUT).unwrap();
        assert_eq!(part_two(&valves), 2967);
    }
}
