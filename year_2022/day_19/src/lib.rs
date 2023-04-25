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
    // fn time_until_afford_cost(&self, cost: Cost) -> Option<u32> {
    //     let (current_amount, collector_count) = match cost.material {
    //         Material::Ore => (self.ore_count, self.ore_collectors),
    //         Material::Clay => (self.clay_count, self.clay_collectors),
    //         Material::Obsidian => (self.obsidian_count, self.obsidian_collectors),
    //         Material::Geode => (self.geode_count, self.geode_collectors),
    //     };
    //     if current_amount >= cost.amount {
    //         Some(0)
    //     } else if collector_count == 0 {
    //         None
    //     } else {
    //         let needed = cost.amount - current_amount;
    //         let quotient = needed / collector_count;
    //         let remainder = needed % collector_count;
    //         let time = if remainder == 0 {
    //             quotient
    //         } else {
    //             quotient + 1
    //         };
    //         Some(time)
    //     }
    // }
    //
    // fn can_reach_geode_collector(&self, blueprint: &Blueprint) -> bool {
    //     let mut state = self.clone();
    //     if state.clay_collectors == 0 {
    //         if let Some(s) = state.build_collector(blueprint, Material::Clay) {
    //             state = s;
    //         } else {
    //             return false;
    //         }
    //     }
    //     if state.obsidian_collectors == 0 {
    //         if let Some(s) = state.build_collector(blueprint, Material::Obsidian) {
    //             state = s;
    //         } else {
    //             return false;
    //         }
    //     }
    //     state.build_collector(blueprint, Material::Geode).is_some()
    // }
    //
    // fn build_collector(&self, blueprint: &Blueprint, collector_material: Material) -> Option<Self> {
    //     let costs = blueprint.collector_costs_by_material(collector_material);
    //     let times = costs
    //         .iter()
    //         .map(|c| self.time_until_afford_cost(*c))
    //         .collect::<Option<Vec<_>>>()?;
    //     let time_to_build = times.into_iter().max().unwrap();
    //     if time_to_build >= self.remaining_time {
    //         return None;
    //     }
    //     let mut next_state = self.clone();
    //     next_state.remaining_time -= time_to_build;
    //     next_state.ore_count += time_to_build * self.ore_collectors;
    //     next_state.clay_count += time_to_build * self.clay_collectors;
    //     next_state.obsidian_count += time_to_build * self.obsidian_collectors;
    //     next_state.geode_count += time_to_build * self.geode_collectors;
    //     for cost in costs.iter() {
    //         match cost.material {
    //             Material::Ore => next_state.ore_count -= cost.amount,
    //             Material::Clay => next_state.clay_count -= cost.amount,
    //             Material::Obsidian => next_state.obsidian_count -= cost.amount,
    //             Material::Geode => next_state.geode_count -= cost.amount,
    //         }
    //     }
    //     match collector_material {
    //         Material::Ore => next_state.ore_collectors += 1,
    //         Material::Clay => next_state.clay_collectors += 1,
    //         Material::Obsidian => next_state.obsidian_collectors += 1,
    //         Material::Geode => next_state.geode_collectors += 1,
    //     }
    //     Some(next_state)
    // }
    //
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

