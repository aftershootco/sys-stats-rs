use crate::soc::Soc;
use crate::SocDetails;
use std::process::Command;

impl SocDetails {
    pub fn get_current_soc_info() -> Soc {
        // parse the soc.json file and return the Soc struct

        // let s = include_str!("db/apple/soc.json");
        // let soc_details: Result<SocCollection, Error> = serde_json::from_str(s);
        // let (name, cc) = Self::get_name_and_core_count();

        Soc::new(None, None, None, 0, None, None, None, None, 0)
    }

    fn get_name_and_core_count() -> (String, u32) {
        let output = Command::new("sh")
            .arg("-c")
            .arg("cat /proc/cpuinfo | grep 'model name' | uniq")
            .output()
            .expect("Failed to execute command");

        let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let core_count = sys_info::cpu_num().unwrap();

        (name.to_string(), core_count)
    }
}
