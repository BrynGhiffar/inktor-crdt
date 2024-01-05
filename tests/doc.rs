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
    let mut doc = SVGDoc::new();
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
    let mut doc = SVGDoc::new();
    let circle = empty_circle();
    doc.add_circle(None, circle);
    doc.children().children.iter_mut().for_each(|o| {
        match o {
            SVGObject::Circle(c) => c.set_pos(100, 50),
            _ => ()
        }
    });
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
    let mut doc = SVGDoc::new();
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
    let mut doc = SVGDoc::new();
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
    let mut doc = SVGDoc::new();
    let rect = empty_rectangle();

    doc.add_rectangle(None, rect);

    
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