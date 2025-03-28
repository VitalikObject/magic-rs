use crate::{network::PiranhaMessage, sc_string::ScString};

pub struct ChangeAvatarNameMessage(pub PiranhaMessage);

impl ChangeAvatarNameMessage {
    pub fn get_avatar_name(&self) -> String {
        unsafe { ScString(*(self.0 .0.wrapping_add(48) as *const *const u8)).to_string() }
    }
}
