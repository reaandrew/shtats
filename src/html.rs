use ramhorns::Template;
use crate::{BufferedOutput, GitStatsViewModel, Reporter};

pub struct HtmlReporter {
    template: String,
}

impl Reporter for HtmlReporter {
    fn write(&self, output: &mut BufferedOutput, stats: GitStatsViewModel) {
        let tpl = Template::new(&self.template).unwrap();

        let rendered = tpl.render(&stats);

        output.write(rendered);
    }
}

impl HtmlReporter {
    pub fn new() -> Self {
        let report_template = include_str!("includes/report.html");
        HtmlReporter {
            template: report_template.into()
        }
    }
}
