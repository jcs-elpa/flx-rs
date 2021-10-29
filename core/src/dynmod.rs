/**
 * $File: dynmod.rs $
 * $Date: 2021-10-17 23:17:32 $
 * $Revision: $
 * $Creator: Jen-Chieh Shen $
 * $Notice: See LICENSE.txt for modification and distribution information
 *                   Copyright © 2021 by Shen, Jen-Chieh $
 */
use emacs::{defun, Env, Result, Value, IntoLisp};

fn flx_rs_score(source: &str, pattern: &str) -> Option<Vec<i32>> {
    let result: flx_rs::Score = flx_rs::score(source, pattern);
    if result == None {
        return None;
    }
    return Some(result.score)
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
fn score(_env: &Env, str: String, query: String) -> Result<Vec<i32>> {
    Ok(flx_rs_score(&str, &query))
}
