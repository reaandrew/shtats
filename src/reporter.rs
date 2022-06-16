use crate::viewmodel::GitStatsJsonViewModel;


pub trait Reporter : ToString {
    fn write(&mut self, stats: &GitStatsJsonViewModel);
}

