#![allow(nonstandard_style)]

use partially::Partial;
use serde::Deserialize;
use serde::Serialize;
use unique_id::Generator;
use wasm_bindgen::prelude::*;
use tsify::Tsify;
use unique_id::string::StringGenerator;

fn gen_str_id() -> String {
    return StringGenerator::default().next_id();
}

#[derive(Tsify, Serialize, Deserialize, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Color(i32 /* red (0 - 255) */, i32 /* green (0 - 255) */, i32 /* blue (0 - 255) */, f32 /* (0 - 100) */);

impl Color {
    fn white() -> Color {
        Color(255, 255, 255, 1.0)
    }

    fn black() -> Color {
        Color(0, 0, 0, 1.0)
    }
}

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
    fn get_id<'a>(&'a self) -> &'a str {
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

#[derive(Copy, Clone)]
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
pub struct SVGGroup {
    id: String,
    fill: Option<Color>,
    stroke: Option<Color>,
    stroke_width: Option<i32>,
    children: Vec<SVGObject>
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
    #[tsify(optional)]
    children: Option<Vec<SVGObject>>,
}

impl partially::Partial for SVGGroup {
    type Item = PartialSVGGroup;
    fn apply_some(&mut self, partial: Self::Item) -> bool {
        let will_apply_some = partial.fill.is_some() || partial.stroke.is_some()
            || partial.stroke_width.is_some() || partial.children.is_some();
        if let Some(fill) = partial.fill {
            self.fill = fill.into();
        }
        if let Some(stroke) = partial.stroke {
            self.stroke = stroke.into();
        }
        if let Some(stroke_width) = partial.stroke_width {
            self.stroke_width = stroke_width.into();
        }
        if let Some(children) = partial.children {
            self.children = children.into();
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

#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SVGCircle {
    id: String,
    pos: Vec2,
    radius: i32,
    fill: Color,
    stroke_width: i32,
    stroke: Color,
    opacity: f32,
}

#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PartialSVGCircle {
    #[tsify(optional)]
    pos: Option<Vec2>,
    #[tsify(optional)]
    radius: Option<i32>,
    #[tsify(optional)]
    fill: Option<Color>,
    #[tsify(optional)]
    stroke_width: Option<i32>,
    #[tsify(optional)]
    stroke: Option<Color>,
    #[tsify(optional)]
    opacity: Option<f32>
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

    fn default() -> Self {
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

#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SVGRectangle {
    id: String,
    pos: Vec2,
    height: i32,
    width: i32,
    fill: Color,
    stroke_width: i32,
    stroke: Color,
    opacity: f32
}

#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PartialSVGRectangle {
    #[tsify(optional)]
    pos: Option<Vec2>,
    #[tsify(optional)]
    height: Option<i32>,
    #[tsify(optional)]
    width: Option<i32>,
    #[tsify(optional)]
    fill: Option<Color>,
    #[tsify(optional)]
    stroke_width: Option<i32>,
    #[tsify(optional)]
    stroke: Option<Color>,
    #[tsify(optional)]
    opacity: Option<f32>
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

    fn default()  -> Self {
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

#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SVGPath {
    id: String,
    fill: Color,
    stroke_width: i32,
    stroke: Color,
    points: Vec<SVGPathCommand>,
    opacity: f32
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
    points: Option<Vec<SVGPathCommand>>,
    #[tsify(optional)]
    opacity: Option<f32>
}

impl partially::Partial for SVGPath {
    type Item = PartialSVGPath;
    fn apply_some(&mut self, partial: Self::Item) -> bool {
        let will_apply_some = partial.fill.is_some()
            || partial.stroke_width.is_some() || partial.stroke.is_some()
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
        if let Some(points) = partial.points {
            self.points = points.into();
        }
        if let Some(opacity) = partial.opacity {
            self.opacity = opacity.into();
        }
        will_apply_some
    }
}

impl SVGPath {
    fn default() -> Self {
        SVGPath {
            id: gen_str_id(),
            fill: Color::white(), 
            stroke_width: 2, 
            stroke: Color::black(), 
            points: vec![],
            opacity: 1.0
        }
    }

    fn find_point_mut<'a>(&'a mut self, point_id: &'a str) -> Option<&'a mut SVGPathCommand> {
        for point in self.points.iter_mut() {
            if point.get_id().eq(point_id) {
                return Some(point);
            }
        }
        return None;
    }

    // fn find_point<'a>(&'a )
}

#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum SVGObject {
    #[serde(rename = "CIRCLE")]
    Circle(SVGCircle),
    #[serde(rename = "RECTANGLE")]
    Rectangle(SVGRectangle),
    #[serde(rename = "PATH")]
    Path(SVGPath),
    #[serde(rename = "GROUP")]
    Group(SVGGroup)
}

#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SVGDocInner {
    children: Vec<SVGObject>
}

