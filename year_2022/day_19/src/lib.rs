use regex::Regex;
use rustc_hash::FxHashSet as HashSet;
use std::cmp;

pub const INPUT: &str = include_str!("../input.txt");

pub fn parse_input(input: &str) -> Vec<Blueprint> {
    let regex_str = concat!(
        r"Blueprint (\d+): ",
        r"Each ore robot costs (\d+) ore. ",
        r"Each clay robot costs (\d+) ore. ",
        r"Each obsidian robot costs (\d+) ore and (\d+) clay. ",
        r"Each geode robot costs (\d+) ore and (\d+) obsidian."
    );
    let regex = Regex::new(regex_str).unwrap();
    regex
        .captures_iter(input)
        .map(|captures| {
            let id = captures[1].parse().unwrap();
            let ore_collector_cost = Cost {
                amount: captures[2].parse().unwrap(),
                resource: Resource::Ore,
            };
            let clay_collector_cost = Cost {
                amount: captures[3].parse().unwrap(),
                resource: Resource::Ore,
            };
            let obsidian_collector_costs = [
                Cost {
                    amount: captures[4].parse().unwrap(),
                    resource: Resource::Ore,
                },
                Cost {
                    amount: captures[5].parse().unwrap(),
                    resource: Resource::Clay,
                },
            ];
            let geode_collector_costs = [
                Cost {
                    amount: captures[6].parse().unwrap(),
                    resource: Resource::Ore,
                },
                Cost {
                    amount: captures[7].parse().unwrap(),
                    resource: Resource::Obsidian,
                },
            ];
            Blueprint::new(
                id,
                Box::new([ore_collector_cost]),
                Box::new([clay_collector_cost]),
                Box::new(obsidian_collector_costs),
                Box::new(geode_collector_costs),
            )
        })
        .collect()
}

pub fn part_one(blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .map(|b| b.id * find_max_geode_count(b, 24))
        .sum()
}

pub fn part_two(blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .take(3)
        .map(|b| find_max_geode_count(b, 32))
        .product()
}

