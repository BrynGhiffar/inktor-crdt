use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct LWWReg<T> where T: Clone {
    val: T,
    time: UnixEpochTimeNanos
}

impl<T> LWWReg<T> where T: Clone {
    pub fn new(val: T) -> LWWReg<T> {
        let time = epoch_now_nanos();
        Self { val, time }
    }
    pub fn value(&self) -> &T {
        &self.val
    }

    pub fn set(&mut self, val: T) {
        self.val = val;
        self.time = epoch_now_nanos();
    }

    pub fn merge(a: &LWWReg<T>, b: &LWWReg<T>) -> LWWReg<T> {
        if a.time < b.time {
            return LWWReg { val: b.val.clone(), time: b.time.clone() };
        }
        return LWWReg { val: a.val.clone(), time: a.time.clone() };
    }
}