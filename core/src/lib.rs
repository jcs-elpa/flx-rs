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

use emacs::{Env, Result};

pub mod dynmod;

// Module Defintion
emacs::plugin_is_GPL_compatible!();

// Empty method to satisify emacs module
#[emacs::module(mod_in_name = false)]
fn init(_: &Env) -> Result<()> {
    Ok(())
}
