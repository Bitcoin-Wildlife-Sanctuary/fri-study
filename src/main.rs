use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const NUM_OF_HINTS_FOR_EXTRACTION: usize = 2;
const NUM_OF_QUERIES: usize = 5;

pub fn get_num_hints(cur_height: usize, reduction_layer: usize, is_nonstandard: bool) -> usize {
    let effective_layers = if cur_height >= reduction_layer {
        cur_height - reduction_layer
    } else {
        0
    };

    let mut cost = 0;

    if is_nonstandard {
        let mut cur = effective_layers;

        while cur > 0 {
            if cur == 5 {
                cur -= 5;
                cost += 3;
            } else {
                if cur >= 4 {
                    cur -= 4;
                } else {
                    cur = 0;
                }
                cost += 2;
            }
        }
    } else {
        cost = effective_layers;
    }

    return NUM_OF_HINTS_FOR_EXTRACTION + NUM_OF_QUERIES * (((1 << reduction_layer) - 1) + cost);
}

pub fn get_num_qm31_mul(reduction_layer: usize) -> usize {
    NUM_OF_QUERIES * ((1 << reduction_layer) - 1) + (reduction_layer - 1)
}

pub struct Result(pub HashMap<ResultIndex, Option<ResultEntry>>);

#[derive(Hash, Copy, Debug, Clone, PartialEq, Eq, Default)]
pub struct ResultIndex {
    pub remaining_levels: usize,
    pub remaining_hint_limit: usize,
    pub remaining_qm31_mul_limit: usize,
}

#[derive(Clone, Debug, Default)]
pub struct ResultEntry {
    pub cost: usize,
    pub num_hints: usize,
    pub num_qm31_mul: usize,
    pub optimal_strategy: Option<ResultIndex>,
}

pub trait DPConfig {
    const IS_NONSTANDARD: bool;

    fn goal_func(a: usize, b: usize) -> usize;
}

pub fn dp<C: DPConfig>(map: Rc<RefCell<Result>>, idx: ResultIndex) -> Option<ResultEntry> {
    if idx.remaining_levels == 0 {
        let mut map = map.as_ref().borrow_mut();

        let entry = Some(ResultEntry {
            cost: 0,
            num_hints: 0,
            num_qm31_mul: 0,
            optimal_strategy: None,
        });

        map.0.insert(idx, entry.clone());
        return entry;
    }

    {
        let map = map.as_ref().borrow();
        if map.0.contains_key(&idx) {
            return map.0.get(&idx).unwrap().clone();
        }
    }

    let mut current_entry = None;
    let mut current_cost = None;

    for step in 1..=core::cmp::min(idx.remaining_levels, 4) {
        let num_hints = get_num_hints(idx.remaining_levels, step, C::IS_NONSTANDARD);
        let num_qm31_mul = get_num_qm31_mul(step);

        if idx.remaining_hint_limit < num_hints || idx.remaining_qm31_mul_limit < num_qm31_mul {
            continue;
        }

        let next_index = ResultIndex {
            remaining_levels: idx.remaining_levels - step,
            remaining_hint_limit: idx.remaining_hint_limit - num_hints,
            remaining_qm31_mul_limit: idx.remaining_qm31_mul_limit - num_qm31_mul,
        };

        let next_result = dp::<C>(map.clone(), next_index);

        if let Some(next_result) = next_result {
            let cost = C::goal_func(num_hints, num_qm31_mul) + next_result.cost;

            if current_cost.is_none() || current_cost.unwrap() > cost {
                current_entry = Some(ResultEntry {
                    cost,
                    num_hints: next_result.num_hints + num_hints,
                    num_qm31_mul: next_result.num_qm31_mul + num_qm31_mul,
                    optimal_strategy: Some(next_index),
                });
                current_cost = Some(cost);
            }
        }
    }

    let mut map = map.as_ref().borrow_mut();
    map.0.insert(idx, current_entry.clone());
    current_entry
}

fn main() {
    pub struct Config;
    impl DPConfig for Config {
        const IS_NONSTANDARD: bool = true;

        fn goal_func(_a: usize, b: usize) -> usize {
            b
        }
    }

    let map = Rc::new(RefCell::new(Result(HashMap::new())));

    let idx = ResultIndex {
        remaining_levels: 28,
        remaining_hint_limit: 700,
        remaining_qm31_mul_limit: 250,
    };

    let res = dp::<Config>(map.clone(), idx);

    if let Some(res) = res {
        println!(
            "Layer {}: cost = {}, num_hints = {}, num_qm31_mul = {}",
            idx.remaining_levels, res.cost, res.num_hints, res.num_qm31_mul
        );

        let mut route = format!("{}", idx.remaining_levels);

        let mut next_index = res.optimal_strategy;
        let map = map.as_ref().borrow();

        loop {
            let next_result = map
                .0
                .get(&next_index.unwrap())
                .unwrap()
                .as_ref()
                .unwrap()
                .clone();
            if next_result.optimal_strategy.is_none() {
                break;
            }

            route = route + format!(" => {}", next_index.unwrap().remaining_levels,).as_str();
            next_index = next_result.optimal_strategy;
        }

        println!("Route: {}", route);
    } else {
        println!("Cannot find a strategy that fits into the limits.")
    }
}

#[cfg(test)]
mod consistency_test {
    use crate::{get_num_hints, get_num_qm31_mul};

    #[test]
    fn test_get_num_hints() {
        assert_eq!(get_num_hints(20, 1, false), 22);
        assert_eq!(get_num_hints(20, 1, true), 13);

        assert_eq!(get_num_hints(20, 3, false), 26);
        assert_eq!(get_num_hints(20, 3, true), 18);

        assert_eq!(get_num_hints(20, 4, false), 33);
        assert_eq!(get_num_hints(20, 4, true), 25);
    }

    #[test]
    fn test_get_num_qm31_mul() {
        assert_eq!(get_num_qm31_mul(1), 1);
        assert_eq!(get_num_qm31_mul(3), 9);
        assert_eq!(get_num_qm31_mul(4), 18);
    }
}
