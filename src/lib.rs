use wasm_bindgen::prelude::*;

pub struct Vec2 {
    x: i32,
    y: i32,
}

pub struct Color(i32 /* red (0 - 255) */, i32 /* green (0 - 255) */, i32 /* blue (0 - 255) */, i32 /* (0 - 100) */);

impl Color {
    fn white() -> Color {
        Color(255, 255, 255, 100)
    }

    fn black() -> Color {
        Color(0, 0, 0, 0)
    }

}

pub enum SVGPathCommand {
    Start(Vec2), // M
    Line(Vec2), // L
    Close(Vec2), // Z
    Bezier(Vec2, Vec2, Vec2), // C
    BezierReflect(Vec2, Vec2), // S
    BezierQuad(Vec2, Vec2), // Q
    BezierQuadReflect(Vec2), // T
}

pub struct SVGGroup {
    fill: Color,
    stroke: Color,
    stroke_width: i32,
    children: Vec<SVGObject>
}

impl SVGGroup {
    pub fn set_fill(&mut self, red: i32, green: i32, blue: i32, opacity: i32) {
        self.fill = Color(red, green, blue, opacity);
    }

    pub fn set_stroke(&mut self, red: i32, green: i32, blue: i32, opacity: i32) {
        self.stroke = Color(red, green, blue, opacity);
    }

    pub fn set_stroke_width(&mut self, width: i32) {
        self.stroke_width = width;
    }

    pub fn add_circle(&mut self, x: i32, y: i32) {
        let mut circle = SVGCircle::default();
        let pos = Vec2 { x, y };
        circle.pos = pos;
        self.children.push(SVGObject::Circle(circle));
    }

    pub fn add_rectangle(&mut self, x: i32, y: i32) {
        let mut rect = SVGRectangle::default();
        rect.pos = Vec2 { x, y };
        self.children.push(SVGObject::Rectangle(rect));
    }

    pub fn start_path(&mut self, x: i32, y: i32) {
        let mut path = SVGPath::default();
        path.points.push(SVGPathCommand::Start(Vec2 { x, y }));
        self.children.push(SVGObject::Path(path));
    }
}

#[wasm_bindgen]
pub struct SVGCircle {
    pos: Vec2,
    radius: i32,
    fill: Color,
    stroke_width: i32,
    stroke: Color
}

impl SVGCircle {

    fn default() -> Self {
        return SVGCircle { pos: Vec2 { x: 0, y: 0 }, radius: 10, fill: Color::white(), stroke_width: 2, stroke: Color::black() };
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

pub struct SVGRectangle {
    pos: Vec2,
    length: i32,
    width: i32,
    fill: Color,
    stroke_width: i32,
    stroke: Color,

}

impl SVGRectangle {

    fn default()  -> Self {
        SVGRectangle { pos: Vec2 { x: 0, y: 0 }, length: 5, width: 10, fill: Color::white(), stroke_width: 2, stroke: Color::black() }
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.pos = Vec2 { x, y };
    }

    pub fn set_length(&mut self, length: i32) {
        self.length = length;
    }

    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    pub fn set_fill(&mut self, red: i32, green: i32, blue: i32, opacity: i32) {
        self.fill = Color(red, green, blue, opacity);
    }

    pub fn set_stroke_width(&mut self, stroke_width: i32) {
        self.stroke_width = stroke_width;
    }

    pub fn set_stroke(&mut self, red: i32, green: i32, blue: i32, opacity: i32) {
        self.stroke = Color(red, green, blue, opacity);
    }
}

pub struct SVGPath {
    fill: Color,
    stroke_width: i32,
    stroke: Color,
    points: Vec<SVGPathCommand>
}

impl SVGPath {
    fn default() -> Self {
        SVGPath { 
            fill: Color::white(), 
            stroke_width: 2, 
            stroke: Color::black(), 
            points: vec![] 
        }
    }
}

enum SVGObject {
    Circle(SVGCircle),
    Rectangle(SVGRectangle),
    Path(SVGPath),
    Group(SVGGroup)
}

#[wasm_bindgen]
pub struct SVGDoc {
    children: Vec<SVGObject>
}

impl SVGDoc {
    // fn object_aux(object: SVGObject) -> JsValue {
    // }
}

#[wasm_bindgen]
impl SVGDoc {
    pub fn new() -> Self {
        return SVGDoc { children: Vec::new() };
    }

    pub fn repr(&self) -> String {
        return String::from("cat");
    }
}