#[wasm_bindgen]
pub struct SVGDoc {
    inner: SVGDocInner,

}

impl SVGDoc {
    fn find_group_mut_aux<'a>(group: &'a mut SVGGroup, group_id: &'a str) -> Option<&'a mut SVGGroup> {
        for children in group.children.iter_mut() {
            let child = match children {
                SVGObject::Group(group) => group,
                _ => continue,
            };
            if child.id == group_id { return Some(child); }
            if let Some(group) = Self::find_group_mut_aux(child, group_id) {
                return Some(group);
            }
        }
        return None;
    }

    fn find_group_mut<'a>(&'a mut self, group_id: &'a str) -> Option<&'a mut SVGGroup> {
        for children in self.inner.children.iter_mut() {
            let child = match children {
                SVGObject::Group(group) => group,
                _ => continue,
            };
            if child.id.eq(group_id) {
                return Some(child);
            }
            if let Some(group) = Self::find_group_mut_aux(child, group_id) {
                return Some(group);
            }
        }
        return None;
    }

    fn find_group_aux<'a>(group: &'a SVGGroup, group_id: &'a str) -> Option<&'a SVGGroup> {
        for children in group.children.iter() {
            let SVGObject::Group(group) = children else { continue; };
            if group.id == group_id { return Some(group) }
            let Some(group) = Self::find_group_aux(group, group_id) else { continue; };
            return Some(group);

        }
        return None;
    }

    fn find_group<'a>(&'a self, group_id: &'a str) -> Option<&'a SVGGroup> {
        for children in self.inner.children.iter() {
            let SVGObject::Group(group) = children else { continue; };
            if group.id == group_id { return Some(group); };
            let Some(group) = Self::find_group_aux(group, group_id) else { continue; };
            return Some(group);
        }
        return None;
    }


    fn find_circle_mut_aux<'a>(group: &'a mut SVGGroup, circle_id: &'a str) -> Option<&'a mut SVGCircle> {
        for children in group.children.iter_mut() {
            if let SVGObject::Circle(circle) = children {
                if circle.id.eq(circle_id) { return Some(circle); }
            } else if let SVGObject::Group(group) = children {
                let Some(circle) = Self::find_circle_mut_aux(group, circle_id) else { continue; };
                if circle.id.eq(circle_id) { return Some(circle); }

            }
        }
        return None;
    }

    fn find_circle_mut<'a>(&'a mut self, circle_id: &'a str) -> Option<&'a mut SVGCircle> {
        for children in self.inner.children.iter_mut() {
            if let SVGObject::Circle(circle) = children {
                if circle.id.eq(circle_id) { return Some(circle); }
            } else if let SVGObject::Group(group) = children {
                let Some(circle) = Self::find_circle_mut_aux(group, circle_id) else { continue; };
                return Some(circle);
            }
        }
        return None;
    }

    fn find_circle_aux<'a>(group: &'a SVGGroup, circle_id: &'a str) -> Option<&'a SVGCircle> {
        for children in group.children.iter() {
            if let SVGObject::Circle(circle) = children {
                if circle.id == circle_id { return Some(circle); };
            } else if let SVGObject::Group(group) = children {
                let Some(circle) = Self::find_circle_aux(group, circle_id) else { continue; };
                return Some(circle);
            }
        }
        return None;
    }

    fn find_circle<'a>(&'a self, circle_id: &'a str) -> Option<&'a SVGCircle> {
        for children in self.inner.children.iter() {
            if let SVGObject::Circle(circle) = children {
                if circle.id == circle_id { return Some(circle); }
            } else if let SVGObject::Group(group) = children {
                let Some(circle) = Self::find_circle_aux(group, circle_id) else { continue; };
                return Some(circle);
            }
        }
        return None;
    }

