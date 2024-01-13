use std::collections::HashMap;

use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub(crate) enum SVGCrdtOps {
    AddGroup {
        group_id: Option<String>,
        new_group_id: String,
        partial_group: PartialSVGGroup,
        timestamp: UnixEpochTimeNanos,
    },
    AddRectangle { 
        rect_id: String,
        group_id: Option<String>,
        partial: PartialSVGRectangle,
        timestamp: UnixEpochTimeNanos,
    },
    AddCircle {
        circle_id: String,
        group_id: Option<String>,
        partial: PartialSVGCircle,
        timestamp: UnixEpochTimeNanos
    },
    AddPath {
        path_id: String,
        group_id: Option<String>,
        partial: PartialSVGPath,
        timestamp: UnixEpochTimeNanos
    },
    AddPointToPath {
        path_id: String,
        point_id: String,
        command: SVGPathCommandType,
        pos: Vec2,
        timestamp: UnixEpochTimeNanos
    },
    EditCircle {
        circle_id: String,
        partial: PartialSVGCircle,
        timestamp: UnixEpochTimeNanos
    },
    EditRectangle {
        rectangle_id: String,
        partial: PartialSVGRectangle,
        timestamp: UnixEpochTimeNanos
    },
    EditPath {
        path_id: String,
        partial: PartialSVGPath,
        timestamp: UnixEpochTimeNanos,
        
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
        new_handle2: Vec2,
        timestamp: UnixEpochTimeNanos
    },
    MoveObjectToGroup {
        object_id: String,
        group_id: Option<String>,
        index: usize,
        timestamp: UnixEpochTimeNanos
    },
    RemoveObject {
        object_id: String,
        timestamp: UnixEpochTimeNanos
    },
    RemovePathPoint {
        path_id: String,
        point_id: String,
        timestamp: UnixEpochTimeNanos
    }
}

impl SVGCrdtOps {
    pub(crate) fn get_timestamp(&self) -> UnixEpochTimeNanos {
        match *self {
            Self::AddGroup { timestamp, .. } => timestamp,
            Self::AddCircle { timestamp, .. } => timestamp,
            Self::AddPath { timestamp, .. } => timestamp,
            Self::AddPointToPath { timestamp, .. } => timestamp,
            Self::AddRectangle { timestamp, .. } => timestamp, 
            Self::EditCircle { timestamp, .. } => timestamp,
            Self::EditPath { timestamp, .. } => timestamp,
            Self::EditPathPointHandle1 { timestamp, .. } => timestamp,
            Self::EditPathPointHandle2 { timestamp, .. } => timestamp,
            Self::EditPathPointPos { timestamp, .. } => timestamp,
            Self::EditPathPointType { timestamp, .. } => timestamp,
            Self::EditRectangle { timestamp, .. } => timestamp,
            Self::MoveObjectToGroup { timestamp, .. } => timestamp,
            Self::RemoveObject { timestamp, .. } => timestamp,
            Self::RemovePathPoint { timestamp, .. } => timestamp,
        }
    }
}

pub(crate) struct SVGDocCrdt {
    // todo is an ordered array ofsvg crdt operations
    todo: VecDeque<SVGCrdtOps>,
    history: Vec<SVGCrdtOps>,
    parent: HashMap<String, Option<String>>,
    // last_timestamp: UnixEpochTimeNanos, // the time of the last executed operation
    tree: SVGDocTree
}

