use anyhow::Result;
use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap};

type Food = (Vec<String>, Vec<String>);

fn main() -> Result<()> {

    let mut food: Vec<Food> = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        let elements: Vec<_> = line.split(" (contains ").collect();
        assert!(elements.len() == 2);
        let ingredients = elements[0].split(" ").map(|x| x.to_owned()).collect();
        let allergens = elements[1].trim_matches(')').split(", ").map(|x| x.to_owned()).collect();

        food.push((ingredients, allergens));
    }

    let mut possibles = possibles(&food);

    println!("Part 1 = {}", p1(&food, &possibles));

    println!("Part 2 = {}", p2(&mut possibles));

    Ok(())
}

fn p2(possibles: &mut HashMap<String, HashSet<String>>) -> String {
    loop {
        // each loop, make list of all things with only one ingredient
        let mut hs = HashSet::new();
        for (_,v) in possibles.iter() {
            if v.len() == 1 {
                hs = hs.union(v).map(|x| x.to_owned()).collect();
            }
        }

        // remove that ingredient from all sets of len > 1
        for (_, v) in possibles.iter_mut() {
           for o in &hs {
                if v.len() > 1 && v.contains(o) {
                    v.remove(o);
                }
            }
        }

        // repeat untill all are len 1
        let l: usize = possibles.iter().map(|(_, v)| v.len()).max().unwrap();
        if l == 1 {
            break;
        }
    }

    for (k, v) in possibles.iter() {
        println!("{} {:?}", k, v);
    }

    let mut keys:Vec<_> = possibles.iter().map(|(k, _)| k).collect();
    keys.sort_unstable();

    let mut s =  vec![];
    for i in keys {
        s.push(possibles[i].iter().map(|x| x).nth(0).unwrap().to_owned());
    }

    s.join(",")
}

fn p1(food: &Vec<Food>, possibles: &HashMap<String, HashSet<String>>) -> usize {
    let allergens: HashSet<String> = possibles
        .iter()
        .map(|(_, i)| i.clone())
        .flatten()
        .collect();

    food
        .iter()
        .map(|(i, _)| i.clone())
        .flatten()
        .filter(|x| !allergens.contains(x))
        .count()
}


fn possibles(food: &Vec<Food>) -> HashMap<String, HashSet<String>>{
    let mut possibles: HashMap<String, HashSet<String>> = HashMap::new();

    for (ingredients, allergens) in food {

        let is: HashSet<String> = ingredients.iter().map(|x| x.to_owned()).collect();

        for allergen in allergens {
            if !possibles.contains_key(allergen) {
                possibles.insert(
                    allergen.clone(), is.clone()
                );
            } else {
                *possibles.get_mut(allergen).unwrap()
                    = possibles[allergen].intersection(&is).map(|x| x.to_owned()).collect();
            }

        }
    }

    possibles
}

