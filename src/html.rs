use std::fmt::{Display, Formatter};
use ramhorns::Template;
use ramhorns::Content;
use crate::output::BufferedOutput;
use crate::reporter::Reporter;
use crate::viewmodel::GitStatsJsonViewModel;

pub trait HtmlTemplate{
    fn get<'a>(&self) -> &'a str;
}

pub struct PreactTemplate{

}

impl HtmlTemplate for PreactTemplate{
    fn get<'a>(&self) -> &'a str {
        return include_str!("../index.html");
    }
}

#[derive(Clone)]
pub struct HtmlReporter {
    template: String,
    output: BufferedOutput
}

impl Display for HtmlReporter{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.output.clone().to_string().as_str())
    }
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
}

impl HtmlReporter {
    pub fn new<T : HtmlTemplate>(template : T) -> Self {
        let report_template = template.get();
        HtmlReporter {
            template: report_template.into(),
            output: BufferedOutput::new()
        }
    }
}

#[cfg(test)]
mod tests{
    use serde_json::{json, Value};
    use crate::html::{HtmlReporter, HtmlTemplate};
    use crate::reporter::Reporter;
    use crate::viewmodel::{GitStatsJsonViewModel, GitStatsJsonViewModelItem, SummaryViewModelItem};

    struct TestTemplate{

    }

    impl HtmlTemplate for TestTemplate{
        fn get<'a>(&self) -> &'a str {
            return "const viewmodel=null;";
        }
    }

    #[test]
    fn test_write(){
        let mut reporter = HtmlReporter::new(TestTemplate{});
        let mut viewmodel = GitStatsJsonViewModel::default();

        let mut viewmodel_item = GitStatsJsonViewModelItem::default();
        viewmodel_item.summary.push(SummaryViewModelItem{
            name: "name1".to_string(),
            value: "value1".to_string()
        });
        viewmodel_item.key = String::from("key1");
        viewmodel_item.data = json!({
            "a": 1
        });

        viewmodel.data.insert(viewmodel_item.key, viewmodel_item.data );
        viewmodel.summary.extend(viewmodel_item.summary.iter()
            .map(|x| {
                return serde_json::to_value(x).unwrap();
            }).collect::<Vec<Value>>());

        reporter.write(viewmodel);

        let expected = "const viewmodel = {\"summary\":[{\"name\":\"name1\",\"value\":\"value1\"}],\"data\":{\"key1\":{\"a\":1}}}";
        assert_eq!(expected, reporter.to_string());
    }
}