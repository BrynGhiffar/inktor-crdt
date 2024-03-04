use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct LWWSVGPath {
    pub id: LWWReg<NodeID>,
    pub fill: LWWReg<Color>,
    pub stroke_width: LWWReg<i32>,
    pub stroke: LWWReg<Color>,
    pub points: LWWReg<Vec<SVGPathCommand>>,
    pub opacity: LWWReg<f32>
}

impl LWWSVGPath {
    pub fn value(&self) -> SVGPath {
        SVGPath { 
            id: self.id.value().clone(), 
            fill: self.fill.value().clone(), 
            stroke_width: self.stroke_width.value().clone(), 
            stroke: self.stroke.value().clone(), 
            points: self.points.value().clone(), 
            opacity: self.opacity.value().clone()
        }
    }

    pub fn set_points(&mut self, points: Vec<SVGPathCommand>) {
        self.points.set(points);
    }
}

impl From<SVGPath> for LWWSVGPath {
    fn from(SVGPath { 
        id, 
        fill, 
        stroke_width, 
        stroke, 
        points, 
        opacity 
    }: SVGPath) -> Self {
        Self { 
            id: LWWReg::new(id), 
            fill: LWWReg::new(fill), 
            stroke_width: LWWReg::new(stroke_width), 
            stroke: LWWReg::new(stroke), 
            points: LWWReg::new(points), 
            opacity: LWWReg::new(opacity) 
        }
    }
}

impl Mergeable for LWWSVGPath {
    fn merge(&self, other: &Self) -> Self {
        Self { 
            id: self.id.merge(&other.id), 
            fill: self.fill.merge(&other.fill), 
            stroke_width: self.stroke_width.merge(&other.stroke_width), 
            stroke: self.stroke.merge(&other.stroke), 
            points: self.points.merge(&other.points), 
            opacity: self.opacity.merge(&other.opacity) 
        }
    }
}

impl partially::Partial for LWWSVGPath {
    type Item = PartialSVGPath;

    fn apply_some(&mut self, partial: Self::Item) -> bool {
        let will_apply_some = partial.fill.is_some()
            || partial.stroke_width.is_some() 
            || partial.stroke.is_some()
            || partial.opacity.is_some()
            || partial.points.is_some();
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
        if let Some(mut points) = partial.points {
            self.points.set(points.drain(..)
                .map(|it| SVGPathCommand::from_partial(it))
                .collect());
        }
        will_apply_some
    }
}