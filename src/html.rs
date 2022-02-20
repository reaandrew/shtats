use ramhorns::Template;
use crate::{BufferedOutput, GitStatsViewModel, Reporter};

pub struct HtmlReporter {
    template: String,
}

impl Reporter for HtmlReporter {
    fn write(&self, output: &mut BufferedOutput, stats: GitStatsViewModel) {
        let data_template="
const total_commits_by_day_model = [
    {{#total_commits_by_day}}
        [\"{{key}}\",{{value}}],
    {{/total_commits_by_day}}
];
const total_lines_by_day_model = [
    {{#total_lines_by_day}}
    [\"{{key}}\",{{lines_added}},{{lines_deleted}}],
    {{/total_lines_by_day}}
        ];";
        let template_content = self.template.replace("const wait=!0;",data_template);
        let tpl = Template::new(template_content).unwrap();

        let rendered = tpl.render(&stats);

        output.write(rendered);
    }
}

impl HtmlReporter {
    pub fn new() -> Self {
        let report_template = include_str!("../index.html") ;
        HtmlReporter {
            template: report_template.into()
        }
    }
}
