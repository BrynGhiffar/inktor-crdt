pub mod circle;
pub mod group;
pub mod path;
pub mod rectangle;
use crate::prelude::*;

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

impl SVGObject {
    pub fn get_id(&self) -> &str {
        match self {
            Self::Circle(circle) => &circle.id,
            Self::Rectangle(rect) => &rect.id,
            Self::Group(grp) => &grp.id,
            Self::Path(pth) => &pth.id
        }
    }
}

#[derive(Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SVGDocTree {
    pub children: Vec<SVGObject>
}


impl SVGDocTree {

    pub (crate) fn new() -> Self {
        Self { children: Vec::new() }
    }

    pub(crate) fn find_group_has_object_id<'a>(
        &'a mut self, 
        object_id: &'a str
    ) -> Option<&'a mut SVGGroup> {
        for children in self.children.iter_mut() {
            let SVGObject::Group(group) = children else { continue; };
            if let Some(group) = Self::find_group_has_object_id_aux(group, object_id) {
                return Some(group);
            }
        }
        return None;
    }

    pub(crate) fn find_group_has_object_id_aux<'a>(
        group: &'a mut SVGGroup, 
        object_id: &'a str
    ) -> Option<&'a mut SVGGroup> {
        for children in group.children.iter() {
            if children.get_id().eq(object_id) { return Some(group); }
        }
        for children in group.children.iter_mut() {
            let SVGObject::Group(cgroup) = children else { continue; };
            if let Some(cgroup) = Self::find_group_has_object_id_aux(cgroup, object_id) {
                return Some(cgroup);
            }
        }
        return None;
    }

    pub(crate) fn find_group_mut_aux<'a>(group: &'a mut SVGGroup, group_id: &'a str) -> Option<&'a mut SVGGroup> {
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

    pub(crate) fn find_group_mut<'a>(&'a mut self, group_id: &'a str) -> Option<&'a mut SVGGroup> {
        for children in self.children.iter_mut() {
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

    pub(crate) fn find_group_aux<'a>(group: &'a SVGGroup, group_id: &'a str) -> Option<&'a SVGGroup> {
        for children in group.children.iter() {
            let SVGObject::Group(group) = children else { continue; };
            if group.id == group_id { return Some(group) }
            let Some(group) = Self::find_group_aux(group, group_id) else { continue; };
            return Some(group);

        }
        return None;
    }

    pub(crate) fn find_group<'a>(&'a self, group_id: &'a str) -> Option<&'a SVGGroup> {
        for children in self.children.iter() {
            let SVGObject::Group(group) = children else { continue; };
            if group.id == group_id { return Some(group); };
            let Some(group) = Self::find_group_aux(group, group_id) else { continue; };
            return Some(group);
        }
        return None;
    }


    pub(crate) fn find_circle_mut_aux<'a>(group: &'a mut SVGGroup, circle_id: &'a str) -> Option<&'a mut SVGCircle> {
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

    pub(crate) fn find_circle_mut<'a>(&'a mut self, circle_id: &'a str) -> Option<&'a mut SVGCircle> {
        for children in self.children.iter_mut() {
            if let SVGObject::Circle(circle) = children {
                if circle.id.eq(circle_id) { return Some(circle); }
            } else if let SVGObject::Group(group) = children {
                let Some(circle) = Self::find_circle_mut_aux(group, circle_id) else { continue; };
                return Some(circle);
            }
        }
        return None;
    }

    pub(crate) fn find_circle_aux<'a>(group: &'a SVGGroup, circle_id: &'a str) -> Option<&'a SVGCircle> {
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

    pub(crate) fn find_circle<'a>(&'a self, circle_id: &'a str) -> Option<&'a SVGCircle> {
        for children in self.children.iter() {
            if let SVGObject::Circle(circle) = children {
                if circle.id == circle_id { return Some(circle); }
            } else if let SVGObject::Group(group) = children {
                let Some(circle) = Self::find_circle_aux(group, circle_id) else { continue; };
                return Some(circle);
            }
        }
        return None;
    }

    pub(crate) fn find_rectangle_mut_aux<'a>(group: &'a mut SVGGroup, rectangle_id: &'a str) -> Option<&'a mut SVGRectangle> {
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

    pub(crate) fn find_rectangle_mut<'a>(&'a mut self, rectangle_id: &'a str) -> Option<&'a mut SVGRectangle> {
        for children in self.children.iter_mut() {
            if let SVGObject::Rectangle(rectangle) = children {
                if rectangle.id.eq(rectangle_id) { return Some(rectangle); }
            } else if let SVGObject::Group(group) = children {
                let Some(rectangle) = Self::find_rectangle_mut_aux(group, rectangle_id) else { continue; };
                return Some(rectangle);
            }
        }
        return None;
    }

    pub(crate) fn find_rectangle_aux<'a>(group:&'a SVGGroup, rectangle_id: &'a str) -> Option<&'a SVGRectangle> {
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

    pub(crate) fn find_rectangle<'a>(&'a self, rectangle_id: &'a str) -> Option<&'a SVGRectangle> {
        for child in self.children.iter() {
            if let SVGObject::Rectangle(rect) = child {
                if rect.id == rectangle_id { return Some(rect); }
            } else if let SVGObject::Group(group) = child {
                let Some(rectangle) = Self::find_rectangle_aux(group, rectangle_id) else { continue; };
                return Some(rectangle);
            }
        }
        return None;
    }

    pub(crate) fn find_path_mut_aux<'a>(group: &'a mut SVGGroup, path_id: &'a str) -> Option<&'a mut SVGPath> {
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

    pub(crate) fn find_path_mut<'a>(&'a mut self, path_id: &'a str) -> Option<&'a mut SVGPath> {
        for children in self.children.iter_mut() {
            if let SVGObject::Path(path) = children {
                if path.id.eq(path_id) { return Some(path); };
            } else if let SVGObject::Group(group) = children {
                let Some(path) = Self::find_path_mut_aux(group, path_id) else { continue; };
                return Some(path);
            }
        }
        return None;
    }

    pub(crate) fn find_path_aux<'a>(group: &'a SVGGroup, path_id: &'a str) -> Option<&'a SVGPath> {
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

    pub(crate) fn find_path<'a>(&'a self, path_id: &'a str) -> Option<&'a SVGPath> {
        for child in self.children.iter() {
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