fn find_max_geode_count(blueprint: &Blueprint, time_limit: u32) -> u32 {
    let initial_state = State {
        remaining_time: time_limit,
        ore: ResourceState {
            amount: 0,
            collector_count: 1,
        },
        ..Default::default()
    };
    let mut max_geode_count = 0;
    let mut seen = HashSet::default();
    let mut states = vec![initial_state];
    while let Some(state) = states.pop() {
        for r in Resource::iter() {
            let collector_count = state.resource(r).collector_count;
            let ResourceBlueprint {
                collector_costs,
                max_useful_collectors,
            } = blueprint.resource(r);
            let capped = max_useful_collectors.map_or(false, |c| collector_count >= c);
            if capped {
                continue;
            }
            let time_to_afford = state.time_to_afford(collector_costs);
            if time_to_afford.is_none() {
                continue;
            }
            let upper_bound = state.upper_bound_geode_count();
            if upper_bound < max_geode_count {
                continue;
            }
            let time_to_build = 1 + time_to_afford.unwrap();
            if time_to_build >= state.remaining_time {
                let final_geode_count =
                    state.geode.amount + state.remaining_time * state.geode.collector_count;
                max_geode_count = cmp::max(max_geode_count, final_geode_count);
                continue;
            }
            let mut new_state = state.clone();
            new_state.remaining_time -= time_to_build;
            for r in Resource::iter() {
                new_state.resource_mut(r).amount +=
                    time_to_build * state.resource(r).collector_count;
            }
            for cost in collector_costs.iter() {
                new_state.resource_mut(cost.resource).amount -= cost.amount;
            }
            new_state.resource_mut(r).collector_count += 1;
            if !seen.contains(&new_state) {
                seen.insert(new_state.clone());
                states.push(new_state);
            }
        }
    }
    max_geode_count
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cost {
    amount: u32,
    resource: Resource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Resource {
    fn iter() -> impl Iterator<Item = Resource> {
        [
            Resource::Ore,
            Resource::Clay,
            Resource::Obsidian,
            Resource::Geode,
        ]
        .into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Blueprint {
    id: u32,
    ore: ResourceBlueprint,
    clay: ResourceBlueprint,
    obsidian: ResourceBlueprint,
    geode: ResourceBlueprint,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceBlueprint {
    collector_costs: Box<[Cost]>,
    max_useful_collectors: Option<u32>,
}

impl Blueprint {
    fn new(
        id: u32,
        ore_collector_costs: Box<[Cost]>,
        clay_collector_costs: Box<[Cost]>,
        obsidian_collector_costs: Box<[Cost]>,
        geode_collector_costs: Box<[Cost]>,
    ) -> Self {
        let all_costs = [
            ore_collector_costs.iter().copied(),
            clay_collector_costs.iter().copied(),
            obsidian_collector_costs.iter().copied(),
            geode_collector_costs.iter().copied(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
        let max_useful_collectors = |r| {
            all_costs
                .iter()
                .filter(|c| c.resource == r)
                .map(|c| c.amount)
                .max()
        };
        Blueprint {
            id,
            ore: ResourceBlueprint {
                collector_costs: ore_collector_costs,
                max_useful_collectors: max_useful_collectors(Resource::Ore),
            },
            clay: ResourceBlueprint {
                collector_costs: clay_collector_costs,
                max_useful_collectors: max_useful_collectors(Resource::Clay),
            },
            obsidian: ResourceBlueprint {
                collector_costs: obsidian_collector_costs,
                max_useful_collectors: max_useful_collectors(Resource::Obsidian),
            },
            geode: ResourceBlueprint {
                collector_costs: geode_collector_costs,
                max_useful_collectors: max_useful_collectors(Resource::Geode),
            },
        }
    }

    fn resource(&self, resource: Resource) -> &ResourceBlueprint {
        match resource {
            Resource::Ore => &self.ore,
            Resource::Clay => &self.clay,
            Resource::Obsidian => &self.obsidian,
            Resource::Geode => &self.geode,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct State {
    remaining_time: u32,
    ore: ResourceState,
    clay: ResourceState,
    obsidian: ResourceState,
    geode: ResourceState,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct ResourceState {
    amount: u32,
    collector_count: u32,
}

impl State {
    fn resource(&self, resource: Resource) -> &ResourceState {
        match resource {
            Resource::Ore => &self.ore,
            Resource::Clay => &self.clay,
            Resource::Obsidian => &self.obsidian,
            Resource::Geode => &self.geode,
        }
    }

    fn resource_mut(&mut self, resource: Resource) -> &mut ResourceState {
        match resource {
            Resource::Ore => &mut self.ore,
            Resource::Clay => &mut self.clay,
            Resource::Obsidian => &mut self.obsidian,
            Resource::Geode => &mut self.geode,
        }
    }

    fn time_to_afford(&self, costs: &[Cost]) -> Option<u32> {
        let times = costs
            .iter()
            .map(|c| {
                let amount = self.resource(c.resource).amount;
                let collector_count = self.resource(c.resource).collector_count;
                if amount >= c.amount {
                    return Some(0);
                } else if collector_count == 0 {
                    return None;
                }
                let needed = c.amount - amount;
                let t = if needed % collector_count == 0 {
                    needed / collector_count
                } else {
                    needed / collector_count + 1
                };
                Some(t)
            })
            .collect::<Option<Vec<_>>>()?;
        times.into_iter().max()
    }

    fn upper_bound_geode_count(&self) -> u32 {
        let mut upper_bound = self.geode.amount;
        for m in 0..self.remaining_time {
            upper_bound += self.geode.collector_count + m;
        }
        upper_bound
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let blueprints = parse_input(INPUT);
        assert_eq!(part_one(&blueprints), 1659);
    }

    #[test]
    fn test_part_two() {
        let blueprints = parse_input(INPUT);
        assert_eq!(part_two(&blueprints), 6804);
    }
}
