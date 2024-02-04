use crate::prelude::*;
pub mod circle;
pub mod rectangle;
pub mod group;
pub mod path;

#[derive(Clone, Serialize, Deserialize)]
pub enum LWWSVGObject {
    Circle(LWWSVGCircle),
    Rectangle(LWWSVGRectangle),
    Path(LWWSVGPath),
    Group(LWWSVGGroup)
}

impl LWWSVGObject {
    pub fn value(&self) -> SVGObject {
        match *self {
            LWWSVGObject::Circle(ref circle) => SVGObject::Circle(circle.value()),
            LWWSVGObject::Rectangle(ref rectangle) => SVGObject::Rectangle(rectangle.value()),
            LWWSVGObject::Group(ref group) => SVGObject::Group(group.value()),
            LWWSVGObject::Path(ref path) => SVGObject::Path(path.value())
        }
    }
}

impl From<SVGObject> for LWWSVGObject {
    fn from(value: SVGObject) -> Self {
        match value {
            SVGObject::Circle(circle) => LWWSVGObject::Circle(circle.into()),
            SVGObject::Group(group) => LWWSVGObject::Group(group.into()),
            SVGObject::Path(path) => LWWSVGObject::Path(path.into()),
            SVGObject::Rectangle(rectangle) => LWWSVGObject::Rectangle(rectangle.into()),
        }
    }
}