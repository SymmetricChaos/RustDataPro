use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum Page {
    About,
    Randomness,
    DataTracking,
}
