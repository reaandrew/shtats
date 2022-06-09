use crate::viewmodel::GitStatsJsonViewModel;


pub trait Reporter {
    fn write(&mut self, stats: GitStatsJsonViewModel);
    fn to_string(&self) -> String;
}

