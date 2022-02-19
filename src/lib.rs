pub mod viewmodel;
pub mod parsers;
pub mod output;
pub mod models;
mod stats;
mod collectors;
pub mod html;
pub mod process;
pub mod duplicates;

use viewmodel::{GitStatsViewModel};
use crate::collectors::create_stat_collectors;
use crate::models::GitCommit;
use crate::output::{BufferedOutput};
use crate::stats::{GitStat, GitStats, LineStats};


pub trait Reporter {
    fn write(&self, output: &mut BufferedOutput, stats: GitStatsViewModel);
}




