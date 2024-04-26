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

pub struct Result(pub Vec<ResultEntry>);

#[derive(Clone, Default)]
pub struct ResultEntry {
    pub cost: usize,
    pub num_hints: usize,
    pub num_qm31_mul: usize,
    pub optimal_reduction_level: usize,
}

pub fn dp(is_nonstandard: bool, goal_func: fn(usize, usize) -> usize) -> Result {
    let mut result = Vec::<ResultEntry>::with_capacity(30);

    result.push(ResultEntry {
        cost: 0,
        num_hints: 0,
        num_qm31_mul: 0,
        optimal_reduction_level: 0,
    });

    for i in 1..=30 {
        let mut current_choice = None;
        let mut current_cost = None;

        for step in 1..=core::cmp::min(i, 5) {
            let num_hints = get_num_hints(i, step, is_nonstandard);
            let num_qm31_mul = get_num_qm31_mul(step);

            let cost = goal_func(num_hints, num_qm31_mul) + result[i - step].cost;

            if current_cost.is_none() || current_cost.unwrap() > cost {
                current_choice = Some((
                    num_hints + result[i - step].num_hints,
                    num_qm31_mul + result[i - step].num_qm31_mul,
                    step,
                ));
                current_cost = Some(cost);
            }
        }

        result.push(ResultEntry {
            cost: current_cost.unwrap(),
            num_hints: current_choice.unwrap().0,
            num_qm31_mul: current_choice.unwrap().1,
            optimal_reduction_level: current_choice.unwrap().2,
        });
    }

    Result(result)
}

fn main() {
    let res = dp(true, |a, _b| a);
    // start with a goal function that optimizes only for stack usage

    for i in (0..res.0.len()).rev() {
        let mut optimal_route = format!("{}", i);
        let mut cur = i;
        while cur != 0 {
            cur = cur - res.0[cur].optimal_reduction_level;
            optimal_route = optimal_route + format!(" -> {}", cur).as_str();
        }

        println!(
            "Layer {}: cost = {}, num_hints = {}, num_qm31_mul = {}, optimal_reduction_level = {}",
            i, res.0[i].cost, res.0[i].num_hints, res.0[i].num_qm31_mul, optimal_route
        )
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
