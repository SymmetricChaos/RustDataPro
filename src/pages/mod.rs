pub mod about;
pub mod random_services;
pub mod reliability;
pub mod session_page;
pub mod sidebar;
pub mod timers;

pub use about::*;
pub use random_services::*;
pub use reliability::*;
pub use session_page::*;
pub use sidebar::*;
pub use timers::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Page {
    About,
    Session,
    Reliability,
}