    fn find_rectangle_mut_aux<'a>(group: &'a mut SVGGroup, rectangle_id: &'a str) -> Option<&'a mut SVGRectangle> {
        for children in group.children.iter_mut() {
            if let SVGObject::Rectangle(rectangle) = children {
                if rectangle.id.eq(rectangle_id) { return Some(rectangle); }
            } else if let SVGObject::Group(group) = children {
                let Some(rectangle) = Self::find_rectangle_mut_aux(group, rectangle_id) else { continue; };
                return Some(rectangle);
            }
        }
        return None;
    }

    fn find_rectangle_mut<'a>(&'a mut self, rectangle_id: &'a str) -> Option<&'a mut SVGRectangle> {
        for children in self.inner.children.iter_mut() {
            if let SVGObject::Rectangle(rectangle) = children {
                if rectangle.id.eq(rectangle_id) { return Some(rectangle); }
            } else if let SVGObject::Group(group) = children {
                let Some(rectangle) = Self::find_rectangle_mut_aux(group, rectangle_id) else { continue; };
                return Some(rectangle);
            }
        }
        return None;
    }

    fn find_rectangle_aux<'a>(group:&'a SVGGroup, rectangle_id: &'a str) -> Option<&'a SVGRectangle> {
        for child in group.children.iter() {
            if let SVGObject::Rectangle(rectangle) = child {
                if rectangle.id == rectangle_id { return Some(rectangle); }
            } else if let SVGObject::Group(group) = child {
                let Some(rectangle) = Self::find_rectangle_aux(group, rectangle_id) else { continue; };
                return Some(rectangle);
            }
        }
        return None;
    }

    fn find_rectangle<'a>(&'a self, rectangle_id: &'a str) -> Option<&'a SVGRectangle> {
        for child in self.inner.children.iter() {
            if let SVGObject::Rectangle(rect) = child {
                if rect.id == rectangle_id { return Some(rect); }
            } else if let SVGObject::Group(group) = child {
                let Some(rectangle) = Self::find_rectangle_aux(group, rectangle_id) else { continue; };
                return Some(rectangle);
            }
        }
        return None;
    }

    fn find_path_mut_aux<'a>(group: &'a mut SVGGroup, path_id: &'a str) -> Option<&'a mut SVGPath> {
        for children in group.children.iter_mut() {
            if let SVGObject::Path(path) = children {
                if path.id.eq(path_id) { return Some(path); }
            } else if let SVGObject::Group(group) = children {
                let Some(path) = Self::find_path_mut_aux(group, path_id) else { continue; };
                return Some(path);
            }
        }
        return None;
    }

    fn find_path_mut<'a>(&'a mut self, path_id: &'a str) -> Option<&'a mut SVGPath> {
        for children in self.inner.children.iter_mut() {
            if let SVGObject::Path(path) = children {
                if path.id.eq(path_id) { return Some(path); };
            } else if let SVGObject::Group(group) = children {
                let Some(path) = Self::find_path_mut_aux(group, path_id) else { continue; };
                return Some(path);
            }
        }
        return None;
    }

    fn find_path_aux<'a>(group: &'a SVGGroup, path_id: &'a str) -> Option<&'a SVGPath> {
        for child in group.children.iter() {
            if let SVGObject::Path(path) = child {
                if path.id.eq(path_id) { return Some(path); };
            } else if let SVGObject::Group(group) = child {
                let Some(path) = Self::find_path_aux(group, path_id) else { continue; };
                return Some(path);
            }
        }
        return None;
    }

    fn find_path<'a>(&'a self, path_id: &'a str) -> Option<&'a SVGPath> {
        for child in self.inner.children.iter() {
            if let SVGObject::Path(path) = child {
                if path.id.eq(path_id) { return Some(path); };
            } else if let SVGObject::Group(group) = child {
                let Some(path) = Self::find_path_aux(group, path_id) else { continue; };
                return Some(path);
            }
        }
        return None;
    }

}

#[wasm_bindgen]
impl SVGDoc {
    pub fn new() -> Self {
        return SVGDoc { inner: SVGDocInner { children: Vec::new() } };
    }

