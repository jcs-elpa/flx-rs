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

// Output Functions

/// Return the QUERY fuzzy score about STR, using flx fuzzy algorithm.
///
/// Sign: (-> Str Str (Option Long))
///
/// Return nil if no match happened.
///
/// (fn STR QUERY)
#[defun]
fn score(env: &Env, str: String, query: String) -> Result<Option<Vector>> {
    let _vec: Option<Vec<i32>> = flx_rs_score(&str, &query);
    if _vec == None {
        return Ok(None);
    }
    let _inner_vec: Vec<i32> = _vec.unwrap();
    let mut vec = env.make_vector(_inner_vec.len(), ())?;
    let mut index = 0;
    for data in _inner_vec {
        vec.set(index, data);
        index += 1;
    }
    Ok(Some(vec))
}

/// Create cache for flx-rs
///
/// Return new HashMap pointer.
#[defun(user_ptr)]
fn create_cache() -> Result<Option<HashMap<&str, HashMap<Option<u32>, VecDeque<Option<u32>>>>>> {
    Ok(Some(HashMap::new()))
}

/// Clear the CACHE.
///
/// Return HashMap pointer.
///
/// (fn CACHE)
#[defun(user_ptr)]
fn clear_cache(cache: Option<HashMap<&str, HashMap<Option<u32>, VecDeque<Option<u32>>>>>)
               -> Result<Option<HashMap<&str, HashMap<Option<u32>, VecDeque<Option<u32>>>>>> {
    if cache.is_none() {
        Ok(None)
    }
    cache.unwrap().clear();
    Ok(cache)
}
