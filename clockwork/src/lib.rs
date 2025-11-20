#![allow(warnings)]

//! This crate contains the main chess engine logic, including the UCI communication,
//! search algorithm, and evaluation function.

pub mod api;
pub mod engine;
pub mod evaluate;
pub mod search;
