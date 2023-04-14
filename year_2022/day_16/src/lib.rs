use rustc_hash::FxHashMap as HashMap;

pub const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq)]
pub struct Valve {
    flow_rate: u32,
    tunnel_ids: Vec<String>,
}

type Valves = HashMap<String, Valve>;

pub fn part_one(valves: &Valves) -> u32 {
    let start_valve = &valves["AA"];
    f(valves, start_valve, 30)
}

fn f(valves: &Valves, current_valve: &Valve, remaining_minutes: u32) -> u32 {
    if remaining_minutes == 0 {
        return 0;
    }
    let next_valves = current_valve.tunnel_ids.iter().map(|id| &valves[id]);
    todo!()
}

pub fn part_two() -> () {
    todo!()
}

pub mod parser {
    use super::{Valve, Valves};
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        character::complete::{line_ending, u32},
        combinator::{iterator, map},
        multi::{many0, separated_list1},
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
        todo!()
    }

    #[test]
    fn test_part_two() {
        todo!()
    }
}
