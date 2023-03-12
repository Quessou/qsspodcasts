use once_cell::sync::Lazy;

#[derive(PartialEq)]
pub enum CommandParameterType {
    Hash,
    Duration,
    CommandName,
    Url,
}

static AUTO_COMPLETIONABLE_PARAMETERS: Lazy<Vec<CommandParameterType>> = Lazy::new(|| {
    vec![
        CommandParameterType::Hash,
        CommandParameterType::CommandName,
    ]
});

pub fn is_auto_completionable(parameter_type: CommandParameterType) -> bool {
    AUTO_COMPLETIONABLE_PARAMETERS
        .iter()
        .any(|p| *p == parameter_type)
}
