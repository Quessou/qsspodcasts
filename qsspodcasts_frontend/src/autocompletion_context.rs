use tui::style::*;
use tui::text::{Line, Span};

#[derive(Default)]
pub struct AutocompletionContext {
    pub current_input: String,
    pub autocompletion_choices: Vec<String>,
    pub current_choice: Option<usize>,
    pub autocompletion_states: Vec<(Vec<String>, usize)>,
}

impl AutocompletionContext {
    pub fn reset(&mut self) {
        self.autocompletion_choices.clear();
        self.current_choice = None;
        self.autocompletion_states.clear();
    }

    pub fn clear(&mut self) {
        self.reset();
        "".clone_into(&mut self.current_input);
    }

    pub fn is_autocompletion_buffer_empty(&self) -> bool {
        self.autocompletion_choices.is_empty()
            || self.current_input.len()
                == self.autocompletion_choices[self.current_choice.unwrap()].len()
    }

    pub fn is_ctxt_initialized(&self) -> bool {
        self.current_choice.is_some() && !self.autocompletion_choices.is_empty()
    }

    pub fn set_autocompletion_choices(&mut self, choices: Vec<String>) {
        self.autocompletion_choices = choices;
        if !self.autocompletion_choices.is_empty() {
            self.current_choice = Some(0);
        } else {
            self.current_choice = None;
        }
    }

    pub fn get_displayed_input(&self) -> Line {
        let empty_string = String::default();
        let current_choice = self.current_choice.unwrap_or(0);
        let autocompletion_text = self
            .autocompletion_choices
            .get(current_choice)
            .unwrap_or(&empty_string)
            .split(&self.current_input)
            .nth(1)
            .unwrap_or("");
        Line::from(vec![
            Span::from(self.current_input.clone()),
            Span::styled(
                autocompletion_text.to_owned(),
                Style::default().add_modifier(Modifier::DIM),
            ),
        ])
    }

    pub fn go_to_next_choice(&mut self) {
        assert!(!self.autocompletion_choices.is_empty());
        self.current_choice =
            Some((self.current_choice.unwrap() + 1) % self.autocompletion_choices.len())
    }

    pub fn is_autocompletion_request_required(&self) -> bool {
        self.current_choice.is_none()
    }

    pub fn is_autocompletion_request_possible(&self) -> bool {
        !self.current_input.is_empty()
    }

    pub fn push_current_state(&mut self) {
        self.autocompletion_states.push((
            self.autocompletion_choices.clone(),
            self.current_choice.unwrap(),
        ));
    }

    pub fn pop_state(&mut self) {
        let (choices, choice_index) = self.autocompletion_states.pop().unwrap_or((vec![], 0));
        self.autocompletion_choices = choices;
        self.current_choice = if !self.autocompletion_choices.is_empty() {
            Some(choice_index)
        } else {
            None
        };
    }

    pub fn narrow_choices(&mut self) {
        self.autocompletion_choices
            .retain(|c: &String| c.starts_with(&self.current_input));
        if !self.autocompletion_choices.is_empty() {
            self.current_choice = Some(0);
        } else {
            self.current_choice = None;
        }
    }

    pub fn confirm(&mut self) {
        if !self.autocompletion_choices.is_empty()
            && self.current_choice.is_some()
            && self.current_input.len()
                < self.autocompletion_choices[self.current_choice.unwrap()].len()
        {
            self.current_input
                .clone_from(&self.autocompletion_choices[self.current_choice.unwrap()]);
            self.reset();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case("l".to_owned(), vec!["list_podcasts".to_owned()], 0 => ("l".to_owned(), "ist_podcasts".to_owned()))]
    fn test_displayed_input(
        current_input: String,
        autocompletion_choices: Vec<String>,
        current_choice: usize,
    ) -> (String, String) {
        let ctxt = AutocompletionContext {
            current_input,
            autocompletion_choices,
            current_choice: Some(current_choice),
            autocompletion_states: vec![],
        };
        let input = ctxt.get_displayed_input();
        (
            input.spans[0].content.to_string(),
            input.spans[1].content.to_string(),
        )
    }

    #[test_case("list_e".to_owned(), vec!["list_podcasts".to_owned(), "list_episodes".to_owned()] => vec!["list_episodes".to_owned()])]
    fn test_narrow_choices(current_input: String, choices: Vec<String>) -> Vec<String> {
        let mut ctxt = AutocompletionContext {
            current_input,
            autocompletion_choices: choices,
            current_choice: Some(0),
            autocompletion_states: vec![],
        };
        ctxt.narrow_choices();
        ctxt.autocompletion_choices
    }
}
