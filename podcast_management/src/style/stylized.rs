use super::color::Color;

pub type StylizedContent<'a> = Vec<(&'a str, Option<Vec<Style>>)>;

#[derive(PartialEq, Debug)]
pub enum Style {
    Bold,
    Italic,
    Underlined,
    Color(Color),
    Background(Color),
}

pub trait Stylized {
    fn to_stylized(&self) -> StylizedContent;
}

impl Stylized for String {
    fn to_stylized(&self) -> StylizedContent {
        vec![(&self, None)]
    }
}

#[cfg(test)]
mod tests {
    use super::Stylized;

    #[test]
    fn test_string_stylized() -> Result<(), String> {
        let s = String::from("toto");
        let s_stylized = s.to_stylized();
        assert_eq!(s.to_string(), s_stylized[0].0);
        assert_eq!(None, s_stylized[0].1);
        Ok(())
    }
}