fn find_max_geode_count(blueprint: &Blueprint) -> u32 {
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
    let mut states = vec![initial_state];
    for minute in 1..=3 {
        dbg!(minute);
        for state in states.iter_mut() {
            let mut updated_state = state.clone();
            for material in materials {
                let costs = blueprint.collector_costs_by_material(material);
                *updated_state.material_count_mut(material) +=
                    state.material_collector_count(material);
                if costs
                    .iter()
                    .all(|c| state.material_count(c.material) >= c.amount)
                {
                    for cost in costs.iter() {
                        *updated_state.material_count_mut(cost.material) -= cost.amount;
                    }
                    *updated_state.material_collector_count_mut(material) += 1;
                }
            }
            dbg!(&state, &updated_state);
            *state = updated_state;
        }
    }
    todo!()
    // let mut states = [initial_state].into_iter().collect::<HashSet<_>>();
    // for t in (0..initial_remaining_time).rev() {
    //     let mut new_states = HashSet::default();
    //     for state in states.iter() {
    //         let elapsed_time = state.remaining_time - t;
    //         let mut state_at_t = state.clone();
    //         state_at_t.remaining_time = t;
    //         for material in materials {
    //             *state_at_t.material_count_mut(material) +=
    //                 state_at_t.material_collector_count(material) * elapsed_time;
    //         }
    //         for material in materials {
    //             let costs = blueprint.collector_costs_by_material(material);
    //             if costs
    //                 .iter()
    //                 .all(|c| state_at_t.material_count(c.material) >= c.amount)
    //             {
    //                 let mut new_state = state_at_t.clone();
    //                 for cost in costs.iter() {
    //                     *new_state.material_count_mut(cost.material) -= cost.amount;
    //                 }
    //                 *new_state.material_collector_count_mut(material) += 1;
    //                 if new_state.can_reach_geode_collector(blueprint) {
    //                     new_states.insert(new_state);
    //                 }
    //             }
    //         }
    //     }
    //     states.extend(new_states);
    //     dbg!(&states, t);
    // }
    // states.into_iter().map(|s| s.geode_count).max().unwrap()
    // ------------------------
    // for new_collector_material in materials {
    //     let costs = blueprint.collector_costs_by_material(new_collector_material);
    //     let mut next_states = states.clone();
    //     for mut state in states {
    //         for _ in 0..state.remaining_time {
    //             state.remaining_time -= 1;
    //             let can_afford_collector = costs
    //                 .iter()
    //                 .all(|c| state.material_count(c.material) >= c.amount);
    //             if can_afford_collector {
    //                 for cost in costs.iter() {
    //                     *state.material_count_mut(cost.material) -= cost.amount;
    //                 }
    //             }
    //             for material in materials {
    //                 *state.material_count_mut(material) += state.material_collector_count(material);
    //             }
    //             if can_afford_collector {
    //                 *state.material_collector_count_mut(new_collector_material) += 1;
    //             }
    //             next_states.push(state.clone());
    //         }
    //     }
    //     states = next_states;
    // }
    // dbg!(states
    //     .iter()
    //     .filter(|s| s.remaining_time == 12
    //         && s.ore_collectors == 1
    //         && s.ore_count == 1
    //         && s.clay_collectors == 3
    //         && s.clay_count == 7
    //         && s.obsidian_collectors == 1)
    //     .count());
    // states.into_iter().map(|s| s.geode_count).max().unwrap_or(0)
    // ------------------------
    // let mut state = initial_state.clone();
    // let ore_collector_costs = blueprint.collector_costs_by_material(Material::Ore);
    // for _ in 0..initial_state.remaining_time {
    //     if ore_collector_costs
    //         .iter()
    //         .all(|c| state.material_count(c.material) >= c.amount)
    //     {
    //         for cost in ore_collector_costs.iter() {
    //             *state.material_count_mut(cost.material) -= cost.amount;
    //         }
    //         state.ore_collectors += 1;
    //         states.push(state.clone());
    //     }
    //     state.remaining_time -= 1;
    //     state.ore_count += state.ore_collectors;
    // }
    // ----------------------
    // let mut stack = vec![initial_state];
    // while let Some(state) = stack.pop() {
    //     let next_collector_materials: Box<[Material]> = if state.clay_collectors == 0 {
    //         Box::new([Material::Ore, Material::Clay])
    //     } else if state.obsidian_collectors == 0 {
    //         Box::new([Material::Clay, Material::Obsidian])
    //     } else if state.geode_collectors == 0 {
    //         Box::new([Material::Obsidian, Material::Geode])
    //     } else {
    //         Box::new([Material::Geode])
    //     };
    //     let mut next_states = next_collector_materials
    //         .iter()
    //         .filter_map(|m| {
    //             state
    //                 .build_collector(blueprint, *m)
    //                 .filter(|s| s.can_reach_geode_collector(blueprint))
    //         })
    //         .peekable();
    //     if next_states.peek().is_none() {
    //         let final_geode_count =
    //             state.geode_count + state.geode_collectors * state.remaining_time;
    //         dbg!(&state, final_geode_count);
    //         max_geode_count = max_geode_count.max(final_geode_count);
    //     }
    //     stack.extend(next_states);
    // }
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
