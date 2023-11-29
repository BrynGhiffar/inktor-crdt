use vect_crdt_rs::SVGDoc;

#[test]
fn something() {
    let a: i32 = 1;
    let b: i32 = 1;
    let res = a.checked_add(b);
    assert_eq!(Some(2), res);
}

#[test]
fn test_svgdoc() {
    let doc = SVGDoc::new();
    assert_eq!(doc.repr(), String::from("cat"));
}