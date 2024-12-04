pub mod handlers;

pub mod pb {
    include!("api/gen/subscription.rs");
    pub const DESCRIPTOR_SET: &[u8] = include_bytes!("api/gen/description.bin");
}
