use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct LWWSVGRectangle {
    pub id: LWWReg<NodeID>,
    pub pos: LWWReg<Vec2>,
    pub height: LWWReg<i32>,
    pub width: LWWReg<i32>,
    pub fill: LWWReg<Color>,
    pub stroke_width: LWWReg<i32>,
    pub stroke: LWWReg<Color>,
    pub opacity: LWWReg<f32>
}

impl LWWSVGRectangle {
    pub fn value(&self) -> SVGRectangle {
        SVGRectangle {
            id: self.id.value().clone(),
            pos: self.pos.value().clone(),
            height: self.height.value().clone(),
            width: self.width.value().clone(),
            fill: self.fill.value().clone(),
            stroke_width: self.stroke_width.value().clone(),
            stroke: self.stroke.value().clone(),
            opacity: self.opacity.value().clone()
        }
    }
}

impl From<SVGRectangle> for LWWSVGRectangle {
    fn from(SVGRectangle { 
        id, 
        pos, 
        height, 
        width, 
        fill, 
        stroke_width, 
        stroke, 
        opacity 
    }: SVGRectangle) -> Self {
        Self { 
            id: LWWReg::new(id), 
            pos: LWWReg::new(pos), 
            height: LWWReg::new(height), 
            width: LWWReg::new(width), 
            fill: LWWReg::new(fill), 
            stroke_width: LWWReg::new(stroke_width), 
            stroke: LWWReg::new(stroke), 
            opacity: LWWReg::new(opacity) 
        }
    }
}

impl Mergeable for LWWSVGRectangle {
    fn merge(&self, other: &Self) -> Self {
        Self { 
            id: self.id.merge(&other.id), 
            pos: self.pos.merge(&other.pos), 
            height: self.height.merge(&other.height), 
            width: self.width.merge(&other.width), 
            fill: self.fill.merge(&other.fill), 
            stroke_width: self.stroke_width.merge(&other.stroke_width), 
            stroke: self.stroke.merge(&other.stroke), 
            opacity: self.opacity.merge(&other.opacity)
        }
    }
}

impl partially::Partial for LWWSVGRectangle {
    type Item = PartialSVGRectangle;
    fn apply_some(&mut self, partial: Self::Item) -> bool {
        let will_apply_some = partial.pos.is_some() 
            || partial.height.is_some()
            || partial.width.is_some() 
            || partial.fill.is_some()
            || partial.stroke_width.is_some() 
            || partial.stroke.is_some() 
            || partial.opacity.is_some();
        if let Some(pos) = partial.pos {
            self.pos.set(pos);
        }
        if let Some(height) = partial.height {
            self.height.set(height);
        }
        if let Some(width) = partial.width {
            self.width.set(width);
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