use crate::{byte_stream::ByteStream, import, malloc};

use super::data::LogicNpcData;

pub trait LogicAvatar: Sized {
    fn new_from_ptr(ptr: *const u8) -> Self;
    fn new() -> Self;
    fn decode(&mut self, stream: &mut ByteStream);
    fn encode(&self, stream: &mut ByteStream);
}

#[repr(transparent)]
pub struct LogicClientAvatar(pub *const u8);

impl LogicClientAvatar {
    pub fn new() -> Self {
        import!(logic_client_avatar_ctor(ptr: *const u8) -> () = 0x1164A4 + 1);
        let instance = malloc(296);
        logic_client_avatar_ctor(instance);
        Self(instance)
    }

    pub fn get_default_avatar() -> Self {
        import!(logic_avatar_set_resource_count(ptr: *const u8, data: *const u8, count: i32) -> () = 0x114E7C + 1);
        import!(logic_data_tables_get_gold_data() -> *const u8 = 0x12D4F8 + 1);
        import!(logic_data_tables_get_elixir_data() -> *const u8 = 0x12D510 + 1);

        let avatar = Self::new();
        logic_avatar_set_resource_count(avatar.0, logic_data_tables_get_gold_data(), 750);
        logic_avatar_set_resource_count(avatar.0, logic_data_tables_get_elixir_data(), 750);

        unsafe {
            *(avatar.0.wrapping_add(96) as *mut i32) = 1; // level
            *(avatar.0.wrapping_add(208) as *mut i32) = 100_000_000; // diamonds
        }

        avatar
    }

    pub fn decode(&mut self, stream: &mut ByteStream) {
        import!(logic_client_avatar_decode(ptr: *const u8, s: *const u8) -> () = 0x1174AC + 1);
        logic_client_avatar_decode(self.0, stream.0);
    }

    pub fn encode(&self, stream: &mut ByteStream) {
        import!(logic_client_avatar_encode(ptr: *const u8, s: *const u8) -> () = 0x115D24 + 1);
        logic_client_avatar_encode(self.0, stream.0);
    }
}

#[repr(transparent)]
pub struct LogicNpcAvatar(pub *const u8);

impl LogicNpcAvatar {
    pub fn new() -> Self {
        import!(logic_npc_avatar_ctor(ptr: *const u8) -> () = 0x117FE4 + 1);
        let instance = malloc(112);
        logic_npc_avatar_ctor(instance);
        Self(instance)
    }

    pub fn set_npc_data(&mut self, data: &LogicNpcData) {
        import!(logic_npc_avatar_set_npc_data(ptr: *const u8, data: *const u8) -> () = 0x118008 + 1);
        logic_npc_avatar_set_npc_data(self.0, data.0);
    }

    pub fn decode(&mut self, stream: &mut ByteStream) {
        import!(logic_npc_avatar_decode(ptr: *const u8, s: *const u8) -> () = 0x1180E4 + 1);
        logic_npc_avatar_decode(self.0, stream.0);
    }

    pub fn encode(&self, stream: &mut ByteStream) {
        import!(logic_npc_avatar_encode(ptr: *const u8, s: *const u8) -> () = 0x117FB8 + 1);
        logic_npc_avatar_encode(self.0, stream.0);
    }
}

impl LogicAvatar for LogicClientAvatar {
    fn new_from_ptr(ptr: *const u8) -> Self {
        Self(ptr)
    }

    fn new() -> Self {
        Self::new()
    }

    fn decode(&mut self, stream: &mut ByteStream) {
        self.decode(stream);
    }

    fn encode(&self, stream: &mut ByteStream) {
        self.encode(stream);
    }
}

impl LogicAvatar for LogicNpcAvatar {
    fn new_from_ptr(ptr: *const u8) -> Self {
        Self(ptr)
    }

    fn new() -> Self {
        Self::new()
    }

    fn decode(&mut self, stream: &mut ByteStream) {
        self.decode(stream);
    }

    fn encode(&self, stream: &mut ByteStream) {
        self.encode(stream);
    }
}
