use std::collections::{HashMap, HashSet};
use util::file::GenericParseError;
use util::res::Result;

struct FoodItem {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl std::str::FromStr for FoodItem {
    type Err = GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match &s.splitn(2, " (contains ").collect::<Vec<&str>>()[..] {
            &[part1, part2] => Ok(FoodItem{
                ingredients: part1.split_whitespace()
                    .map(|s| s.to_owned()).collect::<HashSet<String>>(),
                allergens: part2.replace(&[')', ','][..], "").split_whitespace()
                    .map(|s| s.to_owned()).collect::<HashSet<String>>(),
            }),
            _ => Err(GenericParseError::ValueError(
                format!("Invalid line: {}", s).to_owned()
            )),
        }
    }
}

// part1 returns the allergen possibility mappings needed for part 2
fn part1(food_items: &Vec<FoodItem>) -> HashMap<String, HashSet<String>> {
    let mut could_be_allergen: HashMap<String, HashSet<String>> = HashMap::new();
    for food_item in food_items {
        for allergen in food_item.allergens.iter() {
            could_be_allergen.entry(allergen.clone())
                .and_modify(|e| {
                    *e = e.intersection(&food_item.ingredients).cloned().collect::<HashSet<String>>()
                }).or_insert(food_item.ingredients.clone());
        }
    }
    let mut all_ingredients: HashSet<String> = HashSet::new();
    for food_item in food_items {
        food_item.ingredients.iter().for_each(|i| { all_ingredients.insert(i.clone()); });
    }

    let mut could_not_be_allergen: HashSet<String> = all_ingredients.clone();
    for (_, maybe_allergens) in &could_be_allergen {
        for maybe_allergen in maybe_allergens.iter() {
            could_not_be_allergen.remove(maybe_allergen);
        }
    }

    let n_occurrences = food_items.iter().fold(0, |acc, item| {
        acc + item.ingredients.intersection(&could_not_be_allergen).count()
    });

    println!("[Part 1] Total occurrences of non-allergen ingredients: {}", n_occurrences);
    could_be_allergen
}

fn part2(allergen_possibilities: &HashMap<String, HashSet<String>>) {
    let mut allergens: Vec<(String, String)> = vec![];
    let mut possibilities = allergen_possibilities.clone();
    while allergens.len() < allergen_possibilities.len() {
        // Find allergen with only one possibility
        let allergen_mapping = {
            let decidable = possibilities.iter().find(|(_, possibilities)| {
                possibilities.len() == 1
            }).unwrap();

            // Add that mapping to the list of decided allergens
            (decidable.0.clone(), decidable.1.iter().nth(0).unwrap().clone())
        };
        let ingredient_to_remove = allergen_mapping.1.clone();

        // Add that mapping to the list of decided allergens
        allergens.push(allergen_mapping);

        // Remove that ingredient from other allergens' possibilities
        possibilities.iter_mut().for_each(|(_, possibilities)| {
            possibilities.remove(&ingredient_to_remove);
        });
    }

    // Sort allergen mappings alphabetically by their allergen
    allergens.sort_by(|(a, _), (b, _)| a.cmp(b));

    // Get canonical dangerous ingredient list
    let canonical = allergens.iter().map(|(_, ingredient)| ingredient)
        .cloned().collect::<Vec<String>>().join(",");
    println!("[Part 2] Canonical dangerous ingredient list: {}", canonical);
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let food_items: Vec<FoodItem> = util::file::read_lines_to_type::<FoodItem>(file_path)?;

    let allergen_possibilities = part1(&food_items);
    part2(&allergen_possibilities);
    Ok(())
}