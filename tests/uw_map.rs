use vect_crdt_rs::prelude::*;

#[test]
fn test_insert() {
    let r1 = "r1";
    let mut map1 = UWMap::<String, LWWReg<String>>::new();
    map1.insert(r1.to_string(), "key".to_string(), LWWReg::new("thing".to_string()));
    let res = map1.get(&"key".to_string()).map(|v| v.value());
    assert_eq!(res, Some(&"thing".to_string()));
}

#[test]
fn test_delete() {
    let r1 = "r1";
    let mut map1 = UWMap::<String, LWWReg<String>>::new();
    map1.insert(r1.to_string(), "key".to_string(), LWWReg::new("thing".to_string()));
    map1.remove(r1.to_string(), "key".to_string());
    let vv = map1.value();
    let res = map1.get(&"key".to_string());
    assert_eq!(res.map(|v| v.value()), None);
    assert_eq!(vv.get("key").map(|v| v.value()), None)
}

#[test]
fn test_merge_update() {
    let r1 = "r1";
    let r2 = "r2";
    let mut map1 = UWMap::<&str, LWWReg<&str>>::new();
    map1.insert(r1.to_string(), "key", LWWReg::new("thing"));
    let mut map2 = map1.clone();

    map2.insert(r2.to_string(), "key", LWWReg::new("thong"));
    let res2 = map2.get(&"key");
    assert_eq!(res2.map(|v| v.value()), Some(&"thong"));

    map1.remove(r1.to_string(), "key");
    let res1 = map1.get(&"key");
    assert_eq!(res1.map(|v| v.value()), None);

    let merge_map = UWMap::merge(&map1, &map2);
    let res_merge = merge_map.get(&"key");
    assert_eq!(res_merge.map(|v| v.value()), Some(&"thong"));
}

#[test]
fn test_last_write_wins_reg() {
    let r1 = "r1";
    let r2 = "r2";
    let mut map1 = UWMap::<&str, LWWReg<&str>>::new();
    let mut map2 = UWMap::<&str, LWWReg<&str>>::new();

    map1.insert(r1.to_string(), "key", LWWReg::new("thing"));
    map2.insert(r2.to_string(), "key", LWWReg::new("thong"));

    let merge_map = UWMap::merge(&map1, &map2);
    let res_merge = merge_map.get(&"key");

    assert_eq!(res_merge.map(|v| v.value()), Some(&"thong"))
}

#[test]
fn test_all_keys_exist() {
    let r1 = "r1";
    let r2 = "r2";

    let mut map1 = UWMap::<&str, LWWReg<&str>>::new();
    let mut map2 = UWMap::<&str, LWWReg<&str>>::new();

    map1.insert(r1.to_string(), "ka", LWWReg::new("va"));
    map1.insert(r1.to_string(), "kb", LWWReg::new("vb"));
    map1.insert(r1.to_string(), "kc", LWWReg::new("vc"));

    map2.insert(r2.to_string(), "kd", LWWReg::new("vd"));
    map2.insert(r2.to_string(), "ke", LWWReg::new("ve"));
    map2.insert(r2.to_string(), "kf", LWWReg::new("vf"));

    let merge_map = UWMap::merge(&map1, &map2);
    let mm = merge_map.value();

    assert_eq!(mm.get("ka").map(|v| v.value()), Some(&"va"));
    assert_eq!(mm.get("kb").map(|v| v.value()), Some(&"vb"));
    assert_eq!(mm.get("kc").map(|v| v.value()), Some(&"vc"));
    assert_eq!(mm.get("kd").map(|v| v.value()), Some(&"vd"));
    assert_eq!(mm.get("ke").map(|v| v.value()), Some(&"ve"));
    assert_eq!(mm.get("kf").map(|v| v.value()), Some(&"vf"));
}