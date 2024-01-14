use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct VTime(HashMap<ReplicaId, i64>);

impl VTime {
    pub fn zero() -> Self {
        VTime(HashMap::new())
    }

    pub fn inc(&mut self, replica_id: ReplicaId) {
        let Self(map) = self;
        let val = map.get_mut(&replica_id);
        if let Some(val) = val {
            *val += 1;
        } else {
            map.insert(replica_id, 1);
        }
    }

    pub fn merge(a: &VTime, b: &VTime) -> VTime {
        let Self(map_b) = b;
        let Self(map_a) = a;
        let map_a = map_a.clone();
        let map_b = map_b.clone();
        map_a.iter().fold(map_b, |mut acc, (ka, va)| { 
            let vb = acc.get_mut(ka);
            if let Some(vb) = vb {
                *vb = *va.max(vb);
            } else {
                acc.insert(ka.clone(), va.clone());
            }
            acc
        });
        Self(map_a)
    }
}

impl PartialEq for VTime {
    fn eq(&self, other: &Self) -> bool {
        let Self(self_map) = self;
        let self_keys = self_map
            .keys()
            .map(|k| k.clone())
            .collect::<HashSet<_>>();
        let Self(other_map) = other;
        let other_keys = other_map
            .keys()
            .map(|k| k.clone())
            .collect::<HashSet<_>>();

        let keys = self_keys.union(&other_keys);
        keys.into_iter().fold(true, |acc, nxt| {
            let va = self_map.get(nxt).map(|v| v.clone()).unwrap_or(0);
            let vb = other_map.get(nxt).map(|v| v.clone()).unwrap_or(0);
            if acc {
                return va == vb;
            } else {
                return false;
            }
        })
    }
}

impl Eq for VTime {}

impl PartialOrd for VTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let Self(self_map) = self;
        let self_keys = self_map
            .keys()
            .map(|k| k.clone())
            .collect::<HashSet<_>>();
        let Self(other_map) = other;
        let other_keys = self_map
            .keys()
            .map(|k| k.clone())
            .collect::<HashSet<_>>();

        let keys = self_keys.union(&other_keys);

        keys.into_iter()
            .fold(Some(Ordering::Equal), |acc, nxt| {
                let va = self_map.get(nxt)
                    .map(|v| v.clone())
                    .unwrap_or(0);
                let vb = other_map
                    .get(nxt)
                    .map(|v| v.clone())
                    .unwrap_or(0);
                match acc {
                    Some(Ordering::Equal) if va > vb => Some(Ordering::Greater),
                    Some(Ordering::Equal) if va < vb => Some(Ordering::Less),
                    Some(Ordering::Less) if va > vb => None,
                    Some(Ordering::Greater) if va < vb => None,
                    prev => prev
                }
            })
    }
}