// Update Wins Map
use crate::prelude::*;

pub trait Mergeable {
    // Merge must be commutative
    fn merge(&self, other: &Self) -> Self;
}

pub trait UWMapKey: HashableItem {}
pub trait UWMapItem: Clone {}

impl<T> UWMapKey for T where T: HashableItem {}
impl<T> UWMapItem for T where T: Clone {}

#[derive(Clone, Serialize, Deserialize)]
pub struct UWMap<K, V> where K: UWMapKey, V: UWMapItem + Mergeable {
    removed: HashMap<K, VTime>,
    updated: HashMap<K, VTime>,
    kv: HashMap<K, V>
}

impl<K, V> UWMap<K, V> where K: UWMapKey, V: UWMapItem + Mergeable {
    pub fn new() -> UWMap<K, V> {
        Self {
            removed: HashMap::new(),
            updated: HashMap::new(),
            kv: HashMap::new()
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let updated_vtime = self.updated.get(key)?;
        if let Some(removed_vtime) = self.removed.get(&key) {
            if updated_vtime.partial_cmp(removed_vtime) == Some(Ordering::Less) {
                return None;
            } else {
                return self.kv.get(key);
            }
        }
        return self.kv.get(key);
    }

    pub fn value(&self) -> HashMap<K, V> {
        let init = self.updated.clone();
        let keys = self.removed.iter().fold(init, |mut prev, (key, vtime_removed)| { 
            let vtime_added = prev.get(key);
            if let Some(vtime_added) = vtime_added {
                if vtime_added.partial_cmp(vtime_removed) == Some(Ordering::Less) {
                    prev.remove(key);
                }
            }
            prev
        });

        keys.iter().fold(HashMap::new(), |mut prev, (key, _)| {
            let value = self.kv.get(key);
            if let Some(value) = value {
                prev.insert(key.clone(), value.clone());
            }
            prev
        })
    }

    pub fn insert_novtime_update(
        &mut self,
        key: K,
        value: V
    ) {
        let update_vtime = self.updated.remove(&key);
        let remove_vtime = self.removed.remove(&key);
        match (update_vtime, remove_vtime) {
            (Some(vtime), _) => {
                self.updated.insert(key.clone(), vtime);
                self.kv.insert(key, value);
            },
            (_, Some(vtime)) => {
                self.updated.insert(key.clone(), vtime);
                self.kv.insert(key, value);
            },
            (_, _) => {
                let vtime = VTime::zero();
                self.updated.insert(key.clone(), vtime);
                self.kv.insert(key, value);
            }
        }
    }

    pub fn insert(
        &mut self,
        replica_id: ReplicaId,
        key: K,
        value: V
    ) {
        let update_vtime = self.updated.remove(&key);
        let remove_vtime = self.removed.remove(&key);
        match (update_vtime, remove_vtime) {
            (Some(mut vtime), _) => {
                vtime.inc(replica_id);
                self.updated.insert(key.clone(), vtime);
                self.kv.insert(key, value);
            },
            (_, Some(mut vtime)) => {
                vtime.inc(replica_id);
                self.updated.insert(key.clone(), vtime);
                self.kv.insert(key, value);
            },
            (_, _) => {
                let mut vtime = VTime::zero();
                vtime.inc(replica_id);
                self.updated.insert(key.clone(), vtime);
                self.kv.insert(key, value);
            }
        }
    }

    pub fn inc_vtime(
        &mut self,
        replica_id: ReplicaId,
        key: K
    ) {
        let update_vtime = self.updated.remove(&key);
        let remove_vtime = self.removed.remove(&key);
        let Some(value) = self.kv.remove(&key) else { return; };
        match (update_vtime, remove_vtime) {
            (Some(mut vtime), _) => {
                vtime.inc(replica_id);
                self.updated.insert(key.clone(), vtime);
                self.kv.insert(key, value);
            },
            (_, Some(mut vtime)) => {
                vtime.inc(replica_id);
                self.updated.insert(key.clone(), vtime);
                self.kv.insert(key, value);
            },
            (_, _) => {
                let mut vtime = VTime::zero();
                vtime.inc(replica_id);
                self.updated.insert(key.clone(), vtime);
                self.kv.insert(key, value);
            }
        }
    }

    pub fn remove(
        &mut self,
        replica_id: ReplicaId,
        key: K
    ) {
        let updated_vtime = self.updated.remove(&key);
        let removed_vtime = self.updated.remove(&key);

        match (updated_vtime, removed_vtime) {
            (Some(mut vtime), _) => {
                vtime.inc(replica_id);
                self.kv.remove(&key);
                self.removed.insert(key, vtime);
            },
            (_, Some(mut vtime)) => {
                vtime.inc(replica_id);
                self.kv.remove(&key);
                self.removed.insert(key, vtime);
            },
            (_, _) => {
                let mut vtime = VTime::zero();
                vtime.inc(replica_id);
                self.kv.remove(&key);
                self.removed.insert(key, vtime);
            }
        }
    }

    pub fn merge(
        Self { 
            updated: a_upd, 
            removed: a_rem, 
            kv: a_kv 
        }: &Self, 
        Self { 
            updated: b_upd, 
            removed: b_rem, 
            kv: b_kv 
        }: &Self
    ) -> UWMap<K, V> {
        let mut kv = a_kv.iter()
            .fold(b_kv.clone(), |mut acc, (ka, va)| {
                let vb = acc.get_mut(ka);
                if let Some(vb) = vb {
                    *vb = va.merge(vb);
                } else {
                    acc.insert(ka.clone(), va.clone());
                }
                acc
            });
        let all_updated = b_upd.iter()
            .fold(a_upd.clone(), |mut acc, (k, vb)| {
                if let Some(va) = acc.remove(k) {
                    acc.insert(k.clone(), VTime::merge(&va, &vb));
                } else {
                    acc.insert(k.clone(), vb.clone());
                }
                acc
            });
        let all_removed = b_rem.iter()
            .fold(a_rem.clone(), |mut acc, (k, vb)| {
                if let Some(va) = acc.remove(k) {
                    acc.insert(k.clone(), VTime::merge(&va, &vb));
                } else {
                    acc.insert(k.clone(), vb.clone());
                }
                acc
            });
        
        let updated = all_removed.iter()
            .fold(all_updated.clone(), |mut acc, (k, vr)| {
                match acc.get(k) {
                    Some(vu) if vu.partial_cmp(vr) == Some(Ordering::Less) => {
                        acc.remove(k);
                        kv.remove(k);
                    },
                    _ => {}
                }
                acc
            });

        let removed = all_updated.iter()
            .fold(all_removed.clone(), |mut acc, (k, vu)| {
                match acc.get(k) {
                    Some(vr) if vr.partial_cmp(vu) == Some(Ordering::Less) => { 
                        acc.remove(k);
                    },
                    _ => {}
                }
                acc
            });

        UWMap { removed, updated, kv }
    }

}