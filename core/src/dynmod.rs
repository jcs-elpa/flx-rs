/**
 * $File: dynmod.rs $
 * $Date: 2021-10-17 23:17:32 $
 * $Revision: $
 * $Creator: Jen-Chieh Shen $
 * $Notice: See LICENSE.txt for modification and distribution information
 *                   Copyright © 2021 by Shen, Jen-Chieh $
 */
use emacs::{defun, Env, Result, Value, IntoLisp};

fn flx_rs_score(source: &str, pattern: &str) -> Option<f64> {
    let result = flx_rs::score(source, pattern);
    if result == None {
        return None;
    }
    return Some(result.unwrap() as f64)
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
fn score(_env: &Env, str: String, query: String) -> Result<Option<f64>> {
    Ok(flx_rs_score(&str, &query))
}
