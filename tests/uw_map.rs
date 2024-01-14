use vect_crdt_rs::prelude::*;

#[test]
fn test_insert() {
    let r1 = "r1";
    let mut map1 = UWMap::<String, String>::new();
    map1.insert(r1.to_string(), "key".to_string(), "thing".to_string());
    let res = map1.get(&"key".to_string());
    assert_eq!(res, Some(&"thing".to_string()))
}

#[test]
fn test_delete() {
    let r1 = "r1";
    let mut map1 = UWMap::<String, String>::new();
    map1.insert(r1.to_string(), "key".to_string(), "thing".to_string());
    map1.remove(r1.to_string(), "key".to_string());
    let res = map1.get(&"key".to_string());
    assert_eq!(res, None);
}

#[test]
fn test_merge_update() {
    let r1 = "r1";
    let r2 = "r2";
    let mut map1 = UWMap::<&str, &str>::new();
    map1.insert(r1.to_string(), "key", "thing");
    let mut map2 = map1.clone();

    map2.insert(r2.to_string(), "key", "thong");
    let res2 = map2.get(&"key");
    assert_eq!(res2, Some(&"thong"));

    map1.remove(r1.to_string(), "key");
    let res1 = map1.get(&"key");
    assert_eq!(res1, None);

    let merge_map = UWMap::merge(&map1, &map2);
    let res_merge = merge_map.get(&"key");
    assert_eq!(res_merge, Some(&"thong"));
}

#[test]
fn test_last_write_wins_reg() {
    let r1 = "r1";
    let r2 = "r2";
    let mut map1 = UWMap::<&str, &str>::new();
    let mut map2 = UWMap::<&str, &str>::new();

    map1.insert(r1.to_string(), "key", "thing");
    map2.insert(r2.to_string(), "key", "thong");

    let merge_map = UWMap::merge(&map1, &map2);
    let res_merge = merge_map.get(&"key");

    assert_eq!(res_merge, Some(&"thong"))
}

#[test]
fn test_all_keys_exist() {
    let r1 = "r1";
    let r2 = "r2";

    let mut map1 = UWMap::<&str, &str>::new();
    let mut map2 = UWMap::<&str, &str>::new();

    map1.insert(r1.to_string(), "ka", "va");
    map1.insert(r1.to_string(), "kb", "vb");
    map1.insert(r1.to_string(), "kc", "vc");

    map2.insert(r2.to_string(), "kd", "vd");
    map2.insert(r2.to_string(), "ke", "ve");
    map2.insert(r2.to_string(), "kf", "vf");

    let merge_map = UWMap::merge(&map1, &map2);
    let mm = merge_map.value();

    assert_eq!(mm.get("ka"), Some(&"va"));
    assert_eq!(mm.get("kb"), Some(&"vb"));
    assert_eq!(mm.get("kc"), Some(&"vc"));
    assert_eq!(mm.get("kd"), Some(&"vd"));
    assert_eq!(mm.get("ke"), Some(&"ve"));
    assert_eq!(mm.get("kf"), Some(&"vf"));
}