use regex::Regex;
use rustc_hash::FxHashSet as HashSet;
use std::mem;

pub const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Blueprint {
    id: u32,
    ore: ResourceBlueprint,
    clay: ResourceBlueprint,
    obsidian: ResourceBlueprint,
    geode: ResourceBlueprint,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceBlueprint {
    collector_costs: Box<[Cost]>,
    max_useful_collectors: Option<u32>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cost {
    amount: u32,
    resource: Resource,
}

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct State {
    ore: ResourceState,
    clay: ResourceState,
    obsidian: ResourceState,
    geode: ResourceState,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct ResourceState {
    amount: u32,
    collector_count: u32,
    collector_built: bool,
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
}

fn find_max_geode_count(blueprint: &Blueprint, time_limit: u32) -> u32 {
    let mut initial_state = State::default();
    initial_state.ore.collector_count = 1;
    let mut prev_states = HashSet::default();
    let mut states = [initial_state].into_iter().collect::<HashSet<_>>();
    for _minute in 1..=time_limit {
        mem::swap(&mut prev_states, &mut states);
        for prev_state in prev_states.drain() {
            let mut next_state = prev_state.clone();
            for r in Resource::iter() {
                next_state.resource_mut(r).amount += next_state.resource(r).collector_count;
            }
            for r in Resource::iter() {
                if next_state.resource(r).collector_built {
                    continue;
                }
                let collector_count = next_state.resource(r).collector_count;
                let capped = blueprint
                    .resource(r)
                    .max_useful_collectors
                    .map_or(false, |c| collector_count >= c);
                if capped {
                    continue;
                }
                let costs = &blueprint.resource(r).collector_costs;
                if costs
                    .iter()
                    .all(|c| prev_state.resource(c.resource).amount >= c.amount)
                {
                    next_state.resource_mut(r).collector_built = true;
                    let mut new_state = next_state.clone();
                    for r in Resource::iter() {
                        new_state.resource_mut(r).collector_built = false;
                    }
                    for cost in costs.iter() {
                        new_state.resource_mut(cost.resource).amount -= cost.amount;
                    }
                    new_state.resource_mut(r).collector_count += 1;
                    states.insert(new_state);
                }
            }
            if Resource::iter().any(|r| !next_state.resource(r).collector_built) {
                states.insert(next_state);
            }
        }
    }
    states
        .iter()
        .map(|s| s.resource(Resource::Geode).amount)
        .max()
        .unwrap()
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
