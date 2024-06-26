use crate::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);
}

#[cfg(feature = "debug")]
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format!("[RUST_WASM_DEBUG]: {}", &format_args!($($t)*).to_string())))
}

#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {}
}

// pub fn gen_str_id() -> String {
//     return StringGenerator::default().next_id();
// }

pub fn gen_str_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}

#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Tsify, Clone, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Color(
    pub i32 /* red (0 - 255) */, 
    pub i32 /* green (0 - 255) */, 
    pub i32 /* blue (0 - 255) */, 
    pub f32 /* (0 - 100) */
);

impl Color {
    pub fn white() -> Color {
        Color(255, 255, 255, 1.0)
    }

    pub fn black() -> Color {
        Color(0, 0, 0, 1.0)
    }
}


pub type UnixEpochTimeNanos = u128;
pub type ReplicaId = String;
pub type NodeID = String;

pub fn epoch_now_nanos() -> UnixEpochTimeNanos {
    epoch_now().as_nanos()
}

pub fn epoch_now() -> Duration {
    let start = SystemTime::now();
    let duration = start
        .duration_since(UNIX_EPOCH)
        .unwrap();
    duration
}
pub trait HashableItem: PartialEq + Eq + Hash + Clone { }
impl<T> HashableItem for T where T: PartialEq + Eq + Hash + Clone { }