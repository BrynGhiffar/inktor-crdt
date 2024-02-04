use crate::prelude::*;

#[derive(Serialize, Deserialize, Tsify, Clone, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SVGGroup {
    pub id: NodeID,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: Option<i32>,
    pub opacity: Option<f32>,
    pub children: Vec<SVGObject>
}

#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PartialSVGGroup {
    #[tsify(optional)]
    pub fill: Option<Option<Color>>,
    #[tsify(optional)]
    pub stroke: Option<Option<Color>>,
    #[tsify(optional)]
    pub stroke_width: Option<Option<i32>>,
    #[tsify(optional)]
    pub opacity: Option<Option<f32>>
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
        if let Some(opacity) = partial.opacity {
            self.opacity = opacity.into();
        }
        will_apply_some
    }
}

impl PartialSVGGroup {
    pub fn empty() -> Self {
        PartialSVGGroup {
            fill: None,
            stroke: None,
            stroke_width: None,
            opacity: None
        }
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
            opacity: None,
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