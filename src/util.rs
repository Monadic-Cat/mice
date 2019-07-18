//! Nice to have utilities that aren't core to dice
//! manipulation itself, just handy for some reason.
use crate::{eval_iter, sum_result_iter, Expr, ExprTuple, RollError};

/// Returns a `String` representing the sum
/// of all provided terms.
/// Tuple used is the same form as returned
/// from `dice_vec`, and taken by `roll_vec`.
///
/// The format used is mostly driven by  crate
/// internals, and is subject to change.
pub fn roll_vec_nice(input: &Vec<ExprTuple>) -> Result<String, RollError> {
    let mut results = Vec::new();
    // After this, the function will have returned Err
    // if a member of results is an Err,
    // so it is safe to unwrap.
    let total = sum_result_iter(eval_iter(input.iter().map(|x| Expr::from(*x))).map(|x| {
        results.push(x);
        x
    }))?;
    // Keep unwrap local so I can see *why* it's safe.
    // It will be easier to remove later if I change
    // the above. Additionally, make results immutable.
    let results: Vec<i64> = results.into_iter().map(|x| x.unwrap()).collect();
    let mut nice_string = total.to_string();
    if results.len() > 1 {
        nice_string.push_str(" = (");
        let mut iter = results.iter();
        // I just asked if results was longer than one,
        // so clearly it has least two elements.
        // It is safe to unwrap the first.
        nice_string.push_str(&iter.next().unwrap().to_string());
        for x in iter {
            nice_string.push_str(&format!(", {}", x));
        }
        nice_string.push_str(")");
    }
    Ok(nice_string)
}
