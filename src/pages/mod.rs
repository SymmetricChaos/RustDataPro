pub mod reliability;
pub mod session_page;
pub mod sidebar;

pub use reliability::*;
pub use session_page::*;
pub use sidebar::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Page {
    About,
    Session,
    Reliability,
}
