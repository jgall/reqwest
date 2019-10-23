/// dox
#[derive(Debug)]
pub struct Body(String);

impl Body {
    pub(crate) fn body_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Body {
    fn from(s: &str) -> Body {
        Body(s.to_owned())
    }
}

impl From<String> for Body {
    fn from(s: String) -> Body {
        Body(s)
    }
}
