//! Module containing various utility functions. 
//! ユーティリティだよ。

mod os ;
mod content_encoding ;

use std::f64 ;

/// The generic HTML page to use as response to errors.
pub static ERROR_HTML: &'static str = include_str!("../../asserts/error.html") ;