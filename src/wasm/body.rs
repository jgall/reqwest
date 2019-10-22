/// dox
#[derive(Debug)]
pub struct Body(String);

impl Body {
    pub(crate) fn body_str(&self) -> &str {
        &self.0
    }
}
