#![feature(generic_associated_types)]

pub mod viewmodel;
pub mod parsers;
pub mod output;
pub mod models;
mod stats;
mod collectors;
pub mod html;
pub mod process;
pub mod duplicates;
pub mod errors;
pub mod result;


use viewmodel::{GitStatsJsonViewModel};
use crate::collectors::create_stat_collectors;
use crate::models::GitCommit;
use crate::output::{BufferedOutput};
use crate::stats::{GitStat, LineStats};


pub trait Reporter {
    fn write(&mut self, stats: GitStatsJsonViewModel);
    fn to_string(&self) -> String;
}




