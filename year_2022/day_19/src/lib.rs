use regex::Regex;
use std::cmp;

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

#[derive(Debug, Default)]
struct State {
    max_resource_amount: u32,
    max_collector_count: u32,
}

fn find_max_geode_count(blueprint: &Blueprint) -> u32 {
    const TIME_LIMIT: usize = 24;
    let collector_cost = blueprint.collector_costs_by_material(Material::Ore)[0].amount;
    let mut states: [State; TIME_LIMIT] = Default::default();
    for minute in 1..TIME_LIMIT {
        states[minute].max_resource_amount = states[minute - 1].max_resource_amount + 1;
    }
    let mut collector_count = 1;
    let mut collected = 0;
    for minute in 0..TIME_LIMIT {
        states[minute].max_collector_count = collector_count;
        collected += collector_count;
        if collected > collector_cost {
            collector_count += 1;
            collected -= collector_cost;
            let mut future_collected = collected;
            for state in &mut states[minute + 1..] {
                future_collected += collector_count;
                state.max_resource_amount = cmp::max(state.max_resource_amount, future_collected);
            }
        }
    }
    dbg!(&states);
    // for minute in 1..TIME_LIMIT {
    //     let new_collector_count = if ore_states[minute - 1].max_resource_amount / collector_cost
    //         > ore_states[minute - 1].max_collector_count
    //     {
    //         ore_states[minute - 1].max_collector_count + 1
    //     } else {
    //         ore_states[minute - 1].max_collector_count
    //     };
    //     ore_states[minute].max_collector_count = new_collector_count;
    //     if ore_states[minute].max_collector_count > ore_states[minute - 1].max_collector_count {
    //         for m in minute + 1..TIME_LIMIT {
    //             ore_states[m].max_resource_amount = cmp::max(
    //                 ore_states[m - 1].max_resource_amount + ore_states[minute].max_collector_count,
    //                 ore_states[m].max_resource_amount,
    //             );
    //             ore_states[m].max_collector_count = ore_states[minute].max_collector_count;
    //         }
    //     }
    // }
    // dbg!(ore_states);
    todo!()
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
