use crate::prelude::*;

#[derive(Serialize, Deserialize, Tsify, Clone, Debug)]
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
    #[serde(rename = "BEZIER_QUAD")]
    BezierQuad { id: String, handle: Vec2, pos: Vec2 }, // Q
}

#[derive(Serialize, Deserialize, Tsify, Clone, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum PartialSVGPathCommand {
    #[serde(rename = "START")]
    Start { 
        #[tsify(optional)]
        pos: Option<Vec2>
    }, // M
    #[serde(rename = "LINE")]
    Line { 
        #[tsify(optional)]
        pos: Option<Vec2>
    }, // L
    #[serde(rename = "CLOSE")]
    Close { }, // Z
    #[serde(rename = "BEZIER")]
    Bezier {

        #[tsify(optional)]
        handle1: Option<Vec2>, 

        #[tsify(optional)]
        handle2: Option<Vec2>, 

        #[tsify(optional)]
        pos: Option<Vec2>
    }, // C
    #[serde(rename = "BEZIER_QUAD")]
    BezierQuad { 
        #[tsify(optional)]
        handle: Option<Vec2>, 

        #[tsify(optional)]
        pos: Option<Vec2>
    }, // Q
}

impl SVGPathCommand {
    pub fn get_id<'a>(&'a self) -> &'a str {
        match self {
            Self::Start { id, .. } => id,
            Self::Line { id, .. } => id,
            Self::Close { id } => id,
            Self::Bezier { id, .. } => id,
            Self::BezierQuad { id, .. } => id,
        }
    }

    pub fn from_partial(command: PartialSVGPathCommand) -> SVGPathCommand {
        let id = gen_str_id();
        match command {
            PartialSVGPathCommand::Start { pos } => SVGPathCommand::Start { 
                id, 
                pos: pos.unwrap_or(Vec2 { x: 0, y: 0 })
            },
            PartialSVGPathCommand::Line { pos } => SVGPathCommand::Line { 
                id, 
                pos: pos.unwrap_or(Vec2 { x: 0, y: 0 })
            },
            PartialSVGPathCommand::Close {  } => SVGPathCommand::Close { 
                id
            },
            PartialSVGPathCommand::Bezier { handle1, handle2, pos } => SVGPathCommand::Bezier { 
                id, 
                handle1: handle1.unwrap_or(Vec2 { x: 5, y: 5 }), 
                handle2: handle2.unwrap_or(Vec2 { x: 5, y: -5 }), 
                pos: pos.unwrap_or(Vec2 { x: 0, y: 0 })
            },
            PartialSVGPathCommand::BezierQuad { handle, pos } => SVGPathCommand::BezierQuad { 
                id, 
                handle: handle.unwrap_or(Vec2 { x: 5, y: 5 }), 
                pos: pos.unwrap_or(Vec2 { x: 0, y: 0 })
            },
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
    BEZIER_QUAD = 5,
}


#[derive(Serialize, Deserialize, Tsify, Clone, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SVGPath {
    pub(crate) id: NodeID,
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
    pub fill: Option<Color>,
    #[tsify(optional)]
    pub stroke_width: Option<i32>,
    #[tsify(optional)]
    pub stroke: Option<Color>,
    #[tsify(optional)]
    pub opacity: Option<f32>,
    #[tsify(optional)]
    pub points: Option<Vec<PartialSVGPathCommand>>
}

impl PartialSVGPath {
    pub fn empty() -> Self {
        Self { 
            fill: None, 
            stroke_width: None, 
            stroke: None, 
            opacity: None, 
            points: None
        }
    }
}

impl partially::Partial for SVGPath {
    type Item = PartialSVGPath;
    fn apply_some(&mut self, partial: Self::Item) -> bool {
        let will_apply_some = partial.fill.is_some()
            || partial.stroke_width.is_some()
            || partial.stroke.is_some()
            || partial.points.is_some();
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
        if let Some(mut points) = partial.points {
            self.points = points.drain(..)
                .map(|it| SVGPathCommand::from_partial(it)).collect();
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