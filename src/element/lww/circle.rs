use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct LWWSVGCircle {
    pub id: LWWReg<NodeID>,
    pub pos: LWWReg<Vec2>,
    pub radius: LWWReg<i32>,
    pub fill: LWWReg<Color>,
    pub stroke_width: LWWReg<i32>,
    pub stroke: LWWReg<Color>,
    pub opacity: LWWReg<f32>,
}

impl LWWSVGCircle {
    pub fn value(&self) -> SVGCircle {
        SVGCircle { 
            id: self.id.value().clone(), 
            pos: self.pos.value().clone(), 
            radius: self.radius.value().clone(), 
            fill: self.fill.value().clone(), 
            stroke_width: self.stroke_width.value().clone(), 
            stroke: self.stroke.value().clone(), 
            opacity: self.opacity.value().clone() 
        }
    }
}

impl From<SVGCircle> for LWWSVGCircle {
    fn from(SVGCircle { 
        id, 
        pos, 
        radius, 
        fill, 
        stroke_width, 
        stroke, 
        opacity 
    }: SVGCircle) -> Self {
        Self { 
            id: LWWReg::new(id), 
            pos: LWWReg::new(pos), 
            radius: LWWReg::new(radius), 
            fill: LWWReg::new(fill), 
            stroke_width: LWWReg::new(stroke_width), 
            stroke: LWWReg::new(stroke), 
            opacity: LWWReg::new(opacity)
        }
    }
}

impl Mergeable for LWWSVGCircle {
    fn merge(&self, other: &Self) -> Self {
        Self { 
            id: self.id.merge(&other.id), 
            pos: self.pos.merge(&other.pos), 
            radius: self.radius.merge(&other.radius), 
            fill: self.fill.merge(&other.fill), 
            stroke_width: self.stroke_width.merge(&other.stroke_width), 
            stroke: self.stroke.merge(&other.stroke), 
            opacity: self.opacity.merge(&other.opacity)
        }
    }
}

impl partially::Partial for LWWSVGCircle {
    type Item = PartialSVGCircle;
    fn apply_some(&mut self, partial: Self::Item) -> bool {
        let will_apply_some = partial.pos.is_some() 
            || partial.radius.is_some()
            || partial.fill.is_some() 
            || partial.stroke_width.is_some()
            || partial.stroke.is_some() 
            || partial.opacity.is_some();
        if let Some(pos) = partial.pos {
            self.pos.set(pos);
        }
        if let Some(radius) = partial.radius {
            self.radius.set(radius);
        }
        if let Some(fill) = partial.fill {
            self.fill.set(fill);
        }
        if let Some(stroke_width) = partial.stroke_width {
            self.stroke_width.set(stroke_width);
        }
        if let Some(stroke) = partial.stroke {
            self.stroke.set(stroke);
        }
        if let Some(opacity) = partial.opacity {
            self.opacity.set(opacity);
        }
        will_apply_some
    }
}