    pub fn get_group(&self, group_id: String) -> Option<SVGGroup> {
        let group = self.find_group(&group_id);
        return group.map(|g| g.clone());
    }

    pub fn add_group(&mut self, group_id: Option<String>, partial_group: PartialSVGGroup) {
        let mut new_group = SVGGroup::default();
        new_group.apply_some(partial_group);
        if let Some(group_id) = group_id {
            let Some(group) = self.find_group_mut(&group_id) else { return; };
            group.children.push(SVGObject::Group(new_group));
            return;
        }
        self.inner.children.push(SVGObject::Group(new_group));
    }

    pub fn get_circle(&self, circle_id: String) -> Option<SVGCircle>{
        return self.find_circle(&circle_id).map(|c| c.clone());
    }

    pub fn add_circle(&mut self, group_id: Option<String>, partial_circle: PartialSVGCircle) {
        let mut circle = SVGCircle::default();
        circle.apply_some(partial_circle);
        if let Some(group_id) = group_id {
            let Some(group) = self.find_group_mut(&group_id) else { return; };
            group.children.push(SVGObject::Circle(circle));
            return;
        }
        self.inner.children.push(SVGObject::Circle(circle));
    }

    pub fn edit_circle(&mut self, circle_id: String, edits: PartialSVGCircle) {
        let Some(circle) = self.find_circle_mut(&circle_id) else { return; };
        circle.apply_some(edits);
    }

    pub fn get_rectangle(&self, rectangle_id: String) -> Option<SVGRectangle> {
        let rectangle = self.find_rectangle(&rectangle_id);
        return rectangle.map(|r| r.clone());
    }

    pub fn add_rectangle(&mut self, group_id: Option<String>, partial_rectangle: PartialSVGRectangle) {
        let mut rectangle = SVGRectangle::default();
        rectangle.apply_some(partial_rectangle);
        if let Some(group_id) = group_id {
            let Some(group) = self.find_group_mut(&group_id) else { return; };
            group.children.push(SVGObject::Rectangle(rectangle));
            return;
        }
        self.inner.children.push(SVGObject::Rectangle(rectangle));
    }

    pub fn edit_rectangle(&mut self, rectangle_id: String, edits: PartialSVGRectangle) {
        let Some(rectangle) = self.find_rectangle_mut(&rectangle_id) else { return; };
        rectangle.apply_some(edits);
    }

    pub fn get_path(&self, path_id: String) -> Option<SVGPath> {
        let path = self.find_path(&path_id);
        return path.map(|p| p.clone());
    }

    pub fn add_path(&mut self, group_id: Option<String>, partial_path: PartialSVGPath) {
        let mut path = SVGPath::default();
        path.apply_some(partial_path);
        if let Some(group_id) = group_id {
            let Some(group) = self.find_group_mut(&group_id) else { return; };
            group.children.push(SVGObject::Path(path));
            return;
        }
        self.inner.children.push(SVGObject::Path(path));
    }

    pub fn edit_path(&mut self, path_id: String, partial_path: PartialSVGPath) {
        let Some(path) = self.find_path_mut(&path_id) else { return; };
        path.apply_some(partial_path);
    }

    pub fn edit_path_point_type(
        &mut self, 
        path_id: String, 
        point_id: String, 
        command_type: SVGPathCommandType, 
    ) {
        let Some(path) = self.find_path_mut(&path_id) else { return; };
        let Some(point) = path.find_point_mut(&point_id) else { return; };
        let pos = Vec2 { x: 0, y: 0 };
        let command = match command_type {
            SVGPathCommandType::START => SVGPathCommand::Start { id: gen_str_id(), pos },
            SVGPathCommandType::LINE => SVGPathCommand::Line { id: gen_str_id(), pos },
            SVGPathCommandType::CLOSE => SVGPathCommand::Close { id: gen_str_id() },
            SVGPathCommandType::BEZIER => {
                let handle1 = Vec2 { x: pos.x + 20, y: pos.y + 20 };
                let handle2 = Vec2 { x: pos.x + 20, y: pos.y - 20 };
                SVGPathCommand::Bezier { id: gen_str_id(), handle1, handle2, pos }
            },
            SVGPathCommandType::BEZIER_REFLECT => {
                let handle = Vec2 { x: pos.x, y: pos.y + 20 };
                SVGPathCommand::BezierReflect { id: gen_str_id(), handle, pos }
            },
            SVGPathCommandType::BEZIER_QUAD => {
                let handle = Vec2 { x: pos.x, y: pos.y + 20 };
                SVGPathCommand::BezierQuad { id: gen_str_id(), handle, pos }
            },
            SVGPathCommandType::BEZIER_QUAD_REFLECT => {
                SVGPathCommand::BezierQuadReflect { id: gen_str_id(), pos }
            }
        };
        *point = command;
    }

