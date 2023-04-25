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
        .map(|b| dbg!(b.id) * dbg!(find_max_geode_count(b, 24)))
        .sum()
}

pub fn part_two(blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .take(3)
        .map(|b| dbg!(find_max_geode_count(b, 32)))
        .product()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
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
    fn material_count(&self, material: Material) -> u32 {
        match material {
            Material::Ore => self.ore_count,
            Material::Clay => self.clay_count,
            Material::Obsidian => self.obsidian_count,
            Material::Geode => self.geode_count,
        }
    }

    fn material_count_mut(&mut self, material: Material) -> &mut u32 {
        match material {
            Material::Ore => &mut self.ore_count,
            Material::Clay => &mut self.clay_count,
            Material::Obsidian => &mut self.obsidian_count,
            Material::Geode => &mut self.geode_count,
        }
    }

    fn material_collector_count(&self, material: Material) -> u32 {
        match material {
            Material::Ore => self.ore_collectors,
            Material::Clay => self.clay_collectors,
            Material::Obsidian => self.obsidian_collectors,
            Material::Geode => self.geode_collectors,
        }
    }

    fn material_collector_count_mut(&mut self, material: Material) -> &mut u32 {
        match material {
            Material::Ore => &mut self.ore_collectors,
            Material::Clay => &mut self.clay_collectors,
            Material::Obsidian => &mut self.obsidian_collectors,
            Material::Geode => &mut self.geode_collectors,
        }
    }
}

fn find_max_geode_count(blueprint: &Blueprint, time_limit: u32) -> u32 {
    let initial_state = State {
        ore_collectors: 1,
        ore_count: 0,
        clay_count: 0,
        obsidian_count: 0,
        geode_count: 0,
        clay_collectors: 0,
        obsidian_collectors: 0,
        geode_collectors: 0,
    };
    let materials = [
        Material::Ore,
        Material::Clay,
        Material::Obsidian,
        Material::Geode,
    ];
    let mut states = [initial_state].into_iter().collect::<HashSet<_>>();
    for minute in 1..=time_limit {
        let mut new_states = HashSet::default();
        for prev in states {
            let mut after_tick = prev.clone();
            for material in materials {
                *after_tick.material_count_mut(material) += prev.material_collector_count(material);
            }
            for material in materials {
                let costs = blueprint.collector_costs_by_material(material);
                if costs
                    .iter()
                    .all(|c| prev.material_count(c.material) >= c.amount)
                {
                    let mut add_collector = after_tick.clone();
                    for cost in costs.iter() {
                        *add_collector.material_count_mut(cost.material) -= cost.amount;
                    }
                    *add_collector.material_collector_count_mut(material) += 1;
                    new_states.insert(add_collector);
                }
            }
            new_states.insert(after_tick);
        }
        states = new_states;
        dbg!(minute, states.len());
    }
    states.iter().map(|s| s.geode_count).max().unwrap()
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
