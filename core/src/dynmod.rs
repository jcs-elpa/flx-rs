/**
 * $File: dynmod.rs $
 * $Date: 2021-10-17 23:17:32 $
 * $Revision: $
 * $Creator: Jen-Chieh Shen $
 * $Notice: See LICENSE.txt for modification and distribution information
 *                   Copyright © 2021 by Shen, Jen-Chieh $
 */
use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex};

use emacs::{defun, Env, Result, Value, IntoLisp, Vector};

use once_cell::sync::Lazy;

pub struct StrInfo {
    // Generated through get_hash_for_string
    hash_for_string: HashMap<Option<u32>, VecDeque<Option<u32>>>,

    // Something that get_heatmap_str would return.
    heatmap: Vec<i32>,
}

impl StrInfo {
    fn new() -> StrInfo {
        StrInfo {
            hash_for_string: HashMap::new(),
            heatmap: Vec::new(),
        }
    }
}

static CACHE: Lazy<Mutex<HashMap<String, HashMap<String, StrInfo>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

fn flx_rs_score(source: &str, pattern: &str, cache: &mut Option<HashMap<String, StrInfo>>) -> Option<Vec<i32>> {
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
fn score(env: &Env, str: String, query: String, cache_id: Option<String>) -> Result<Option<Vector>> {
    let cache: Option<HashMap<String, StrInfo>> = None;

    if !cache_id.is_none() {
        cache = HashMap::new();
        CAHCE.try_lock().expect("Failed to access cache registry").insert(cache_id.unwrap(), cache);
    }

    let _vec: Option<Vec<i32>> = flx_rs_score(&str, &query, &mut cache);
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
