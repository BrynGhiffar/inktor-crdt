use vect_crdt_rs::prelude::*;

#[test]
fn test_merge() {
    let mut doc1 = SVGDoc::new("1".to_string());
    doc1.add_circle(None, PartialSVGCircle::empty());
    let Some(merge_str) = doc1.save() else { return; };
    let mut doc2 = SVGDoc::new("2".to_string());
    doc2.merge(merge_str);
    let children1 =  doc1.children();
    let children2 = doc2.children();
    let Some(SVGObject::Circle(expected_circle)) = children1.children.first() else { 
        panic!("Expected circle should not be none");
    };
    let Some(SVGObject::Circle(result_circle)) = children2.children.first() else { 
        panic!("Result circle should not be none");
    };
    assert_eq!(expected_circle.id, result_circle.id);
}