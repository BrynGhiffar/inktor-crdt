use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct LWWSVGGroup {
    pub id: LWWReg<NodeID>,
    pub fill: LWWReg<Option<Color>>,
    pub stroke: LWWReg<Option<Color>>,
    pub stroke_width: LWWReg<Option<i32>>,
    pub opacity: LWWReg<Option<f32>>,
    pub children: LWWReg<Vec<SVGObject>>
}

impl LWWSVGGroup {
    pub fn value(&self) -> SVGGroup {
        SVGGroup { 
            id: self.id.value().clone(), 
            fill: self.fill.value().clone(), 
            stroke: self.stroke.value().clone(), 
            stroke_width: self.stroke_width.value().clone(), 
            opacity: self.opacity.value().clone(),
            children: self.children.value().clone() 
        }
    }
}

impl From<SVGGroup> for LWWSVGGroup {
    fn from(SVGGroup { 
        id, 
        fill, 
        stroke, 
        stroke_width, 
        opacity,
        children 
    }: SVGGroup) -> Self {
        Self { 
            id: LWWReg::new(id), 
            fill: LWWReg::new(fill), 
            stroke: LWWReg::new(stroke), 
            stroke_width: LWWReg::new(stroke_width), 
            opacity: LWWReg::new(opacity),
            children: LWWReg::new(children)
        }
    }
}

impl Mergeable for LWWSVGGroup {
    fn merge(&self, other: &Self) -> Self {
        Self { 
            id: self.id.merge(&other.id), 
            fill: self.fill.merge(&other.fill), 
            stroke: self.stroke.merge(&other.stroke), 
            stroke_width: self.stroke_width.merge(&other.stroke_width), 
            opacity: self.opacity.merge(&other.opacity),
            children: self.children.merge(&other.children)
        }
    }
}

impl partially::Partial for LWWSVGGroup {
    type Item = PartialSVGGroup;

    fn apply_some(&mut self, partial: Self::Item) -> bool {
        let will_apply_some = partial.fill.is_some()
            || partial.stroke.is_some()
            || partial.stroke_width.is_some()
            || partial.opacity.is_some();
        if let Some(fill) = partial.fill {
            match fill {
                JSNullable::Some { item: val } => {
                    self.fill.set(Some(val));
                },
                JSNullable::None => {
                    self.fill.set(None);
                }
            }
        }
        if let Some(stroke) = partial.stroke {
            match stroke {
                JSNullable::Some { item } => {
                    self.stroke.set(Some(item));
                },
                JSNullable::None => {
                    self.stroke.set(None);
                }
            };
        }
        if let Some(stroke_width) = partial.stroke_width {
            match stroke_width {
                JSNullable::Some { item } => {
                    self.stroke_width.set(Some(item));
                },
                JSNullable::None => {
                    self.stroke_width.set(None);
                }
            };
        }
        if let Some(opacity) = partial.opacity {
            match opacity {
                JSNullable::Some { item } => {
                    self.opacity.set(Some(item));
                },
                JSNullable::None => {
                    self.opacity.set(None);
                }
            };
        }
        will_apply_some
    }
}

