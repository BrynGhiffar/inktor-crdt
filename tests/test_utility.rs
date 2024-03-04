use inktor_crdt::prelude::*;

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