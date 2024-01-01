use crate::prelude::*;

#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SVGGroup {
    pub(crate) id: String,
    pub(crate) fill: Option<Color>,
    pub(crate) stroke: Option<Color>,
    pub(crate) stroke_width: Option<i32>,
    pub(crate) children: Vec<SVGObject>
}

#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PartialSVGGroup {
    #[tsify(optional)]
    fill: Option<Option<Color>>,
    #[tsify(optional)]
    stroke: Option<Option<Color>>,
    #[tsify(optional)]
    stroke_width: Option<Option<i32>>,
}

impl partially::Partial for SVGGroup {
    type Item = PartialSVGGroup;
    fn apply_some(&mut self, partial: Self::Item) -> bool {
        let will_apply_some = partial.fill.is_some() || partial.stroke.is_some()
            || partial.stroke_width.is_some();
        if let Some(fill) = partial.fill {
            self.fill = fill.into();
        }
        if let Some(stroke) = partial.stroke {
            self.stroke = stroke.into();
        }
        if let Some(stroke_width) = partial.stroke_width {
            self.stroke_width = stroke_width.into();
        }
        will_apply_some
    }
}

impl SVGGroup {

    pub fn default() -> Self {
        let id = gen_str_id();
        return SVGGroup {
            id,
            fill: None, 
            stroke: None,
            stroke_width: None,
            children: Vec::new()
        }
    }

    pub fn set_fill(&mut self, red: i32, green: i32, blue: i32, opacity: f32) {
        self.fill = Some(Color(red, green, blue, opacity));
    }

    pub fn set_stroke(&mut self, red: i32, green: i32, blue: i32, opacity: f32) {
        self.stroke = Some(Color(red, green, blue, opacity));
    }

    pub fn set_stroke_width(&mut self, width: i32) {
        self.stroke_width = Some(width);
    }
}