    pub fn edit_path_point_pos(&mut self, path_id: String, point_id: String, new_pos: Vec2) {
        let Some(path) = self.find_path_mut(&path_id) else { return; };
        let Some(point) = path.find_point_mut(&point_id) else { return; };
        match point {
            SVGPathCommand::Start { pos, .. } => {
                *pos = new_pos;
            },
            SVGPathCommand::Line { pos, .. } => {
                *pos = new_pos;
            },
            SVGPathCommand::Bezier { pos, .. } => {
                *pos = new_pos;
            },
            SVGPathCommand::BezierQuad { pos, .. } => {
                *pos = new_pos;
            },
            SVGPathCommand::BezierQuadReflect { pos, .. } => {
                *pos = new_pos;
            },
            _ => ()
        };
    }

    pub fn edit_path_point_handle1(&mut self, path_id: String, point_id: String, new_handle1: Vec2) {
        let Some(path) = self.find_path_mut(&path_id) else { return; };
        let Some(point) = path.find_point_mut(&point_id) else { return; };
        match point {
            SVGPathCommand::Bezier { handle1, .. } => {
                *handle1 = new_handle1;
            },
            SVGPathCommand::BezierReflect { handle, .. } => {
                *handle = new_handle1;
            },
            SVGPathCommand::BezierQuad { handle, .. } => {
                *handle = new_handle1;
            },
            _ => ()
        }
    }

    pub fn edit_path_point_handle2(&mut self, path_id: String, point_id: String, new_handle2: Vec2) {
        let Some(path) = self.find_path_mut(&path_id) else { return; };
        let Some(point) = path.find_point_mut(&point_id) else { return; };
        match point {
            SVGPathCommand::Bezier { handle2, .. } => {
                *handle2 = new_handle2;
            },
            _ => ()
        };
    }

    pub fn add_point_to_path(&mut self, path_id: String, command: SVGPathCommandType, pos: Vec2) {
        let Some(path) = self.find_path_mut(&path_id) else { return; };
        match command {
            SVGPathCommandType::START => {
                path.points.push(SVGPathCommand::Start { id: gen_str_id(), pos });
            },
            SVGPathCommandType::LINE => {
                path.points.push(SVGPathCommand::Line { id: gen_str_id(), pos });
            },
            SVGPathCommandType::CLOSE => {
                path.points.push(SVGPathCommand::Close { id: gen_str_id() });
            },
            SVGPathCommandType::BEZIER => {
                let handle1 = Vec2 { x: pos.x + 20, y: pos.y + 20 };
                let handle2 = Vec2 { x: pos.x + 20, y: pos.y - 20 };
                path.points.push(SVGPathCommand::Bezier { id: gen_str_id(), handle1, handle2, pos });
            },
            SVGPathCommandType::BEZIER_REFLECT => {
                let handle = Vec2 { x: pos.x, y: pos.y + 20 };
                path.points.push(SVGPathCommand::BezierReflect { id: gen_str_id(), handle, pos });
            },
            SVGPathCommandType::BEZIER_QUAD => {
                let handle = Vec2 { x: pos.x, y: pos.y + 20 };
                path.points.push(SVGPathCommand::BezierQuad { id: gen_str_id(), handle, pos });
            },
            SVGPathCommandType::BEZIER_QUAD_REFLECT => {
                path.points.push(SVGPathCommand::BezierQuadReflect { id: gen_str_id(), pos });
            }
        };
    }

    pub fn children(&self) -> SVGDocInner {
        return self.inner.clone();
    }

    pub fn repr(&self) -> String {
        return serde_json::to_string(&self.inner).unwrap();
    }
}
