use std::collections::HashMap;
use chrono::Datelike;
use serde_json::{Error, json};
use crate::models::{GitCommit};
use crate::stats::{GitStat, JsonValue, LineStats, LineStatsAverage};
use crate::viewmodel::{GitStatsJsonViewModelItem};

pub struct LinesByAverageCollector{
    first_commit: GitCommit,
    last_commit: GitCommit,
    by_hour: HashMap<String, LineStats>,
    by_day: HashMap<String, LineStats>,
    by_week: HashMap<String, LineStats>,
    by_month: HashMap<String, LineStats>,
    by_commit_total: LineStats,
    total_commits: i32
}

impl LinesByAverageCollector {
    pub fn default() -> Self {
        Self {
            first_commit: Default::default(),
            last_commit: Default::default(),
            by_hour: Default::default(),
            by_day: Default::default(),
            by_week: Default::default(),
            by_month: Default::default(),
            by_commit_total: Default::default(),
            total_commits: 0
        }
    }

    fn calculate_by(&self, hashmap: &HashMap<String, LineStats>, divisor: f64) -> LineStatsAverage{
        let mut total_added:f64 = 0.;
        let mut total_deleted:f64 = 0.;
        for line_stat in hashmap.values(){
            total_added += line_stat.added as f64;
            total_deleted += line_stat.deleted as f64;
        }

        let avg_added = total_added / divisor;
        let avg_deleted = total_deleted / divisor;

        return LineStatsAverage{
            avg_added: (f64::trunc(avg_added  * 100.0) / 100.0).round(),
            avg_deleted: (f64::trunc(avg_deleted  * 100.0) / 100.0).round()
        }
    }

    fn calculate_by_day(&self) -> LineStatsAverage{
        let total_duration = self.last_commit.date - self.first_commit.date;
       return self.calculate_by(&self.by_day, total_duration.num_days() as f64);
    }

    fn calculate_by_hour(&self) -> LineStatsAverage{
        let total_duration = self.last_commit.date - self.first_commit.date;
        return self.calculate_by(&self.by_hour, total_duration.num_hours() as f64);
    }

    fn calculate_by_week(&self) -> LineStatsAverage{
        let total_duration = self.last_commit.date - self.first_commit.date;
        return self.calculate_by(&self.by_week, total_duration.num_weeks() as f64);
    }

    fn calculate_by_month(&self) -> LineStatsAverage{
        let d1 =  self.last_commit.date;
        let d2 =  self.first_commit.date;
        let months_difference = ( d1.year() - d2.year()) * 12 + d1.month() as i32 - d2.month() as i32;
        return self.calculate_by(&self.by_month, months_difference as f64);
    }

    fn calculate_by_commit(&self) -> LineStatsAverage{
        let added =  (self.by_commit_total.added / self.total_commits as i64) as f64;
        let deleted = (self.by_commit_total.deleted / self.total_commits as i64) as f64;
        return LineStatsAverage{
            avg_added: (f64::trunc(added  * 100.0) / 100.0).round(),
            avg_deleted: (f64::trunc(deleted  * 100.0) / 100.0).round()
        }
    }
}

impl JsonValue for LinesByAverageCollector{
    fn get_json_viewmodel(&self) -> Result<GitStatsJsonViewModelItem, Error> {
        let by_day = self.calculate_by_day();
        let by_hour = self.calculate_by_hour();
        let by_week = self.calculate_by_week();
        let by_month = self.calculate_by_month();
        let by_commit = self.calculate_by_commit();

        let obj = json!({
            "hour": json!({
                "added": by_hour.avg_added,
                "deleted": by_hour.avg_deleted,
                "churn": by_hour.avg_added + by_hour.avg_deleted
            }),
            "day": json!({
                "added": by_day.avg_added,
                "deleted": by_day.avg_deleted,
                "churn": by_day.avg_added + by_day.avg_deleted
            }),
            "week": json!({
                "added": by_week.avg_added,
                "deleted": by_week.avg_deleted,
                "churn": by_week.avg_added + by_week.avg_deleted
            }),
            "month": json!({
                "added": by_month.avg_added,
                "deleted": by_month.avg_deleted,
                "churn": by_month.avg_added + by_month.avg_deleted
            }),
            "commit": json!({
                "added": by_commit.avg_added,
                "deleted": by_commit.avg_deleted,
                "churn": by_commit.avg_added + by_commit.avg_deleted
            })
        });
        return Ok(GitStatsJsonViewModelItem {
            summary: vec![],
            key: String::from("avg_line_stats"),
            data: obj,
        });
    }
}

impl GitStat for LinesByAverageCollector {
    fn process(&mut self, commit: &GitCommit) {
        if self.first_commit == GitCommit::default(){
            self.first_commit = commit.clone();
        }
        self.last_commit = commit.clone();
        let hour_stat = self.by_hour.entry(commit.hour_key())
            .or_insert(LineStats {
                added: 0,
                deleted: 0,
            });
        hour_stat.added += commit.total_lines_added() as i64;
        hour_stat.deleted += commit.total_lines_deleted() as i64;
        
        let day_stat = self.by_day.entry(commit.day_key())
            .or_insert(LineStats {
                added: 0,
                deleted: 0,
            });
        day_stat.added += commit.total_lines_added() as i64;
        day_stat.deleted += commit.total_lines_deleted() as i64;

        let week_stat = self.by_week.entry(commit.week_key())
            .or_insert(LineStats {
                added: 0,
                deleted: 0,
            });
        week_stat.added += commit.total_lines_added() as i64;
        week_stat.deleted += commit.total_lines_deleted() as i64;

        let month_stat = self.by_month.entry(commit.month_key())
            .or_insert(LineStats {
                added: 0,
                deleted: 0,
            });
        month_stat.added += commit.total_lines_added() as i64;
        month_stat.deleted += commit.total_lines_deleted() as i64;

        self.by_commit_total.added += commit.total_lines_added() as i64;
        self.by_commit_total.deleted += commit.total_lines_deleted() as i64;
        self.total_commits += 1
    }
}

#[cfg(test)]
mod tests {
    use crate::collectors::lines_by_average::LinesByAverageCollector;
    use crate::models::{GitCommit, GitCommitBuilder};
    use crate::stats::{GitStat, JsonValue};

    #[test]
    fn test_process() {
        let mut subject = LinesByAverageCollector::default();
        let commit: GitCommit = GitCommitBuilder::new()
            .with_lines(1, 2, "a.txt")
            .build();
        subject.process(&commit);

        assert_eq!(subject.by_day.len(), 1)
    }

    #[test]
    fn test_json_viewmodel() {
        let mut subject = LinesByAverageCollector::default();
        subject.process(&GitCommitBuilder::new()
            .for_date_time("2022-01-01 12:00:00")
            .with_lines(2, 12, "1.txt")
            .build());
        subject.process(&GitCommitBuilder::new()
            .for_date_time("2022-01-16 18:00:00")
            .with_lines(2, 4, "1.txt")
            .build());

        let result = subject.get_json_viewmodel().unwrap();
        //TODO: Create a struct for the result, deserialize the result and assert on the struct
        //      This applies to all types of assertions like this.
        assert_eq!(result.data.to_string(), "{\"commit\":{\"added\":2.0,\"churn\":10.0,\"deleted\":8.0},\"day\":{\"added\":0.0,\"churn\":1.0,\"deleted\":1.0},\"hour\":{\"added\":0.0,\"churn\":0.0,\"deleted\":0.0},\"month\":{\"added\":null,\"churn\":null,\"deleted\":null},\"week\":{\"added\":2.0,\"churn\":10.0,\"deleted\":8.0}}");
    }
}