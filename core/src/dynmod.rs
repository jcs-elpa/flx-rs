/**
 * $File: dynmod.rs $
 * $Date: 2021-10-17 23:17:32 $
 * $Revision: $
 * $Creator: Jen-Chieh Shen $
 * $Notice: See LICENSE.txt for modification and distribution information
 *                   Copyright © 2021 by Shen, Jen-Chieh $
 */
use emacs::{defun, Env, Result, Value, IntoLisp, Vector};

fn flx_rs_score(source: &str, pattern: &str) -> Option<Vec<i32>> {
    let result: flx_rs::Option<flx_rs::Score> = flx_rs::score(source, pattern);
    if result == None {
        return None;
    }
    let vec: Vec<i32> = result.indices.clone();
    vec.insert(0, result.score);
    return Some(vec)
}

// Output Functions

/// Return the QUERY fuzzy score about STR, using flx fuzzy algorithm.
///
/// Sign: (-> Str Str (Option Long))
///
/// Return nil if no match happened.
///
/// (fn STR QUERY)
#[defun]
fn score(env: &Env, str: String, query: String) -> Result<Vector> {
    let _vec: Option<Vec<i32>> = flx_rs_score(&str, &query);
    let vec = env.make_vector(_vec.len(), ())?;
    let index = 0;
    for data in _vec {
        vec.set(index, data);
        index += 1;
    }
    Ok(vec)
}