impl SVGDocCrdt {
    fn do_mut_op(&mut self, op: SVGCrdtOps) {
        match op {
            SVGCrdtOps::AddGroup { group_id, new_group_id, partial_group, .. } => {
                self.do_add_group_op(group_id, new_group_id, partial_group)
            },
            SVGCrdtOps::AddCircle { circle_id, group_id, partial, .. } => {
                self.do_add_circle_op(circle_id, group_id, partial)
            },
            SVGCrdtOps::AddRectangle { rect_id, group_id, partial, .. } => {
                self.do_add_rectangle_op(rect_id, group_id, partial)
            },
            SVGCrdtOps::AddPath { path_id, group_id, partial, .. } => {
                self.do_add_path_op(path_id, group_id, partial)
            },
            SVGCrdtOps::AddPointToPath { path_id, point_id, command, pos, .. } => {
                self.do_add_point_to_path_op(path_id, point_id, command, pos)
            },
            SVGCrdtOps::EditCircle { circle_id, partial, .. } => {
                self.do_edit_circle_op(circle_id, partial);
            },
            SVGCrdtOps::EditRectangle { rectangle_id, partial: edits, .. } => {
                self.do_edit_rectangle_op(rectangle_id, edits)
            },
            SVGCrdtOps::EditPath { path_id, partial, .. } => {
                self.do_edit_path_op(path_id, partial)
            },
            SVGCrdtOps::EditPathPointType { path_id, point_id, command_type, .. } => {
                self.do_edit_path_point_type_op(path_id, point_id, command_type)
            },
            SVGCrdtOps::EditPathPointPos { path_id, point_id, new_pos, .. } => {
                self.do_edit_path_point_pos_op(path_id, point_id, new_pos);
            },
            SVGCrdtOps::EditPathPointHandle1 { path_id, point_id, new_handle1, .. } => {
                self.do_edit_path_point_handle1_op(path_id, point_id, new_handle1);
            },
            SVGCrdtOps::EditPathPointHandle2 { path_id, point_id, new_handle2, .. } => {
                self.do_edit_path_point_handle2_op(path_id, point_id, new_handle2);
            },
            SVGCrdtOps::MoveObjectToGroup { object_id, group_id, index, .. } => {
                if let Some(group_id) = group_id {
                    self.do_move_object_to_group_op(object_id, group_id, index);
                } else {
                    self.do_move_object_to_root_op(object_id, index);
                }
            },
            SVGCrdtOps::RemoveObject { object_id, .. } => {
                self.do_remove_object_op(object_id)
            },
            SVGCrdtOps::RemovePathPoint { path_id, point_id, .. } => {
                self.do_remove_path_point_op(path_id, point_id)
            }
        }
    }

    fn is_ancestor(&self, object1_id: String, object2_id: String) -> bool{
        // Is object1 an ancestor of object2
        let Some(Some(parent)) = self.parent.get(&object2_id) else { return false; };
        let parent = parent.clone();
        if &parent == &object1_id { return true; }
        return self.is_ancestor(object1_id, parent);
    }

    fn execute_all_todo(&mut self) {
        while let Some(op) = self.todo.pop_front() {
            self.do_mut_op(op.clone());
            self.history.push(op);
        }
    }
}


// do_ operations
impl SVGDocCrdt {
    fn do_add_group_op(
        &mut self, 
        group_id: Option<String>,
        new_group_id: String,
        partial_group: PartialSVGGroup
    ) {
        let mut new_group = SVGGroup::default();
        new_group.id = new_group_id.clone();
        new_group.apply_some(partial_group);
        *self.parent.entry(new_group_id.clone()).or_insert(group_id.clone()) = group_id.clone();
        if let Some(group_id) = group_id {
            if new_group_id == group_id { return }
            if self.is_ancestor(new_group_id.clone(), group_id.clone()) { return; }
            let Some(group) = self.tree.find_group_mut(&group_id) else { return; };
            group.children.push(SVGObject::Group(new_group));
            return;
        }
        self.tree.children.push(SVGObject::Group(new_group));
    }

    fn do_add_circle_op(
        &mut self,
        circle_id: String,
        group_id: Option<String>,
        partial_circle: PartialSVGCircle
    ) {
        let mut circle = SVGCircle::default();
        circle.id = circle_id.clone();
        circle.apply_some(partial_circle);
        *self.parent.entry(circle_id.clone()).or_insert(group_id.clone()) = group_id.clone();
        if let Some(group_id) = group_id {
            if circle_id == group_id { return }
            if self.is_ancestor(circle_id.clone(), group_id.clone()) { return; }
            let Some(group) = self.tree.find_group_mut(&group_id) else { return; };
            group.children.push(SVGObject::Circle(circle));
            return;
        }
        self.tree.children.push(SVGObject::Circle(circle));
    }

