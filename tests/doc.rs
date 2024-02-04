use vect_crdt_rs::prelude::*;

fn empty_circle() -> PartialSVGCircle {
    PartialSVGCircle {
        fill: None,
        opacity: None,
        pos: None,
        radius: None,
        stroke: None,
        stroke_width: None
    }
}

fn empty_rectangle() -> PartialSVGRectangle {
    PartialSVGRectangle {
        fill: None,
        pos: None,
        height: None,
        width: None,
        stroke_width: None,
        stroke: None,
        opacity: None
    }
}

#[test]
fn test_create_circle() {
    let mut doc = SVGDoc::new("1".to_string());
    let circle = empty_circle();
    doc.add_circle(None, circle);
    assert!(doc.children().children.iter().all(|o| {
        match o {
            SVGObject::Circle(_) => true,
            _  => false
        }
    }));
    assert_eq!(doc.children().children.len(), 1)
}

#[test]
fn test_edit_circle() {
    let mut doc = SVGDoc::new("1".to_string());
    let circle = empty_circle();
    doc.add_circle(None, circle);
    assert_eq!(doc.children().children.len(), 1);
    let circle_id = doc.children().children[0].get_id().to_string();
    let mut circle_edits = empty_circle();
    circle_edits.pos = Some(Vec2 { x: 100, y: 50 });
    doc.edit_circle(circle_id, circle_edits);
    assert!(doc.children().children.iter().all(|o| {
        match o {
            SVGObject::Circle(c) => {
                if !c.pos.x == 100 {
                    return false;
                }
                if !c.pos.y == 50 {
                    return false;
                }
                true
            },
            _ => false,
        }
    }));
    assert_eq!(doc.children().children.len(), 1);
}

#[test]
fn test_remove_circle() {
    let mut doc = SVGDoc::new("1".to_string());
    let circle = empty_circle();
    doc.add_circle(None, circle);
    assert!(doc.children().children.iter().all(|o| {
        match o {
            SVGObject::Circle(_) => true,
            _  => false
        }
    }), "Assert circle exists");
    assert_eq!(doc.children().children.len(), 1, "Assert circle only element");
    
    let id = match &doc.children().children[0] {
        SVGObject::Circle(c) => c.id.clone(),
        _ => panic!("Child should be a circle")
    };

    doc.remove_object(id);
    assert_eq!(doc.children().children.len(), 0, "Assert circle does not exist");
}

#[test]
fn test_create_rectangle() {
    let mut doc = SVGDoc::new("1".to_string());
    let rect = empty_rectangle();

    doc.add_rectangle(None, rect);
    assert!(doc.children().children.iter().all(|o| {
        match o {
            SVGObject::Rectangle(_) => true,
            _ => false
        }
    }), "Assert circle only element");
    assert_eq!(doc.children().children.len(), 1, "Assert rectangle only element")
}

#[test]
fn test_edit_rectangle() {
    let mut doc = SVGDoc::new("1".to_string());
    let rect = empty_rectangle();

    doc.add_rectangle(None, rect);

    let rect_id = doc.children().children[0].get_id().to_string();
    let mut rect_edits = empty_rectangle();
    rect_edits.opacity = Some(0.5);
    doc.edit_rectangle(rect_id, rect_edits);
    assert!(doc.children().children.iter().all(|o| {
        match o {
            SVGObject::Rectangle(o) => {
                if !(o.opacity == 0.5) { return false; }
                return true
            },
            _ => false
        }
    }));
    assert_eq!(doc.children().children.len(), 1, "assert only rectangle exists");
}

#[test]
fn test_move_ancestor_into_grandchild_failed() {
    let mut doc = SVGDoc::new("1".to_string());
    doc.add_group(None, PartialSVGGroup::empty());
    let group1_id = match &doc.children().children[0] {
        SVGObject::Group(g) => g.id.clone(),
        _ => panic!("First should be group")
    };
    doc.add_group(Some(group1_id.clone()), PartialSVGGroup::empty());
    let children = doc.children();
    let group2_id = match children.children.get(0) {
        Some(SVGObject::Group(g)) => {
            match &g.children.get(0) {
                Some(SVGObject::Group(g)) => g.id.clone(),
                _ => panic!("Child group should exist")
            }
        },
        _ => panic!("Group should exist")
    };
    assert_eq!(doc.children().children.len(), 1);

    // Operation must fail. With nothing changed in the tree.
    doc.move_object_to_group(group1_id.clone(), group2_id.clone(), 0);

    assert_eq!(doc.children().children.len(), 1);

    let exp_group1_id = match &doc.children().children[0] {
        SVGObject::Group(g) => g.id.clone(),
        _ => panic!("First should be group")
    };
    let children = doc.children();
    let exp_group2_id = match children.children.get(0) {
        Some(SVGObject::Group(g)) => {
            match &g.children.get(0) {
                Some(SVGObject::Group(g)) => g.id.clone(),
                _ => panic!("Child group should exist")
            }
        },
        _ => panic!("Group should exist")
    };

    assert_eq!(group1_id, exp_group1_id);
    assert_eq!(group2_id, exp_group2_id);
}