use crate::{editor::Line, prelude::*};

pub struct SearchInfo {
    pub prev_location: Location,
    pub prev_scroll_offset: Position,
    pub query: Option<Line>,
}
