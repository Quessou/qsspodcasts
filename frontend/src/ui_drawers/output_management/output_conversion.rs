use command_management::output::output_type::OutputType;
use tui::text::Spans;

use crate::style::stylized::VecSpans;

//use crate::style::stylized::Stylized;

// OutputType => Vec<Spans>
// impl From<OutputType> for Vec<Spans>

impl From<OutputType> for VecSpans<'_> {
    fn from(output_type: OutputType) -> Self {
        let dummy = String::from("");
        let complete_output: VecSpans = match &output_type {
            OutputType::RawString(s) => s.as_str().into(),
            OutputType::Podcasts(podcasts) => podcasts.into(),
            _ => dummy.as_str().into(),
        };
        complete_output
    }
}
