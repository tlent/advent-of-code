use std::collections::HashMap;
use std::collections::HashSet;

static INPUT: &str = include_str!("../input");

fn main() {
    let data = parse_input(INPUT);
    let ingredients = data
        .iter()
        .fold(HashSet::new(), |result, (ingredients, _)| {
            &result | ingredients
        });
    let allergens = data
        .iter()
        .fold(HashSet::new(), |result, (_, allergens)| &result | allergens);
    let mut ingredients_by_allergen = HashMap::new();
    for a in allergens.iter() {
        ingredients_by_allergen.insert(
            a,
            data.iter()
                .fold(ingredients.clone(), |result, (ingredients, allergens)| {
                    if allergens.contains(a) {
                        &result & ingredients
                    } else {
                        result
                    }
                }),
        );
    }
    let unsafe_ingredients = allergens.iter().fold(HashSet::new(), |result, a| {
        &result | &ingredients_by_allergen[a]
    });
    let part_one = data
        .iter()
        .map(|(ingredients, _)| (ingredients - &unsafe_ingredients).len())
        .sum::<usize>() as u32;
    println!("part one: {}", part_one);
    let mut matched_pairs = Vec::with_capacity(allergens.len());
    while !ingredients_by_allergen.is_empty() {
        for (a, ingredients) in ingredients_by_allergen.iter() {
            if ingredients.len() == 1 {
                let i = ingredients.iter().nth(0).unwrap();
                matched_pairs.push((a.clone(), i.clone()));
            }
        }
        for (a, i) in matched_pairs.iter() {
            ingredients_by_allergen.remove(a);
            for ingredient_set in ingredients_by_allergen.values_mut() {
                ingredient_set.remove(i);
            }
        }
    }
    matched_pairs.sort_by_key(|&(a, _)| a);
    let part_two = matched_pairs
        .into_iter()
        .map(|(_, i)| i)
        .collect::<Vec<_>>()
        .join(",");
    println!("part two: {}", part_two);
}

fn parse_input(s: &str) -> Vec<(HashSet<String>, HashSet<String>)> {
    s.lines()
        .map(|l| {
            let mut parts = l.trim_end_matches(')').split(" (contains ");
            let ingredients = parts.next().unwrap().split(' ').map(String::from).collect();
            let allergens = parts
                .next()
                .unwrap()
                .split(", ")
                .map(String::from)
                .collect();
            (ingredients, allergens)
        })
        .collect()
}
