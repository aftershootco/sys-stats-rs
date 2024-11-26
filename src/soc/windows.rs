use crate::soc::Soc;
use crate::SocDetails;

impl SocDetails {
    pub fn get_current_soc_info() -> Soc {
        Soc::new(None, None, None, 0, None, None, None, None, 0)
    }
}
