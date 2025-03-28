use crate::sc_string::ScString;

pub struct LogicNpcData(pub *const u8);

impl LogicNpcData {
    pub fn get_level_json_file_name(&self) -> ScString {
        ScString(self.0.wrapping_add(144) as *const u8)
    }
}
