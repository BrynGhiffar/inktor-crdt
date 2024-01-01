use crate::prelude::*;


pub(crate) enum SVGCrdtOps {
    CreateRectangle { 
        rect_id: String,
        group_id: Option<String>,
        partial: PartialSVGRectangle,
        timestamp: UnixEpochTimeNanos,
    },
    EditRectangle {
        rectangle_id: String,
        partial: PartialSVGRectangle,
        timestamp: UnixEpochTimeNanos
    },
    CreateCircle {
        circle_id: String,
        group_id: Option<String>,
        partial: PartialSVGCircle,
        timestamp: UnixEpochTimeNanos
    },
    EditCircle {
        circle_id: String,
        partial: PartialSVGCircle,
        timestamp: UnixEpochTimeNanos
    },
    CreatePath {
        path_id: String,
        group_id: Option<String>,
        partial: PartialSVGPath,
        timestamp: UnixEpochTimeNanos
    },
    EditPath {
        path_id: String,
        partial: PartialSVGPath,
        timestamp: UnixEpochTimeNanos,
        
    },
    AddPointToPath {
        path_id: String,
        command: SVGPathCommandType,
        pos: Vec2,
        timestamp: UnixEpochTimeNanos
    },
    EditPathPointType {
        path_id: String,
        point_id: String,
        command_type: SVGPathCommandType,
        timestamp: UnixEpochTimeNanos
    },
    EditPathPointPos {
        path_id: String,
        point_id: String,
        new_pos: Vec2,
        timestamp: UnixEpochTimeNanos
    },
    EditPathPointHandle1 {
        path_id: String,
        point_id: String,
        new_handle1: Vec2,
        timestamp: UnixEpochTimeNanos
    },
    EditPathPointHandle2 {
        path_id: String,
        point_id: String,
        new_handl2: Vec2,
        timestamp: UnixEpochTimeNanos
    },
    MoveObjectToGroup {
        object_id: String,
        group_id: Option<String>,
        timestamp: UnixEpochTimeNanos
    },
    RemoveObject {
        object_id: String,
        timestamp: UnixEpochTimeNanos
    }
}

pub(crate) struct SVGDocCrdt {
    todo: VecDeque<SVGCrdtOps>,
    history: Vec<SVGCrdtOps>,
    // last_timestamp: UnixEpochTimeNanos, // the time of the last executed operation
    tree: SVGDocTree
}

impl SVGDocCrdt {
    pub fn new() -> Self {
        return Self { 
            tree: SVGDocTree { children: Vec::new() },
            todo: VecDeque::new(),
            history: Vec::new(),
        };
    }

    pub fn get_group(&self, group_id: String) -> Option<SVGGroup> {
        let group = self.tree.find_group(&group_id);
        return group.map(|g| g.clone());
    }

    pub fn add_group(&mut self, group_id: Option<String>, partial_group: PartialSVGGroup) {
        let mut new_group = SVGGroup::default();
        new_group.apply_some(partial_group);
        if let Some(group_id) = group_id {
            let Some(group) = self.tree.find_group_mut(&group_id) else { return; };
            group.children.push(SVGObject::Group(new_group));
            return;
        }
        self.tree.children.push(SVGObject::Group(new_group));
    }

    pub fn get_circle(&self, circle_id: String) -> Option<SVGCircle>{
        return self.tree.find_circle(&circle_id).map(|c| c.clone());
    }

    pub fn add_circle(&mut self, group_id: Option<String>, partial_circle: PartialSVGCircle) {
        let mut circle = SVGCircle::default();
        circle.apply_some(partial_circle);
        if let Some(group_id) = group_id {
            let Some(group) = self.tree.find_group_mut(&group_id) else { return; };
            group.children.push(SVGObject::Circle(circle));
            return;
        }
        self.tree.children.push(SVGObject::Circle(circle));
    }

    pub fn edit_circle(&mut self, circle_id: String, edits: PartialSVGCircle) {
        let Some(circle) = self.tree.find_circle_mut(&circle_id) else { return; };
        circle.apply_some(edits);
    }

    pub fn get_rectangle(&self, rectangle_id: String) -> Option<SVGRectangle> {
        let rectangle = self.tree.find_rectangle(&rectangle_id);
        return rectangle.map(|r| r.clone());
    }

    pub fn add_rectangle(&mut self, group_id: Option<String>, partial_rectangle: PartialSVGRectangle) {
        let mut rectangle = SVGRectangle::default();
        rectangle.apply_some(partial_rectangle);
        if let Some(group_id) = group_id {
            let Some(group) = self.tree.find_group_mut(&group_id) else { return; };
            group.children.push(SVGObject::Rectangle(rectangle));
            return;
        }
        self.tree.children.push(SVGObject::Rectangle(rectangle));
    }

