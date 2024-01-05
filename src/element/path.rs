use crate::prelude::*;

#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum SVGPathCommand {
    #[serde(rename = "START")]
    Start { id: String, pos: Vec2 }, // M
    #[serde(rename = "LINE")]
    Line { id: String, pos: Vec2 }, // L
    #[serde(rename = "CLOSE")]
    Close { id: String }, // Z
    #[serde(rename = "BEZIER")]
    Bezier { id: String, handle1: Vec2, handle2: Vec2, pos: Vec2 }, // C
    #[serde(rename = "BEZIER_REFLECT")]
    BezierReflect { id: String, handle: Vec2, pos: Vec2 }, // S
    #[serde(rename = "BEZIER_QUAD")]
    BezierQuad { id: String, handle: Vec2, pos: Vec2 }, // Q
    #[serde(rename = "BEZIER_QUAD_REFLECT")]
    BezierQuadReflect { id: String, pos: Vec2 }, // T
}

impl SVGPathCommand {
    pub fn get_id<'a>(&'a self) -> &'a str {
        match self {
            Self::Start { id, .. } => id,
            Self::Line { id, .. } => id,
            Self::Close { id } => id,
            Self::Bezier { id, .. } => id,
            Self::BezierReflect { id, .. } => id,
            Self::BezierQuad { id, .. } => id,
            Self::BezierQuadReflect { id, .. } => id
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum SVGPathCommandType {
    START = 0,
    LINE = 1,
    CLOSE = 2,
    BEZIER = 3,
    BEZIER_REFLECT = 4,
    BEZIER_QUAD = 5,
    BEZIER_QUAD_REFLECT = 6
}


#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SVGPath {
    pub(crate) id: String,
    pub(crate) fill: Color,
    pub(crate) stroke_width: i32,
    pub(crate) stroke: Color,
    pub(crate) points: Vec<SVGPathCommand>,
    pub(crate) opacity: f32
}

#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PartialSVGPath {
    #[tsify(optional)]
    fill: Option<Color>,
    #[tsify(optional)]
    stroke_width: Option<i32>,
    #[tsify(optional)]
    stroke: Option<Color>,
    #[tsify(optional)]
    opacity: Option<f32>
}

impl partially::Partial for SVGPath {
    type Item = PartialSVGPath;
    fn apply_some(&mut self, partial: Self::Item) -> bool {
        let will_apply_some = partial.fill.is_some()
            || partial.stroke_width.is_some() || partial.stroke.is_some();
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

impl SVGPath {
    pub(crate) fn default() -> Self {
        SVGPath {
            id: gen_str_id(),
            fill: Color::white(), 
            stroke_width: 2, 
            stroke: Color::black(), 
            points: vec![],
            opacity: 1.0
        }
    }

    pub(crate) fn find_point_mut<'a>(&'a mut self, point_id: &'a str) -> Option<&'a mut SVGPathCommand> {
        for point in self.points.iter_mut() {
            if point.get_id().eq(point_id) {
                return Some(point);
            }
        }
        return None;
    }
}