    fn do_add_rectangle_op(
        &mut self,
        rectangle_id: String,
        group_id: Option<String>, 
        partial_rectangle: PartialSVGRectangle
    ) {
        let mut rectangle = SVGRectangle::default();
        rectangle.id = rectangle_id.clone();
        rectangle.apply_some(partial_rectangle);
        *self.parent.entry(rectangle_id.clone()).or_insert(group_id.clone()) = group_id.clone();
        if let Some(group_id) = group_id {
            if rectangle_id == group_id { return }
            if self.is_ancestor(rectangle_id.clone(), group_id.clone()) { return; }
            let Some(group) = self.tree.find_group_mut(&group_id) else { return; };
            group.children.push(SVGObject::Rectangle(rectangle));
            return;
        }
        self.tree.children.push(SVGObject::Rectangle(rectangle));
    }

    fn do_add_path_op(
        &mut self,
        path_id: String,
        group_id: Option<String>,
        partial_path: PartialSVGPath
    ) {
        let mut path = SVGPath::default();
        path.id = path_id.clone();
        path.apply_some(partial_path);
        *self.parent.entry(path_id.clone()).or_insert(group_id.clone()) = group_id.clone();
        if let Some(group_id) = group_id {
            if path_id == group_id { return }
            if self.is_ancestor(path_id.clone(), group_id.clone()) { return; }
            let Some(group) = self.tree.find_group_mut(&group_id) else { return; };
            group.children.push(SVGObject::Path(path));
            return;
        }
        self.tree.children.push(SVGObject::Path(path));
    }

    fn do_add_point_to_path_op(
        &mut self,
        path_id: String,
        point_id: String,
        command: SVGPathCommandType,
        pos: Vec2
    ) {
        let Some(path) = self.tree.find_path_mut(&path_id) else { return; };
        match command {
            SVGPathCommandType::START => {
                path.points.push(SVGPathCommand::Start { id: point_id, pos });
            },
            SVGPathCommandType::LINE => {
                path.points.push(SVGPathCommand::Line { id: point_id, pos });
            },
            SVGPathCommandType::CLOSE => {
                path.points.push(SVGPathCommand::Close { id: point_id });
            },
            SVGPathCommandType::BEZIER => {
                let handle1 = Vec2 { x: pos.x + 20, y: pos.y + 20 };
                let handle2 = Vec2 { x: pos.x + 20, y: pos.y - 20 };
                path.points.push(SVGPathCommand::Bezier { id: point_id, handle1, handle2, pos });
            },
            SVGPathCommandType::BEZIER_REFLECT => {
                let handle = Vec2 { x: pos.x, y: pos.y + 20 };
                path.points.push(SVGPathCommand::BezierReflect { id: point_id, handle, pos });
            },
            SVGPathCommandType::BEZIER_QUAD => {
                let handle = Vec2 { x: pos.x, y: pos.y + 20 };
                path.points.push(SVGPathCommand::BezierQuad { id: point_id, handle, pos });
            },
            SVGPathCommandType::BEZIER_QUAD_REFLECT => {
                path.points.push(SVGPathCommand::BezierQuadReflect { id: point_id, pos });
            }
        };
    }

    fn do_edit_circle_op(
        &mut self, 
        circle_id: String, 
        edits: PartialSVGCircle
    ) {
        let Some(circle) = self.tree.find_circle_mut(&circle_id) else { return; };
        circle.apply_some(edits);
    }

    fn do_edit_rectangle_op(
        &mut self,
        rectangle_id: String,
        edits: PartialSVGRectangle
    ) {
        let Some(rectangle) = self.tree.find_rectangle_mut(&rectangle_id) else { return; };
        rectangle.apply_some(edits);
    }

    fn do_edit_path_op(
        &mut self,
        path_id: String,
        partial_path: PartialSVGPath
    ) {
        let Some(path) = self.tree.find_path_mut(&path_id) else { return; };
        path.apply_some(partial_path);
    }

