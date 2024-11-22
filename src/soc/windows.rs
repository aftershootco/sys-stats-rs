use crate::soc::{Soc, SocCollection};
use crate::SocDetails;

impl SocDetails {
    /// Get the SOC details by name
    /// TODO: when multiple SOCs are available, match more details or ID etc
    pub fn get_soc_info_by_name(name: &str) -> Soc {
        Soc::new(None, None, None, 0, None, None, None, None, 0)
    }

    pub fn get_current_soc_info() -> Soc {
        Soc::new(None, None, None, 0, None, None, None, None, 0)
    }
}
