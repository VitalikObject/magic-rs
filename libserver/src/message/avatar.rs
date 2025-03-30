use crate::{import, logic::avatar::LogicClientAvatar, malloc, math::LogicLong, network::PiranhaMessage, sc_string::ScString, helper::compress_in_zlib_format};

pub struct ChangeAvatarNameMessage(pub PiranhaMessage);

impl ChangeAvatarNameMessage {
    pub fn get_avatar_name(&self) -> String {
        unsafe { ScString(*(self.0 .0.wrapping_add(48) as *const *const u8)).to_string() }
    }
}

pub struct AskForAvatarProfileMessage(pub PiranhaMessage);

impl AskForAvatarProfileMessage {
    pub fn get_account_id(&self) -> &LogicLong {
        unsafe { &**(self.0 .0.wrapping_add(48) as *const *const LogicLong) }
    }   
}

pub struct AvatarProfileMessage(pub PiranhaMessage);

impl AvatarProfileMessage {
    pub fn new() -> Self {
        import!(avatar_profile_message(ptr: *const u8) -> () = 0x20FA92);

        let instance = malloc(52);
        avatar_profile_message(instance);
        Self(PiranhaMessage(instance))
    }

    pub fn set_avatar_profile_full_entry(&mut self, avatar_profile_full_entry: AvatarProfileFullEntry) {
        unsafe { *(self.0 .0.wrapping_add(48) as *mut usize) = avatar_profile_full_entry.0 as usize }
    }
}

#[repr(transparent)]
pub struct AvatarProfileFullEntry(pub *const u8);

impl AvatarProfileFullEntry {
    pub fn new() -> Self {
        import!(avatar_profile_full_entry(ptr: *const u8) -> () = 0x20F6B4);

        let instance = malloc(32);
        avatar_profile_full_entry(instance);
        Self(instance)
    }

    pub fn set_home_json(&mut self, home_json: &str) {
        unsafe {
            let compressed = compress_in_zlib_format(home_json.as_bytes());
            *(self.0.wrapping_add(8) as *mut i32) = compressed.len() as i32;
            let compressed_ptr = malloc(compressed.len());
            std::slice::from_raw_parts_mut(compressed_ptr as *mut u8, compressed.len()).copy_from_slice(compressed.as_slice());
            *(self.0.wrapping_add(4) as *mut *const u8) = compressed_ptr;
        }
    }    

    pub fn set_logic_client_avatar(&mut self, logic_client_avatar: LogicClientAvatar) {
        unsafe { *(self.0.wrapping_add(0) as *mut usize) = logic_client_avatar.0 as usize }
    }    
}