    pub fn edit_rectangle(&mut self, rectangle_id: String, edits: PartialSVGRectangle) {
        let Some(rectangle) = self.tree.find_rectangle_mut(&rectangle_id) else { return; };
        rectangle.apply_some(edits);
    }

    pub fn get_path(&self, path_id: String) -> Option<SVGPath> {
        let path = self.tree.find_path(&path_id);
        return path.map(|p| p.clone());
    }

    pub fn add_path(&mut self, group_id: Option<String>, partial_path: PartialSVGPath) {
        let mut path = SVGPath::default();
        path.apply_some(partial_path);
        if let Some(group_id) = group_id {
            let Some(group) = self.tree.find_group_mut(&group_id) else { return; };
            group.children.push(SVGObject::Path(path));
            return;
        }
        self.tree.children.push(SVGObject::Path(path));
    }

    pub fn edit_path(&mut self, path_id: String, partial_path: PartialSVGPath) {
        let Some(path) = self.tree.find_path_mut(&path_id) else { return; };
        path.apply_some(partial_path);
    }

    pub fn edit_path_point_type(
        &mut self, 
        path_id: String, 
        point_id: String, 
        command_type: SVGPathCommandType, 
    ) {
        let Some(path) = self.tree.find_path_mut(&path_id) else { return; };
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
        let Some(path) = self.tree.find_path_mut(&path_id) else { return; };
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
        let Some(path) = self.tree.find_path_mut(&path_id) else { return; };
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
        let Some(path) = self.tree.find_path_mut(&path_id) else { return; };
        let Some(point) = path.find_point_mut(&point_id) else { return; };
        match point {
            SVGPathCommand::Bezier { handle2, .. } => {
                *handle2 = new_handle2;
            },
            _ => ()
        };
    }

    pub fn add_point_to_path(&mut self, path_id: String, command: SVGPathCommandType, pos: Vec2) {
        let Some(path) = self.tree.find_path_mut(&path_id) else { return; };
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

    pub fn move_object_to_group(&mut self, object_id: String, group_id: String, index: usize) {
        let rootIndex = self.tree.children.iter().position(|o: &SVGObject| o.get_id().eq(&object_id));
        if let Some(old_index) = rootIndex {
            let object = self.tree.children.remove(old_index);
            let Some(new_group) = self.tree.find_group_mut(&group_id) else { return; };
            if index < new_group.children.len() {
                new_group.children.insert(index, object);
            } else {
                new_group.children.push(object);
            }
            return;
        }
        let group = self.tree.find_group_has_object_id(&object_id);
        if let Some(group) = group {
            let groupIndex = group.children.iter().position(|o| o.get_id().eq(&object_id));
            let Some(old_index) = groupIndex else { return; };
            let object = group.children.remove(old_index);
            let Some(new_group) = self.tree.find_group_mut(&group_id) else { return; };
            if index < new_group.children.len() {
                new_group.children.insert(index, object);
            } else {
                new_group.children.push(object);
            }
            return;
        };
    }

    pub fn move_object_to_root(&mut self, object_id: String, index: usize) {
        let rootIndex = self.tree.children.iter().position(|o: &SVGObject| o.get_id().eq(&object_id));
        if let Some(old_index) = rootIndex {
            let object = self.tree.children.remove(old_index);
            if index < self.tree.children.len() {
                self.tree.children.insert(index, object);
            } else {
                self.tree.children.push(object);
            }
            return;
        }
        let group = self.tree.find_group_has_object_id(&object_id);
        if let Some(group) = group {
            let groupIndex = group.children.iter().position(|o| o.get_id().eq(&object_id));
            let Some(old_index) = groupIndex else { return; };
            let object = group.children.remove(old_index);
            if index < self.tree.children.len() {
                self.tree.children.insert(index, object);
            } else {
                self.tree.children.push(object);
            }
            return;
        };
    }

    pub fn remove_object(&mut self, object_id: String) {
        let index = self.tree.children
            .iter()
            .position(|o| o.get_id().eq(&object_id));
        if let Some(index) = index {
            self.tree.children.remove(index);
            return;
        }
        let group = self.tree.find_group_has_object_id(&object_id);
        if let Some(group) = group {
            let index = group.children.iter()
                .position(|o| o.get_id().eq(&object_id));
            let Some(index) = index else { return; };
            group.children.remove(index);
        }
    }

    pub fn children(&self) -> SVGDocTree {
        return self.tree.clone();
    }

    pub fn repr(&self) -> String {
        return serde_json::to_string(&self.tree).unwrap();
    }
}