    fn do_edit_path_point_type_op(
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

    fn do_edit_path_point_pos_op(
        &mut self,
        path_id: String,
        point_id: String,
        new_pos: Vec2
    ) {
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

    fn do_edit_path_point_handle1_op(
        &mut self,
        path_id: String,
        point_id: String,
        new_handle1: Vec2
    ) {
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

    fn do_edit_path_point_handle2_op(
        &mut self,
        path_id: String,
        point_id: String,
        new_handle2: Vec2
    ) {
        let Some(path) = self.tree.find_path_mut(&path_id) else { return; };
        let Some(point) = path.find_point_mut(&point_id) else { return; };
        match point {
            SVGPathCommand::Bezier { handle2, .. } => {
                *handle2 = new_handle2;
            },
            _ => ()
        };
    }

    fn do_move_object_to_group_op(
        &mut self, 
        object_id: String, 
        group_id: String, 
        index: usize
    ) {
        // TODO: This will fail if you are putting an object inside itself. i.e. object_id == group_id
        // TODO: This will fail if object_id is an ancestor of group_id. That is when object_id is also a group.
        // TODO: Need to add check for whether object_id is ancestor of group_id
        if object_id == group_id { return }
        if self.is_ancestor(object_id.clone(), group_id.clone()) { return; }
        let rootIndex = self.tree.children.iter().position(|o: &SVGObject| o.get_id().eq(&object_id));
        if let Some(old_index) = rootIndex {
            let object = self.tree.children.remove(old_index);
            let Some(new_group) = self.tree.find_group_mut(&group_id) else { return; };
            *self.parent.entry(object_id.clone()).or_insert(Some(group_id.clone())) = Some(group_id.clone());
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
            *self.parent.entry(object_id.clone()).or_insert(Some(group_id.clone())) = Some(group_id.clone());
            if index < new_group.children.len() {
                new_group.children.insert(index, object);
            } else {
                new_group.children.push(object);
            }
            return;
        };
    }

    fn do_move_object_to_root_op(
        &mut self,
        object_id: String,
        index: usize
    ) {
        // Check is ancestor is not required, since no element is ancestor to the root element.
        let rootIndex = self.tree.children.iter().position(|o: &SVGObject| o.get_id().eq(&object_id));
        if let Some(old_index) = rootIndex {
            let object = self.tree.children.remove(old_index);
            *self.parent.entry(object_id.clone()).or_insert(None) = None;
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
            *self.parent.entry(object_id.clone()).or_insert(None) = None;
            if index < self.tree.children.len() {
                self.tree.children.insert(index, object);
            } else {
                self.tree.children.push(object);
            }
            return;
        };
    }

    fn do_remove_path_point_op(
        &mut self,
        path_id: String,
        point_id: String
    ) {
        let path = self.tree.find_path_mut(&path_id);
        let Some(path) = path else { return; };
        let index = path.points.iter()
            .position(|o| o.get_id().eq(&point_id));
        let Some(index) = index else { return; };
        path.points.remove(index);
    }

    fn do_remove_object_op(
        &mut self,
        object_id: String,
    ) {
        let index = self.tree.children
            .iter()
            .position(|o| o.get_id().eq(&object_id));
        self.parent.remove(&object_id);
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
}

impl SVGDocCrdt {
    pub fn new() -> Self {
        return Self { 
            tree: SVGDocTree::new(),
            todo: VecDeque::new(),
            parent: HashMap::new(),
            history: Vec::new(),
        };
    }

    pub fn get_group(&self, group_id: String) -> Option<SVGGroup> {
        let group = self.tree.find_group(&group_id);
        return group.map(|g| g.clone());
    }

    pub fn get_circle(&self, circle_id: String) -> Option<SVGCircle>{
        return self.tree.find_circle(&circle_id).map(|c| c.clone());
    }

    pub fn get_rectangle(
        &self, 
        rectangle_id: String
    ) -> Option<SVGRectangle> {
        let rectangle = self.tree.find_rectangle(&rectangle_id);
        return rectangle.map(|r| r.clone());
    }

    pub fn get_path(&self, path_id: String) -> Option<SVGPath> {
        let path = self.tree.find_path(&path_id);
        return path.map(|p| p.clone());
    }

    pub fn add_group(
        &mut self, 
        group_id: Option<String>, 
        partial_group: PartialSVGGroup
    ) {
        let timestamp = epoch_now_nanos();
        let new_group_id = gen_str_id();
        self.todo.push_back(SVGCrdtOps::AddGroup { 
            group_id, 
            new_group_id,
            partial_group, 
            timestamp 
        });
        self.execute_all_todo()
    }

    pub fn add_circle(
        &mut self, 
        group_id: Option<String>, 
        partial_circle: PartialSVGCircle
    ) {
        let timestamp = epoch_now_nanos();
        let circle_id = gen_str_id();
        self.todo.push_back(SVGCrdtOps::AddCircle { 
            circle_id, 
            group_id, 
            partial: partial_circle, 
            timestamp
        });
        self.execute_all_todo();
    }

    pub fn add_rectangle(
        &mut self, 
        group_id: Option<String>, 
        partial_rectangle: PartialSVGRectangle
    ) {
        let timestamp = epoch_now_nanos();
        let rect_id = gen_str_id();
        self.todo.push_back(SVGCrdtOps::AddRectangle { 
            rect_id, 
            group_id, 
            partial: partial_rectangle, 
            timestamp 
        });
        self.execute_all_todo();
    }

    pub fn edit_circle(
        &mut self, 
        circle_id: String, 
        edits: PartialSVGCircle
    ) {
        let timestamp = epoch_now_nanos();
        self.todo.push_back(SVGCrdtOps::EditCircle { 
            circle_id, 
            partial: edits, 
            timestamp
        });
        self.execute_all_todo();
    }

    pub fn edit_rectangle(
        &mut self, 
        rectangle_id: String, 
        edits: PartialSVGRectangle
    ) {
        let timestamp = epoch_now_nanos();
        self.todo.push_back(SVGCrdtOps::EditRectangle { 
            rectangle_id,
            partial: edits, 
            timestamp
        });
        self.execute_all_todo();
    }

    pub fn add_path(
        &mut self, 
        group_id: Option<String>, 
        partial_path: PartialSVGPath
    ) {
        let timestamp = epoch_now_nanos();
        let path_id = gen_str_id();
        self.todo.push_back(SVGCrdtOps::AddPath { 
            path_id, 
            group_id, 
            partial: partial_path, 
            timestamp 
        });
        self.execute_all_todo();
    }

    pub fn edit_path(
        &mut self, 
        path_id: String, 
        partial_path: PartialSVGPath
    ) {
        let timestamp = epoch_now_nanos();
        self.todo.push_back(SVGCrdtOps::EditPath { 
            path_id, 
            partial: partial_path, 
            timestamp 
        });
        self.execute_all_todo();
    }

    pub fn edit_path_point_type(
        &mut self, 
        path_id: String, 
        point_id: String,
        command_type: SVGPathCommandType, 
    ) {
        let timestamp = epoch_now_nanos();
        self.todo.push_back(SVGCrdtOps::EditPathPointType {
            path_id, 
            point_id, 
            command_type, 
            timestamp
        });
        self.execute_all_todo();

    }

    pub fn edit_path_point_pos(
        &mut self, 
        path_id: String, 
        point_id: String, 
        new_pos: Vec2
    ) {
        let timestamp = epoch_now_nanos();
        self.todo.push_back(SVGCrdtOps::EditPathPointPos { 
            path_id, 
            point_id, 
            new_pos, 
            timestamp
        });
        self.execute_all_todo();
    }

    pub fn edit_path_point_handle1(
        &mut self, 
        path_id: String, 
        point_id: String, 
        new_handle1: Vec2
    ) {
        let timestamp = epoch_now_nanos();
        self.todo.push_back(SVGCrdtOps::EditPathPointHandle1 { 
            path_id, 
            point_id, 
            new_handle1, 
            timestamp
        });
        self.execute_all_todo();
    }

    pub fn edit_path_point_handle2(
        &mut self, 
        path_id: String, 
        point_id: String, 
        new_handle2: Vec2
    ) {
        let timestamp = epoch_now_nanos();
        self.todo.push_back(SVGCrdtOps::EditPathPointHandle2 { 
            path_id, 
            point_id, 
            new_handle2, 
            timestamp
        });
        self.execute_all_todo();
    }

    pub fn add_point_to_path(
        &mut self, 
        path_id: String, 
        command: SVGPathCommandType, 
        pos: Vec2
    ) {
        let timestamp = epoch_now_nanos();
        let point_id = gen_str_id();
        self.todo.push_back(SVGCrdtOps::AddPointToPath { 
            path_id, 
            point_id,
            command, 
            pos, 
            timestamp
        });
        self.execute_all_todo();
    }

    pub fn move_object_to_group(
        &mut self, 
        object_id: String, 
        group_id: String, 
        index: usize
    ) {
        let timestamp = epoch_now_nanos();
        self.todo.push_back(SVGCrdtOps::MoveObjectToGroup { 
            object_id, 
            group_id: Some(group_id), 
            index,
            timestamp
        });
        self.execute_all_todo();
    }

    pub fn move_object_to_root(&mut self, object_id: String, index: usize) {
        let timestamp = epoch_now_nanos();
        self.todo.push_back(SVGCrdtOps::MoveObjectToGroup { 
            object_id, 
            group_id: None, 
            timestamp, 
            index
        });
        self.execute_all_todo();
    }

    pub fn remove_object(&mut self, object_id: String) {
        let timestamp = epoch_now_nanos();
        self.todo.push_back(SVGCrdtOps::RemoveObject { 
            object_id, 
            timestamp
        });
        self.execute_all_todo();
    }

    pub fn remove_path_point(
        &mut self, 
        path_id: String, 
        point_id: String
    ) {
        let timestamp = epoch_now_nanos();
        self.todo.push_back(SVGCrdtOps::RemovePathPoint { 
            path_id, 
            point_id,
            timestamp 
        });
        self.execute_all_todo();
    }

    pub fn save_oplog(&self) -> Option<String> {
        serde_json::to_string(&self.history).ok()
    }

    pub fn merge(&mut self, oplog: Vec<SVGCrdtOps>) {

        #[cfg(feature = "debug")]
        let start_at = epoch_now().as_millis();

        let mut i1 = 0;
        let mut i2 = 0;
        let n1 = self.history.len();
        let n2 = oplog.len();
        while i1 < n1 && i2 < n2 {
            let t1 = self.history[i1].get_timestamp();
            let t2 = oplog[i2].get_timestamp();
            if t1 < t2 {
                self.todo.push_back(self.history[i1].clone());
                i1 += 1;
            } else if t2 < t1 {
                self.todo.push_back(oplog[i2].clone());
                i2 += 1;
            } else if t1 == t2 {
                self.todo.push_back(self.history[i1].clone());
                i1 += 1;
                i2 += 1;
            }
        }
        while i1 < n1 {
            self.todo.push_back(self.history[i1].clone());
            i1 += 1;
        }
        while i2 < n2 {
            self.todo.push_back(oplog[i2].clone());
            i2 += 1;
        }

        #[cfg(feature = "debug")]
        {
            let end_at = epoch_now().as_millis();
            let merge_duration = end_at - start_at;
            console_log!("{} operations was merged", n1 + n2);
            console_log!("Merge took: {}ms", merge_duration);
            console_log!("Merging started at: {}ms", start_at);
            console_log!("Merging ended at: {}ms", end_at);
        }

        self.clear_tree();
        self.execute_all_todo();
    }
    
    pub fn clear_tree(&mut self) {
        self.history = vec![];
        self.tree = SVGDocTree::new();
        self.parent = HashMap::new();
    }

    pub fn children(&self) -> SVGDocTree {
        return self.tree.clone();
    }

    pub fn repr(&self) -> String {
        return serde_json::to_string(&self.tree).unwrap();
    }
}