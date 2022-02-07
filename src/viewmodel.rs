use crate::GitStats;
use ramhorns::{Content};

#[derive(Content)]
struct SummaryViewModel {
    name: String,
    value: String,
}

#[derive(Content)]
struct KeyValue {
    key: String,
    value: i32,
}


#[derive(Content)]
pub struct GitStatsViewModel {
    summary: Vec<SummaryViewModel>,
    total_commits_by_day: Vec<KeyValue>,
}

impl GitStatsViewModel {
    pub(crate) fn new(stats: &GitStats) -> Self {
        let mut instance = Self {
            summary: vec![],
            total_commits_by_day: vec![]
        };


        instance.summary.push(SummaryViewModel{
            name: "First committer".to_string(),
            value: stats.summary.first_committer.clone()
        });
        instance.summary.push(SummaryViewModel{
            name: "Date of first commit".to_string(),
            value: stats.summary.date_first_commit.clone()
        });
        instance.summary.push(SummaryViewModel{
            name: "Number of commits".to_string(),
            value: stats.summary.commit_count.to_string()
        });

        for (key, value) in stats.total_commits_by_day.clone() {
            instance.total_commits_by_day.push(KeyValue{
                key,
                value
            })
        }
        instance.total_commits_by_day.sort_by(|a, b| a.key.cmp(&b.key));

        return instance;
    }
}