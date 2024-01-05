use crate::prelude::*;

#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SVGRectangle {
    pub id: String,
    pub pos: Vec2,
    pub height: i32,
    pub width: i32,
    pub fill: Color,
    pub stroke_width: i32,
    pub stroke: Color,
    pub opacity: f32
}

#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PartialSVGRectangle {
    #[tsify(optional)]
    pub pos: Option<Vec2>,
    #[tsify(optional)]
    pub height: Option<i32>,
    #[tsify(optional)]
    pub width: Option<i32>,
    #[tsify(optional)]
    pub fill: Option<Color>,
    #[tsify(optional)]
    pub stroke_width: Option<i32>,
    #[tsify(optional)]
    pub stroke: Option<Color>,
    #[tsify(optional)]
    pub opacity: Option<f32>
}

impl partially::Partial for SVGRectangle {
    type Item = PartialSVGRectangle;
    fn apply_some(&mut self, partial: Self::Item) -> bool {
        let will_apply_some = partial.pos.is_some() || partial.height.is_some()
            || partial.width.is_some() || partial.fill.is_some()
            || partial.stroke_width.is_some() || partial.stroke.is_some() || partial.opacity.is_some();
        if let Some(pos) = partial.pos {
            self.pos = pos.into();
        }
        if let Some(height) = partial.height {
            self.height = height.into();
        }
        if let Some(width) = partial.width {
            self.width = width.into();
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

impl SVGRectangle {

    pub(crate) fn default()  -> Self {
        let id = gen_str_id();
        SVGRectangle { 
            id, 
            pos: Vec2 { x: 0, y: 0 }, 
            height: 5, 
            width: 10, 
            fill: Color::white(), 
            stroke_width: 2, 
            stroke: Color::black(),
            opacity: 1.0
        }
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.pos = Vec2 { x, y };
    }

    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }

    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    pub fn set_fill(&mut self, red: i32, green: i32, blue: i32, opacity: f32) {
        self.fill = Color(red, green, blue, opacity);
    }

    pub fn set_stroke_width(&mut self, stroke_width: i32) {
        self.stroke_width = stroke_width;
    }

    pub fn set_stroke(&mut self, red: i32, green: i32, blue: i32, opacity: f32) {
        self.stroke = Color(red, green, blue, opacity);
    }
}