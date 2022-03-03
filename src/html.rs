use ramhorns::Template;
use crate::{BufferedOutput, GitStatsViewModel, Reporter};

#[derive(Clone)]
pub struct HtmlReporter {
    template: String,
    output: BufferedOutput
}

impl Reporter for HtmlReporter {
    fn write(&mut self, stats: GitStatsViewModel) {
        let data_template="
const summaries = [
{{#summary}}
[\"{{name}}\",\"{{value}}\"],
{{/summary}}
];
const total_commits_by_day_model = [
    {{#total_commits_by_day}}
        [\"{{key}}\",{{value}}],
    {{/total_commits_by_day}}
];
const total_lines_by_day_model = [
    {{#total_lines_by_day}}
    [\"{{key}}\",{{lines_added}},{{lines_deleted}}],
    {{/total_lines_by_day}}
        ];
const total_files_by_day_model = [
    {{#total_files_by_day}}
    [\"{{key}}\",{{files_added}},{{files_deleted}},{{files_modified}},{{files_renamed}}],
    {{/total_files_by_day}}
        ];";
        let template_content = self.template.replace("const wait=!0;",data_template);
        let tpl = Template::new(template_content).unwrap();

        let rendered = tpl.render(&stats);

        self.output.write(rendered);
    }

    fn to_string(&self) -> String {
        return self.output.clone().to_string();
    }
}

impl HtmlReporter {
    pub fn new(output: BufferedOutput) -> Self {
        let report_template = include_str!("../index.html") ;
        HtmlReporter {
            template: report_template.into(),
            output
        }
    }
}
