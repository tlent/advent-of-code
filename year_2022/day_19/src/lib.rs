use regex::Regex;
use rustc_hash::FxHashSet as HashSet;
use std::mem;

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
    ore_state: MaterialState,
    clay_state: MaterialState,
    obsidian_state: MaterialState,
    geode_state: MaterialState,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct MaterialState {
    amount: u32,
    collector_count: u32,
    collector_built: bool,
}

impl State {
    fn material(&self, material: Material) -> &MaterialState {
        match material {
            Material::Ore => &self.ore_state,
            Material::Clay => &self.clay_state,
            Material::Obsidian => &self.obsidian_state,
            Material::Geode => &self.geode_state,
        }
    }

    fn material_mut(&mut self, material: Material) -> &mut MaterialState {
        match material {
            Material::Ore => &mut self.ore_state,
            Material::Clay => &mut self.clay_state,
            Material::Obsidian => &mut self.obsidian_state,
            Material::Geode => &mut self.geode_state,
        }
    }
}

fn find_max_geode_count(blueprint: &Blueprint, time_limit: u32) -> u32 {
    let materials = [
        Material::Ore,
        Material::Clay,
        Material::Obsidian,
        Material::Geode,
    ];
    let max_ore_collectors = materials
        .into_iter()
        .filter_map(|m| {
            blueprint
                .collector_costs_by_material(m)
                .iter()
                .filter(|c| c.material == Material::Ore)
                .map(|c| c.amount)
                .max()
        })
        .max()
        .unwrap();
    let max_clay_collectors = materials
        .into_iter()
        .filter_map(|m| {
            blueprint
                .collector_costs_by_material(m)
                .iter()
                .filter(|c| c.material == Material::Clay)
                .map(|c| c.amount)
                .max()
        })
        .max()
        .unwrap();
    let max_obsidian_collectors = materials
        .into_iter()
        .filter_map(|m| {
            blueprint
                .collector_costs_by_material(m)
                .iter()
                .filter(|c| c.material == Material::Obsidian)
                .map(|c| c.amount)
                .max()
        })
        .max()
        .unwrap();
    let mut initial_state = State::default();
    initial_state.ore_state.collector_count = 1;
    let mut prev_states = HashSet::default();
    let mut states = [initial_state].into_iter().collect::<HashSet<_>>();
    for _minute in 1..=time_limit {
        mem::swap(&mut prev_states, &mut states);
        for prev_state in prev_states.drain() {
            let mut next_state = prev_state.clone();
            for m in materials {
                next_state.material_mut(m).amount += next_state.material(m).collector_count;
            }
            for m in materials {
                if next_state.material(m).collector_built {
                    continue;
                }
                let collector_count = next_state.material(m).collector_count;
                let capped = match m {
                    Material::Ore => collector_count >= max_ore_collectors,
                    Material::Clay => collector_count >= max_clay_collectors,
                    Material::Obsidian => collector_count >= max_obsidian_collectors,
                    Material::Geode => false,
                };
                if capped {
                    continue;
                }
                let costs = blueprint.collector_costs_by_material(m);
                if costs
                    .iter()
                    .all(|c| prev_state.material(c.material).amount >= c.amount)
                {
                    next_state.material_mut(m).collector_built = true;
                    let mut new_state = next_state.clone();
                    for m in materials {
                        new_state.material_mut(m).collector_built = false;
                    }
                    for cost in costs.iter() {
                        new_state.material_mut(cost.material).amount -= cost.amount;
                    }
                    new_state.material_mut(m).collector_count += 1;
                    states.insert(new_state);
                }
            }
            if materials
                .into_iter()
                .any(|m| !next_state.material(m).collector_built)
            {
                states.insert(next_state);
            }
        }
    }
    states
        .iter()
        .map(|s| s.material(Material::Geode).amount)
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
        todo!()
    }
}
