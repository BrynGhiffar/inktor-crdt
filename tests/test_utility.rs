use vect_crdt_rs::prelude::*;

pub fn empty_circle() -> PartialSVGCircle {
    PartialSVGCircle {
        fill: None,
        opacity: None,
        pos: None,
        radius: None,
        stroke: None,
        stroke_width: None
    }
}

pub fn empty_rectangle() -> PartialSVGRectangle {
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

pub fn empty_group() -> PartialSVGGroup {
    PartialSVGGroup {
        fill: None,
        stroke_width: None,
        stroke: None,
    }
}
