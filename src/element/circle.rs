use crate::prelude::*;


#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SVGCircle {
    pub id: String,
    pub pos: Vec2,
    pub radius: i32,
    pub fill: Color,
    pub stroke_width: i32,
    pub stroke: Color,
    pub opacity: f32,
}

#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PartialSVGCircle {
    #[tsify(optional)]
    pub pos: Option<Vec2>,
    #[tsify(optional)]
    pub radius: Option<i32>,
    #[tsify(optional)]
    pub fill: Option<Color>,
    #[tsify(optional)]
    pub stroke_width: Option<i32>,
    #[tsify(optional)]
    pub stroke: Option<Color>,
    #[tsify(optional)]
    pub opacity: Option<f32>
}

impl PartialSVGCircle {
    pub fn empty() -> Self {
        PartialSVGCircle { 
            pos: None, 
            radius: None, 
            fill: None, 
            stroke_width: None, 
            stroke: None, 
            opacity: None
        }
    }
}


impl partially::Partial for SVGCircle {
    type Item = PartialSVGCircle;
    fn apply_some(&mut self, partial: Self::Item) -> bool {
        let will_apply_some = partial.pos.is_some() || partial.radius.is_some()
            || partial.fill.is_some() || partial.stroke_width.is_some()
            || partial.stroke.is_some() || partial.opacity.is_some();
        if let Some(pos) = partial.pos {
            self.pos = pos.into();
        }
        if let Some(radius) = partial.radius {
            self.radius = radius.into();
        }
        if let Some(fill) = partial.fill {
            self.fill = fill.into();
        }
        if let Some(stroke_width) = partial.stroke_width {
            self.stroke_width = stroke_width.into();
        }
        if let Some(stroke) = partial.stroke {
            self.stroke = stroke.into();
        }
        if let Some(opacity) = partial.opacity {
            self.opacity = opacity.into();
        }
        will_apply_some
    }
}

impl SVGCircle {

    pub (crate) fn default() -> Self {
        let id = gen_str_id();
        return SVGCircle { id, 
            pos: Vec2 { x: 0, y: 0 }, 
            radius: 10, 
            fill: Color::white(), 
            stroke_width: 2, 
            stroke: Color::black(),
            opacity: 1.0
        };
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.pos = Vec2 { x, y };
    }

    pub fn set_radius(&mut self, radius: i32) {
        self.radius = radius;
    }

    pub fn set_fill(&mut self, fill: Color) {
        self.fill = fill;
    }

    pub fn set_stroke_width(&mut self, stroke_width: i32) {
        self.stroke_width = stroke_width;
    }

    pub fn set_stroke(&mut self, stroke: Color) {
        self.stroke = stroke
    }

}