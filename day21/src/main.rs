use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Item {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

fn main() {
    let input = include_str!("input.txt");
    let mut total_ingredients: HashSet<String> = HashSet::new();
    let mut total_allergens: HashSet<String> = HashSet::new();
    let mut items: Vec<Item> = Vec::new();
    for line in input.lines() {
        let mut spl = line.strip_suffix(")").unwrap().split(" (contains ");
        let ingredients: HashSet<String> = spl
            .next()
            .unwrap()
            .split(" ")
            .map(|x| x.to_owned())
            .collect();
        let allergens: HashSet<String> = spl
            .next()
            .unwrap()
            .split(", ")
            .map(|x| x.to_owned())
            .collect();

        for i in ingredients.iter() {
            total_ingredients.insert(i.to_owned());
        }
        for a in allergens.iter() {
            total_allergens.insert(a.to_owned());
        }
        items.push(Item {
            ingredients,
            allergens,
        });
    }

    let mut marked_ingredients = HashSet::new();
    let mut possible_matches: HashMap<String, Vec<String>> = HashMap::new();

    for current_allergen in total_allergens {
        let mut map = HashMap::new();
        let mut allergen_counter = 0;
        for it in items.iter() {
            if !it.allergens.contains(&current_allergen) {
                continue;
            }
            allergen_counter += 1;
            for i in it.ingredients.iter() {
                match map.get(&i) {
                    Some(count) => {
                        map.insert(i, count + 1);
                    }
                    None => {
                        map.insert(i, 1);
                    }
                }
            }
        }
        let possible_ingredients: Vec<_> = map
            .iter()
            .filter(|(&_k, &v)| v == allergen_counter)
            .collect();
        for x in possible_ingredients.iter() {
            marked_ingredients.insert(x.0.clone());
            match possible_matches.get_mut(x.0.clone()) {
                Some(count) => {
                    count.push(current_allergen.clone());
                }
                None => {
                    let v = vec![current_allergen.clone()];
                    possible_matches.insert(x.0.clone().clone(), v);
                }
            }
        }
    }
    let mut c = 0;
    for it in items.iter() {
        for ing in it.ingredients.iter() {
            if !marked_ingredients.contains(ing) {
                c += 1;
            }
        }
    }
    println!("Part 1: {}", c);

    // Simple elimination
    println!("Part 2: {:#?}", possible_matches);
}
