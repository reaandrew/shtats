use ramhorns::Template;
use serde_json::json;
use crate::{BufferedOutput, GitStatsJsonViewModel, Reporter};
use ramhorns::Content;

#[derive(Clone)]
pub struct HtmlReporter {
    template: String,
    output: BufferedOutput
}

#[derive(Content)]
struct Context{
    pub data: String
}

impl Reporter for HtmlReporter {
    fn write(&mut self, stats: GitStatsJsonViewModel) {
        let data_template="const viewmodel = {{{data}}}";

        let json_value = serde_json::to_string(&stats).unwrap();
        let template_content = self.template.replace("const viewmodel = null;",data_template)
            .replace("const viewmodel=null;",data_template);
        let tpl = Template::new(template_content).unwrap();

        let rendered = tpl.render(&Context{data: json_value});

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
