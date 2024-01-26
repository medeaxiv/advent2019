use ahash::AHashMap as HashMap;

use super::Result;

pub const INPUT_FILE: &str = "inputs/day14/input.txt";

mod parser;

pub fn part1(input: &str) -> Result<impl std::fmt::Display> {
    solve_part1(input)
}

fn solve_part1(input: &str) -> Result<u64> {
    let reactions = parser::parse(input)?;
    let fuel = reactions.index["FUEL"];
    let ore = reactions.index["ORE"];

    let counts = count_required_chemicals(fuel, 1, &reactions);

    Ok(counts[ore].0)
}

pub fn part2(input: &str) -> Result<impl std::fmt::Display> {
    solve_part2(input)
}

fn solve_part2(input: &str) -> Result<u64> {
    let reactions = parser::parse(input)?;
    let maximum_ore = 1_000_000_000_000u64;
    let fuel = reactions.index["FUEL"];
    let ore = reactions.index["ORE"];

    let mut counts = count_required_chemicals(fuel, 1, &reactions);
    let fuel_cost = counts[ore].0; // upper bound on the cost of fuel
    let mut fuel_amount = maximum_ore / fuel_cost; // lower bound on the amount of fuel
    let mut consumed_ore;

    loop {
        // Reuse the allocated memory from `counts`
        count_required_chemicals_into(fuel, fuel_amount, &reactions, &mut counts);
        consumed_ore = counts[ore].0;

        if consumed_ore > maximum_ore {
            break;
        }

        let remaining_ore = maximum_ore - consumed_ore;
        let remaining_fuel = remaining_ore / fuel_cost;

        // If there's enough raw ore left for at least a full unit of fuel...
        if remaining_fuel > 0 {
            // count again with that extra amount of fuel
            fuel_amount += remaining_fuel;
        } else {
            // else inch towards the limit relying on leftover chemicals
            fuel_amount += 1;
        }
    }

    // I'm always testing one past the maximum amount, remove the last one
    Ok(fuel_amount - 1)
}

fn count_required_chemicals(
    chemical: usize,
    amount: u64,
    reactions: &Reactions,
) -> Box<[(u64, u64)]> {
    let mut dest = vec![(0, 0); reactions.chemicals.len()].into_boxed_slice();
    count_required_chemicals_into(chemical, amount, reactions, &mut dest);
    dest
}

fn count_required_chemicals_into(
    chemical: usize,
    amount: u64,
    reactions: &Reactions,
    dest: &mut [(u64, u64)],
) {
    fn count(
        chemical: usize,
        amount_required: u64,
        reactions: &Reactions,
        counts: &mut [(u64, u64)],
    ) {
        if let Some(reaction) = &reactions.reactions[chemical] {
            if amount_required > counts[chemical].1 {
                let new_required = amount_required - counts[chemical].1;
                let reaction_count = new_required / reaction.amount_produced
                    + if new_required % reaction.amount_produced != 0 {
                        1
                    } else {
                        0
                    };
                let amount_produced = reaction_count * reaction.amount_produced;
                counts[chemical].0 += amount_produced;
                counts[chemical].1 = amount_produced - new_required;

                for &(ingredient, ingredient_count) in reaction.ingredients.iter() {
                    count(
                        ingredient,
                        ingredient_count * reaction_count,
                        reactions,
                        counts,
                    );
                }
            } else {
                counts[chemical].1 -= amount_required;
            }
        } else {
            counts[chemical].0 += amount_required;
        }
    }

    dest.fill((0, 0));
    count(chemical, amount, reactions, dest);
}

#[derive(Clone)]
pub struct Reaction<T> {
    amount_produced: u64,
    ingredients: Vec<(T, u64)>,
}

#[derive(Default, Clone)]
pub struct Reactions<'a> {
    chemicals: Vec<&'a str>,
    index: HashMap<&'a str, usize>,
    reactions: Vec<Option<Reaction<usize>>>,
}

impl<'a> Reactions<'a> {
    fn insert_chemical(&mut self, chemical: &'a str) -> usize {
        if let Some(idx) = self.index.get(chemical) {
            *idx
        } else {
            let idx = self.chemicals.len();
            self.chemicals.push(chemical);
            self.index.insert(chemical, idx);
            self.reactions.push(None);
            idx
        }
    }

    pub fn add_chemical(&mut self, chemical: &'a str) {
        self.insert_chemical(chemical);
    }

    pub fn add_reaction(&mut self, chemical: &'a str, reaction: Reaction<&'a str>) {
        let reaction_idx = self.insert_chemical(chemical);
        let reaction = Reaction {
            amount_produced: reaction.amount_produced,
            ingredients: reaction
                .ingredients
                .into_iter()
                .map(|(chemical, quantity)| (self.insert_chemical(chemical), quantity))
                .collect(),
        };

        self.reactions[reaction_idx] = Some(reaction);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn input(which: usize) -> Result<String> {
        let file = format!("inputs/day14/test.{}.txt", which);
        let file = std::fs::read_to_string(file)?;
        Ok(file)
    }

    #[rstest]
    #[case(0, 31)]
    #[case(1, 165)]
    #[case(2, 13312)]
    #[case(3, 180697)]
    #[case(4, 2210736)]
    fn test_part1(#[case] which: usize, #[case] expected: u64) -> Result<()> {
        crate::util::test::setup_tracing();
        let input = input(which)?;
        let result = solve_part1(&input)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[rstest]
    #[case(2, 82892753)]
    #[case(3, 5586022)]
    #[case(4, 460664)]
    fn test_part2(#[case] which: usize, #[case] expected: u64) -> Result<()> {
        crate::util::test::setup_tracing();
        let input = input(which)?;
        let result = solve_part2(&input)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
