/**
 * $File: lib.rs $
 * $Date: 2021-10-27 23:19:04 $
 * $Revision: $
 * $Creator: Jen-Chieh Shen $
 * $Notice: See LICENSE.txt for modification and distribution information
 *                   Copyright © 2021 by Shen, Jen-Chieh $
 */
extern crate emacs;
extern crate flx_rs;

use emacs::{ Env, Result };

pub mod dynmod;

// Module Defintion
emacs::plugin_is_GPL_compatible!();

// Empty method to satisify emacs module
#[emacs::module(mod_in_name = false)]
fn init(_: &Env) -> Result<()> {
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

    /// Clear the CACHE.
    ///
    /// Return HashMap pointer.
    ///
    /// (fn CACHE)
    #[defun]
    fn clear_cache(cache: String) -> Result<()> {
        Ok(())
    }

    Ok(())
}
