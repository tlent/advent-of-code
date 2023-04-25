use regex::Regex;
use rustc_hash::FxHashSet as HashSet;

pub const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Blueprint {
    id: u32,
    ore_collector_costs: Box<[Cost]>,
    clay_collector_costs: Box<[Cost]>,
    obsidian_collector_costs: Box<[Cost]>,
    geode_collector_costs: Box<[Cost]>,
}

impl Blueprint {
    fn collector_costs_by_material(&self, material: Material) -> &[Cost] {
        match material {
            Material::Ore => &self.ore_collector_costs,
            Material::Clay => &self.clay_collector_costs,
            Material::Obsidian => &self.obsidian_collector_costs,
            Material::Geode => &self.geode_collector_costs,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cost {
    amount: u32,
    material: Material,
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
            let ore_collector_costs = Box::new([Cost {
                amount: captures[2].parse().unwrap(),
                material: Material::Ore,
            }]);
            let clay_collector_costs = Box::new([Cost {
                amount: captures[3].parse().unwrap(),
                material: Material::Ore,
            }]);
            let obsidian_collector_costs = Box::new([
                Cost {
                    amount: captures[4].parse().unwrap(),
                    material: Material::Ore,
                },
                Cost {
                    amount: captures[5].parse().unwrap(),
                    material: Material::Clay,
                },
            ]);
            let geode_collector_costs = Box::new([
                Cost {
                    amount: captures[6].parse().unwrap(),
                    material: Material::Ore,
                },
                Cost {
                    amount: captures[7].parse().unwrap(),
                    material: Material::Obsidian,
                },
            ]);
            Blueprint {
                id,
                ore_collector_costs,
                clay_collector_costs,
                obsidian_collector_costs,
                geode_collector_costs,
            }
        })
        .collect()
}

pub fn part_one(blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .map(|b| dbg!(b.id) * dbg!(find_max_geode_count(b)))
        .sum()
}

pub fn part_two() -> () {
    todo!()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    remaining_time: u32,
    ore_count: u32,
    clay_count: u32,
    obsidian_count: u32,
    geode_count: u32,
    ore_collectors: u32,
    clay_collectors: u32,
    obsidian_collectors: u32,
    geode_collectors: u32,
}

impl State {
    fn time_until_afford_cost(&self, cost: Cost) -> Option<u32> {
        let (current_amount, collector_count) = match cost.material {
            Material::Ore => (self.ore_count, self.ore_collectors),
            Material::Clay => (self.clay_count, self.clay_collectors),
            Material::Obsidian => (self.obsidian_count, self.obsidian_collectors),
            Material::Geode => (self.geode_count, self.geode_collectors),
        };
        if current_amount >= cost.amount {
            Some(0)
        } else if collector_count == 0 {
            None
        } else {
            let needed = cost.amount - current_amount;
            let quotient = needed / collector_count;
            let remainder = needed % collector_count;
            let time = if remainder == 0 {
                quotient
            } else {
                quotient + 1
            };
            Some(time)
        }
    }

    fn can_reach_geode_collector(&self, blueprint: &Blueprint) -> bool {
        let mut state = self.clone();
        if state.clay_collectors == 0 {
            if let Some(s) = state.build_collector(blueprint, Material::Clay) {
                state = s;
            } else {
                return false;
            }
        }
        if state.obsidian_collectors == 0 {
            if let Some(s) = state.build_collector(blueprint, Material::Obsidian) {
                state = s;
            } else {
                return false;
            }
        }
        state.build_collector(blueprint, Material::Geode).is_some()
    }

    fn build_collector(&self, blueprint: &Blueprint, collector_material: Material) -> Option<Self> {
        let costs = blueprint.collector_costs_by_material(collector_material);
        let times = costs
            .iter()
            .map(|c| self.time_until_afford_cost(*c))
            .collect::<Option<Vec<_>>>()?;
        let time_to_build = times.into_iter().max().unwrap();
        if time_to_build >= self.remaining_time {
            return None;
        }
        let mut next_state = self.clone();
        next_state.remaining_time -= time_to_build;
        next_state.ore_count += time_to_build * self.ore_collectors;
        next_state.clay_count += time_to_build * self.clay_collectors;
        next_state.obsidian_count += time_to_build * self.obsidian_collectors;
        next_state.geode_count += time_to_build * self.geode_collectors;
        for cost in costs.iter() {
            match cost.material {
                Material::Ore => next_state.ore_count -= cost.amount,
                Material::Clay => next_state.clay_count -= cost.amount,
                Material::Obsidian => next_state.obsidian_count -= cost.amount,
                Material::Geode => next_state.geode_count -= cost.amount,
            }
        }
        match collector_material {
            Material::Ore => next_state.ore_collectors += 1,
            Material::Clay => next_state.clay_collectors += 1,
            Material::Obsidian => next_state.obsidian_collectors += 1,
            Material::Geode => next_state.geode_collectors += 1,
        }
        Some(next_state)
    }
}

fn find_max_geode_count(blueprint: &Blueprint) -> u32 {
    let mut seen_states = HashSet::default();
    let mut max_geode_count = 0;
    let initial_state = State {
        remaining_time: 24,
        ore_collectors: 1,
        ore_count: 0,
        clay_count: 0,
        obsidian_count: 0,
        geode_count: 0,
        clay_collectors: 0,
        obsidian_collectors: 0,
        geode_collectors: 0,
    };
    let mut stack = vec![initial_state];
    while let Some(state) = stack.pop() {
        let next_collector_materials: Box<[Material]> = if state.clay_collectors == 0 {
            Box::new([Material::Ore, Material::Clay])
        } else if state.obsidian_collectors == 0 {
            Box::new([Material::Clay, Material::Obsidian])
        } else if state.geode_collectors == 0 {
            Box::new([Material::Obsidian, Material::Geode])
        } else {
            Box::new([Material::Geode])
        };
        let mut next_states = next_collector_materials
            .iter()
            .filter_map(|m| {
                state
                    .build_collector(blueprint, *m)
                    .filter(|s| s.can_reach_geode_collector(blueprint))
            })
            .peekable();
        if next_states.peek().is_none() {
            let final_geode_count =
                state.geode_count + state.geode_collectors * state.remaining_time;
            max_geode_count = max_geode_count.max(final_geode_count);
        }
        for next_state in next_states {
            if seen_states.insert(next_state.clone()) {
                stack.push(next_state);
            }
        }
    }
    max_geode_count
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
