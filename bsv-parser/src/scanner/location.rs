use crate::Span;
use std::sync::Arc;
use url::Url;

pub struct Location {
    pub uri: Arc<Url>,
    pub range: Span,
}

impl Location {
    pub fn new(uri: Arc<Url>, range: Span) -> Location {
        Location { uri, range }
    }
}
