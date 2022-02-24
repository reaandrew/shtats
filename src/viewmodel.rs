use bytesize::ByteSize;
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
struct LinesValue{
    key: String,
    lines_added: i32,
    lines_deleted: i32
}

#[derive(Content)]
struct FilesValue{
    key: String,
    files_added: i32,
    files_deleted: i32,
    files_modified: i32,
    files_renamed: i32
}



#[derive(Content)]
pub struct GitStatsViewModel {
    summary: Vec<SummaryViewModel>,
    total_commits_by_day: Vec<KeyValue>,
    total_lines_by_day: Vec<LinesValue>,
    total_files_by_day: Vec<FilesValue>
}

impl GitStatsViewModel {
    pub(crate) fn new(stats: &GitStats) -> Self {
        let mut instance = Self {
            summary: vec![],
            total_commits_by_day: vec![],
            total_lines_by_day: vec![],
            total_files_by_day: vec![]
        };


        Self::add_summary(stats, &mut instance);

        for (key, value) in stats.total_commits_by_day.clone() {
            instance.total_commits_by_day.push(KeyValue{
                key,
                value
            })
        }
        instance.total_commits_by_day.sort_by(|a, b| a.key.cmp(&b.key));


        for (key, value) in stats.total_lines_by_day.clone(){
            instance.total_lines_by_day.push(LinesValue{
                key,
                lines_added: value.added,
                lines_deleted: value.deleted
            })
        }
        instance.total_lines_by_day.sort_by(|a, b| a.key.cmp(&b.key));

        for (key, value) in stats.total_files_by_day.clone(){
            instance.total_files_by_day.push(FilesValue{
                key,
                files_added: value.added,
                files_deleted: value.deleted,
                files_modified: value.modified,
                files_renamed: value.renamed
            })
        }
        instance.total_files_by_day.sort_by(|a, b| a.key.cmp(&b.key));
        return instance;
    }

    fn add_summary(stats: &GitStats, instance: &mut GitStatsViewModel) {
        instance.summary.push(SummaryViewModel {
            name: "First committer".to_string(),
            value: stats.summary.first_committer.clone()
        });
        instance.summary.push(SummaryViewModel {
            name: "Date of first commit".to_string(),
            value: stats.summary.date_first_commit.clone()
        });
        instance.summary.push(SummaryViewModel {
            name: "Number of commits".to_string(),
            value: stats.summary.commit_count.to_string()
        });
        instance.summary.push(SummaryViewModel {
            name: "Total lines added".to_string(),
            value: stats.summary.total_lines_added.to_string()
        });
        instance.summary.push(SummaryViewModel {
            name: "Total lines deleted".to_string(),
            value: stats.summary.total_lines_deleted.to_string()
        });

        instance.summary.push(SummaryViewModel{
            name: "Max number of lines in a commit message".to_string(),
            value: stats.message_stats.max_lines.to_string()
        });

        instance.summary.push(SummaryViewModel{
            name: "Max size of a commit message".to_string(),
            value: ByteSize(stats.message_stats.max_size as u64).to_string()
        });

        instance.summary.push(SummaryViewModel{
            name: "Avg number of lines in a commit message".to_string(),
            value: stats.message_stats.avg_lines.to_string()
        });

        instance.summary.push(SummaryViewModel{
            name: "Avg size of a commit message".to_string(),
            value: ByteSize(stats.message_stats.avg_size as u64).to_string()
        });

        instance.summary.push(SummaryViewModel{
            name: "Total number of lines across all commit messages".to_string(),
            value: stats.total_message_lines.to_string()
        });

        instance.summary.push(SummaryViewModel{
            name: "Total size of all commit messages".to_string(),
            value: ByteSize(stats.total_message_size as u64).to_string()
        });
    }
}