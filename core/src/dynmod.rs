/**
 * $File: dynmod.rs $
 * $Date: 2021-10-17 23:17:32 $
 * $Revision: $
 * $Creator: Jen-Chieh Shen $
 * $Notice: See LICENSE.txt for modification and distribution information
 *                   Copyright © 2021 by Shen, Jen-Chieh $
 */
use std::collections::{HashMap, VecDeque};
use emacs::{defun, Env, Result, Value, IntoLisp, Vector};

fn flx_rs_score(source: &str, pattern: &str) -> Option<Vec<i32>> {
    let result: Option<flx_rs::Score> = flx_rs::score(source, pattern);
    if result.is_none() {
        return None;
    }
    let _result: flx_rs::Score = result.unwrap();
    let mut vec: Vec<i32> = _result.indices.clone();
    vec.insert(0, _result.score);
    return Some(vec)
}
