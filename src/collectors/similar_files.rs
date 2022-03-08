use serde_json::Error;
use crate::duplicates::DuplicateDetector;
use crate::{GitCommit, GitStat};
use crate::stats::JsonValue;
use crate::viewmodel::GitStatsJsonViewModelItem;

pub struct SimilarFilesChangingCollector {
    dup_detector: DuplicateDetector,
}

impl SimilarFilesChangingCollector {
    pub fn _default() -> Self {
        Self {
            dup_detector: DuplicateDetector::new(10)
        }
    }
}

impl JsonValue for SimilarFilesChangingCollector {
    fn get_json_viewmodel(&self) -> Result<GitStatsJsonViewModelItem, Error> {
        // TODO: implement me
        return Ok(Default::default());
    }
}

impl GitStat for SimilarFilesChangingCollector {
    fn process(&mut self, commit: &GitCommit) {
        let files = commit.file_operations
            .iter()
            .map(|x| x.file.as_str())
            .collect::<Vec<&str>>();
        self.dup_detector.add(files);
    }
}