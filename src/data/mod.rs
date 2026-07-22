pub mod assessment_data;
pub mod client_data;
pub mod ioa_data;
pub mod ksf_data;
pub mod output_data;
pub mod session_data;
pub mod timeline;
pub mod timer;

pub use assessment_data::*;
pub use client_data::*;
pub use ioa_data::*;
pub use ksf_data::*;
pub use output_data::*;
pub use session_data::*;
pub use timeline::*;
pub use timer::*;

#[derive(Debug, Default)]
pub struct Data {
    pub client: ClientData,
    pub session: SessionData,
    pub assessments: AssessmentsData,
    pub ksf: KsfData,
}

impl Data {
    pub fn clear(&mut self) {
        *self = Self::default()
    }
}
