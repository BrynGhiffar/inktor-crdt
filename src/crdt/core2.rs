use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveLog {
    old_group_id: Option<NodeID>,
    new_group_id: Option<NodeID>,
    object_id: NodeID,
    index: FractionalIndex,
    timestamp: UnixEpochTimeNanos
}

static NEW_NODE_ROOT_ID: &'static str = "NEW_NODES_ROOT_ID";

// pub struct MoveOp {
//     group_id: Option<NodeID>,
//     object_id: NodeID,
//     timestamp: UnixEpochTimeNanos,
//     index: usize,
// }

#[derive(Clone, Serialize, Deserialize)]
pub struct LWWNodeMapItem {
    object: LWWReg<LWWSVGObject>,
    parent_id: LWWReg<Option<NodeID>>,
    index: LWWReg<FractionalIndex>,
}

impl LWWNodeMapItem {
    pub fn new(NodeMapItem { object, parent_id, index }: NodeMapItem) -> Self {
        Self { 
            object: LWWReg::new(object.into()), 
            parent_id: LWWReg::new(parent_id), 
            index: LWWReg::new(index)
        }
    }
    
    pub fn update_object(&mut self, object: SVGObject) {
        self.object.set(object.into());
    }

    pub fn update_circle(&mut self, edits: PartialSVGCircle) {
        let LWWSVGObject::Circle(ref mut circle) = self.object.val.borrow_mut() else { return; };
        circle.apply_some(edits);
    }

    pub fn update_group(&mut self, edits: PartialSVGGroup) {
        let LWWSVGObject::Group(ref mut g) = self.object.val.borrow_mut() else { return; };
        g.apply_some(edits);
    }

    pub fn update_rectangle(&mut self, edits: PartialSVGRectangle) {
        let LWWSVGObject::Rectangle(ref mut rect) = self.object.val.borrow_mut() else { return; };
        rect.apply_some(edits);
    }

    pub fn update_path(&mut self, edits: PartialSVGPath) {
        let LWWSVGObject::Path(ref mut path) = self.object.val.borrow_mut() else { return; };
        path.apply_some(edits);
    }

    pub fn update_path_points(&mut self, points: Vec<SVGPathCommand>) {
        let LWWSVGObject::Path(ref mut path) = self.object.val.borrow_mut() else { return; };
        path.set_points(points);
    }

    pub fn update_parent_id(&mut self, parent_id: Option<NodeID>) {
        self.parent_id.set(parent_id);
    }

    pub fn update_index(&mut self, index: FractionalIndex) {
        self.index.set(index);
    }

    pub fn value(&self) -> NodeMapItem {
        NodeMapItem { 
            object: self.object.value().value(), 
            parent_id: self.parent_id.value().clone(), 
            index: self.index.value().clone()
        }
    }
}

impl From<NodeMapItem> for LWWNodeMapItem {
    fn from(value: NodeMapItem) -> Self {
        Self::new(value)
    }
}

pub struct NodeMapItem {
    object: SVGObject,
    parent_id: Option<NodeID>,
    index: FractionalIndex,
}

impl Mergeable for LWWNodeMapItem {
    fn merge(&self, other: &Self) -> Self {
        let object = match (self.object.value(), other.object.value()) {
            (LWWSVGObject::Group(g1), LWWSVGObject::Group(g2)) => {
                LWWReg::new(LWWSVGObject::Group(g1.merge(g2)))
            },
            (LWWSVGObject::Circle(c1), LWWSVGObject::Circle(c2)) => {
                LWWReg::new(LWWSVGObject::Circle(c1.merge(c2)))
            },
            (LWWSVGObject::Path(p1), LWWSVGObject::Path(p2)) => {
                LWWReg::new(LWWSVGObject::Path(p1.merge(p2)))
            },
            (LWWSVGObject::Rectangle(r1), LWWSVGObject::Rectangle(r2)) => {
                LWWReg::new(LWWSVGObject::Rectangle(r1.merge(r2)))
            },
            (_, _) => { 
                self.object.merge(&other.object) 
            }
        };
        Self {
            object,
            parent_id: self.parent_id.clone(),
            index: self.index.merge(&other.index)
        }
    }
}

pub struct SVGDocCrdt2 {
    replica_id: ReplicaId,
    node_map: UWMap<NodeID, LWWNodeMapItem>,
    move_history: Vec<MoveLog>,
    send_buffer: Vec<MoveLog>,
}

impl SVGDocCrdt2 {
    pub fn new(replica_id: ReplicaId) -> Self {
        Self { 
            replica_id,
            node_map: UWMap::new(), 
            move_history: Vec::new(),
            send_buffer: Vec::new()
        }
    }

    pub fn clear(&mut self) {
        self.node_map = UWMap::new();
        self.move_history = Vec::new();
        self.send_buffer = Vec::new();
    }
    
    fn is_ancestor(&self, object1_id: &str, object2_id: &str) -> bool{
        // Is object1 an ancestor of object2
        let Some(NodeMapItem { parent_id: Some(parent), .. }) = self.node_map
            .get(&object2_id.to_string()) 
            .map(|v| v.value())
            else { return false; };
        if parent == object1_id { return true; }
        return self.is_ancestor(object1_id, &parent);
    }

    fn get_children(&self, object_id: &Option<NodeID>) -> Option<Vec<(FractionalIndex, NodeID)>> {
        // Returns the children of a group node or root node.
        if let Some(object_id) = object_id {
            if let Some(NodeMapItem { object: SVGObject::Group(_), .. }) = self.node_map
                .get(object_id)
                .map(|v| v.value()) {}
            else { return None; }
        }
        let mut res = self.node_map.value()
            .iter()
            .map(|(k, v)| (k, v.value()))
            .fold(Vec::new(), |mut acc, (node_id, NodeMapItem { parent_id, index: idx, .. })| {
                if parent_id != *object_id { return acc; }
                acc.push((idx.clone(), node_id.clone()));
                acc
            });
        res.sort_by(|(idx_a, _), (idx_b, _)| idx_a.cmp(idx_b));
        Some(res)
    }

    fn get_fractional_index_insert_at(&self, parent_id: &Option<NodeID>, object_id: &NodeID, index: Option<usize>) -> Option<FractionalIndex> {
        // Returns the children of a group node or root node.
        console_log!("Target index: {:?}", index);
        let children = match self.get_children(parent_id) {
            Some(v) => v,
            None => return None,
        };
        let children = children.iter()
            .filter(|(_, node_id)| node_id != object_id)
            .collect::<Vec<_>>();
        if children.len() == 0 {
            // let val = Some(thread_rng().sample(Open01));
            return Some(FractionalIndex::default());
        }
        let generate_last = || {
            let (last, _) = children.last().unwrap();
            let res = FractionalIndex::new_after(&last);
            Some(res)
        };
        let generate_first = || {
            let (first, _) = children.first().unwrap();
            Some(FractionalIndex::new_before(&first))
        };
        let index = match index {
            Some(index) => index,
            None => {
                return generate_last()
            },
        };
        if index >= children.len() {
            let (last, _) = children.last().unwrap();
            let res = FractionalIndex::new_after(&last);
            return Some(res);
        }
        if index == 0 {
            return generate_first();
        }
        let (lower, _) = children.get(index - 1).unwrap();
        let (upper, _) = children.get(index).unwrap();
        return FractionalIndex::new_between(lower, upper);
    }

    pub fn get_group(&self, group_id: NodeID) -> Option<SVGGroup> {
        self.node_map.get(&group_id).map(|r| 
            match r.value() {
                NodeMapItem { object: SVGObject::Group(g), .. } => Some(g),
                _ => None
            }
        )
        .flatten()
    }

    pub fn get_circle(&self, circle_id: NodeID) -> Option<SVGCircle>{
        self.node_map.get(&circle_id).map(|r| 
            match r.value() {
                NodeMapItem { object: SVGObject::Circle(circle), .. } => Some(circle),
                _ => None
            }
        )
        .flatten()
    }
    
    pub fn get_rectangle(&self, rectangle_id: NodeID) -> Option<SVGRectangle> {
        self.node_map.get(&rectangle_id)
            .map(|r| 
                match r.value() {
                    NodeMapItem { object: SVGObject::Rectangle(r), .. } => Some(r),
                    _ => None
                }
            )
            .flatten()
    }

    pub fn get_path(&self, path_id: NodeID) -> Option<SVGPath> {
        self.node_map.get(&path_id)
            .map(|r| match r.value() {
                NodeMapItem { object: SVGObject::Path(p), .. } => Some(p),
                _ => None
            })
            .flatten()
    }

    pub fn add_group(
        &mut self, 
        group_id: Option<String>, 
        partial_group: PartialSVGGroup
    ) {
        if let Some(group_id) = group_id.clone() {
            match self.node_map.get(&group_id).map(|v| v.value()) {
                Some(NodeMapItem { object: SVGObject::Group(_), .. }) => {},
                _ => return,
            }
        }
        let mut group = SVGGroup::default();
        group.apply_some(partial_group);
        let new_group_id = gen_str_id();
        group.id = new_group_id.clone();
        let item = NodeMapItem {
            object: SVGObject::Group(group),
            parent_id: Some(NEW_NODE_ROOT_ID.to_string()),
            index: FractionalIndex::default()
        };
        self.node_map.insert(self.replica_id.clone(), new_group_id.clone(), item.into());
        // self.parent.insert(new_group_id.clone(), (Some(NEW_NODE_ROOT_ID.to_string()), 0.5));
        self.move_object(group_id, new_group_id, None);
    }

    pub fn add_circle(
        &mut self, 
        group_id: Option<String>, 
        partial_circle: PartialSVGCircle
    ) {
        if let Some(group_id) = group_id.clone() {
            match self.node_map.get(&group_id).map(|v| v.value()) {
                Some(NodeMapItem { object: SVGObject::Group(_), .. }) => {},
                _ => return,
            }
        }
        let mut circle = SVGCircle::default();
        circle.apply_some(partial_circle);
        let circle_id = gen_str_id();
        circle.id = circle_id.clone();
        let item = NodeMapItem {
            object: SVGObject::Circle(circle),
            parent_id: Some(NEW_NODE_ROOT_ID.to_string()),
            index: FractionalIndex::default(),
        };
        self.node_map.insert(self.replica_id.clone(), circle_id.clone(), item.into());
        self.move_object(group_id, circle_id, None);
    }

    pub fn add_rectangle(
        &mut self,
        group_id: Option<String>,
        partial_rectangle: PartialSVGRectangle
    ) {
        if let Some(group_id) = group_id.clone() {
            match self.node_map.get(&group_id).map(|v| v.value()) {
                Some(NodeMapItem { object: SVGObject::Group(_), .. }) => {},
                _ => return,
            }
        }
        let mut rectangle = SVGRectangle::default();
        rectangle.apply_some(partial_rectangle);
        let rect_id = gen_str_id();
        rectangle.id = rect_id.clone();
        let item = NodeMapItem {
            object: SVGObject::Rectangle(rectangle),
            parent_id: Some(NEW_NODE_ROOT_ID.to_string()),
            index: FractionalIndex::default()
        };
        self.node_map.insert(self.replica_id.clone(), rect_id.clone(), item.into());
        // self.parent.insert(rect_id.clone(), (Some(NEW_NODE_ROOT_ID.to_string()), 0.5));
        self.move_object(group_id, rect_id, None);
    }

    pub fn add_path(
        &mut self,
        group_id: Option<NodeID>,
        partial_path: PartialSVGPath
    ) {
        if let Some(group_id) = group_id.clone() {
            match self.node_map.get(&group_id).map(|v| v.value()) {
                Some(NodeMapItem { object: SVGObject::Group(_), .. }) => {},
                _ => return,
            }
        }
        let mut path = SVGPath::default();
        path.apply_some(partial_path);
        let path_id = gen_str_id();
        path.id = path_id.clone();
        let item = NodeMapItem {
            object: SVGObject::Path(path),
            parent_id: Some(NEW_NODE_ROOT_ID.to_string()),
            index: FractionalIndex::default()
        };
        self.node_map.insert(self.replica_id.clone(), path_id.clone(), item.into());
        self.move_object(group_id, path_id, None);
    }

    pub fn add_point_to_path(
        &mut self,
        path_id: String,
        command_type: SVGPathCommandType,
        pos: Vec2
    ) {
        let Some(NodeMapItem { object: SVGObject::Path(path), parent_id, index }) = self.node_map
            .get(&path_id)
            .map(|v| v.value())
            else { return; };
        let mut path = path.clone();
        let point_id = gen_str_id();
        match command_type {
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
            SVGPathCommandType::BEZIER_QUAD => {
                let handle = Vec2 { x: pos.x, y: pos.y + 20 };
                path.points.push(SVGPathCommand::BezierQuad { id: point_id, handle, pos });
            },
        };
        let item = NodeMapItem {
            object: SVGObject::Path(path.clone()),
            parent_id: parent_id.clone(),
            index: index.clone()
        };
        self.node_map.insert(self.replica_id.clone(), path_id, item.into());
    }

    pub fn edit_circle(&mut self, circle_id: NodeID, edits: PartialSVGCircle) {
        let Some(NodeMapItem { object: SVGObject::Circle(_), .. }) = self.node_map
            .get(&circle_id.clone()) 
            .map(|v| v.value())
            else { return; };
        // let mut circle = circle.clone();
        // circle.apply_some(edits); 
        let Some(item) = self.node_map.get(&circle_id) else { return; };
        let mut item = item.clone();
        item.update_circle(edits);
        self.node_map.insert(self.replica_id.clone(), circle_id, item);
    }

    pub fn edit_group(&mut self, group_id: NodeID, edits: PartialSVGGroup) {
        let Some(NodeMapItem { object: SVGObject::Group(_), .. }) = self.node_map
            .get(&group_id)
            .map(|v| v.value()) else { return; };
        let Some(item) = self.node_map.get(&group_id) else { return; };
        let mut item = item.clone();
        item.update_group(edits);
        self.node_map.insert(self.replica_id.clone(), group_id, item);
    }

    pub fn edit_rectangle(&mut self, rectangle_id: NodeID, edits: PartialSVGRectangle) {
        let Some(NodeMapItem { object: SVGObject::Rectangle(_), .. }) = self.node_map
            .get(&rectangle_id)
            .map(|v| v.value()) else { return; };
        // let mut rect = rect.clone();
        // rect.apply_some(edits);
        let Some(item) = self.node_map.get(&rectangle_id) else { return; };
        let mut item = item.clone();
        item.update_rectangle(edits);
        // item.update_object(SVGObject::Rectangle(rect));
        self.node_map.insert(self.replica_id.clone(), rectangle_id, item);
    }

    pub fn edit_path(&mut self, path_id: NodeID, edits: PartialSVGPath) {
        let Some(NodeMapItem { object: SVGObject::Path(_), .. }) = self.node_map
            .get(&path_id)
            .map(|v| v.value()) else { return; };
        // let mut path = path.clone();
        // path.apply_some(edits);
        let Some(item) = self.node_map.get(&path_id) else { return; };
        let mut item = item.clone();
        item.update_path(edits);
        // item.update_object(SVGObject::Path(path));
        self.node_map.insert(self.replica_id.clone(), path_id, item);
    }

    pub fn edit_path_point_type(
        &mut self, 
        path_id: NodeID, 
        point_id: NodeID, 
        command_type: SVGPathCommandType
    ) {
        let Some(NodeMapItem { object: SVGObject::Path(path), .. }) = self.node_map
            .get(&path_id)
            .map(|v| v.value()) else { return; };
        let mut points = path.points.clone();
        let idx = points.iter().position(|p| p.get_id() == &point_id);
        let Some(idx) = idx else { return; };
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
            SVGPathCommandType::BEZIER_QUAD => {
                let handle = Vec2 { x: pos.x, y: pos.y + 20 };
                SVGPathCommand::BezierQuad { id: gen_str_id(), handle, pos }
            },
        };
        match points.get_mut(idx) {
            Some(point) => {
                *point = command
            },
            _ => {}
        };
        // path.points = points;
        let Some(item) = self.node_map.get(&path_id) else { return; };
        let mut item = item.clone();
        item.update_path_points(points);
        // item.update_object(SVGObject::Path(path));
        self.node_map.insert(self.replica_id.clone(), path_id, item);
    }

    pub fn edit_path_point_pos(
        &mut self,
        path_id: NodeID,
        point_id: NodeID,
        new_pos: Vec2
    ) {
        let Some(NodeMapItem { object: SVGObject::Path(path), .. }) = self.node_map
            .get(&path_id)
            .map(|v| v.value()) else { return; };
        // let mut path = path.clone();
        let mut points = path.points.clone();
        let Some(idx) = points.iter().position(|p| p.get_id() == point_id) else {
            return;
        };
        match points.get_mut(idx) {
            Some(point) => {
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
                    _ => ()
                };
            },
            _ => {}
        };
        // path.points = points;
        let Some(item) = self.node_map.get(&path_id) else { return; };
        let mut item = item.clone();
        item.update_path_points(points);
        // item.update_object(SVGObject::Path(path));
        self.node_map.insert(self.replica_id.clone(), path_id, item);
    }

    pub fn edit_path_point_handle1(
        &mut self,
        path_id: NodeID,
        point_id: NodeID,
        new_handle1: Vec2
    ) {
        let Some(NodeMapItem { object: SVGObject::Path(path), .. }) = self.node_map
            .get(&path_id)
            .map(|v| v.value()) else { return; };
        // let mut path = path.clone();
        let mut points = path.points.clone();
        let Some(idx) = points.iter().position(|p| p.get_id() == point_id) else {
            return;
        };
        match points.get_mut(idx) {
            Some(point) => {
                match point {
                    SVGPathCommand::Bezier { handle1, .. } => {
                        *handle1 = new_handle1;
                    },
                    SVGPathCommand::BezierQuad { handle, .. } => {
                        *handle = new_handle1;
                    },
                    _ => ()
                }
            },
            _ => {}
        };
        // path.points = points;
        let Some(item) = self.node_map.get(&path_id) else { return; };
        let mut item = item.clone();
        // item.update_object(SVGObject::Path(path));
        item.update_path_points(points);
        self.node_map.insert(self.replica_id.clone(), path_id, item);
    }

    pub fn edit_path_point_handle2(
        &mut self,
        path_id: NodeID,
        point_id: NodeID,
        new_handle2: Vec2
    ) {
        let Some(NodeMapItem { object: SVGObject::Path(path), .. }) = self.node_map
            .get(&path_id)
            .map(|v| v.value()) else { return; };
        // let mut path = path.clone();
        let mut points = path.points.clone();
        let Some(idx) = points.iter().position(|p| p.get_id() == point_id) else {
            return;
        };
        match points.get_mut(idx) {
            Some(point) => {
                match point {
                    SVGPathCommand::Bezier { handle2, .. } => {
                        *handle2 = new_handle2;
                    },
                    _ => ()
                };
            },
            _ => {}
        };
        // path.points = points;
        let Some(item) = self.node_map.get(&path_id) else { return; };
        let mut item = item.clone();
        // item.update_object(SVGObject::Path(path));
        item.update_path_points(points);
        self.node_map.insert(self.replica_id.clone(), path_id, item);
    }

    pub fn remove_object(&mut self, node_id: NodeID) {
        self.node_map.remove(self.replica_id.clone(), node_id.clone());
        // let Some(group_id) = self.parent.remove(&node_id.clone()) else { return; };
    }

    pub fn remove_path_point(
        &mut self,
        path_id: NodeID,
        point_id: NodeID
    ) {
        let Some(NodeMapItem { object: SVGObject::Path(path), .. }) = self.node_map
            .get(&path_id)
            .map(|v| v.value()) else { return; };
        let mut points = path.points;
        let index = points.iter()
            .position(|o| o.get_id().eq(&point_id));
        let Some(index) = index else { return; };
        points.remove(index);
        let Some(item) = self.node_map.get(&path_id) else { return; };
        let mut item = item.clone();
        // item.update_object(SVGObject::Path(path));
        item.update_path_points(points);
        self.node_map.insert(self.replica_id.clone(), path_id, item);
    }

    pub fn move_object(&mut self, group_id: Option<NodeID>, object_id: String, index: Option<usize>) {
        let now = epoch_now_nanos();
        let Some(NodeMapItem { parent_id: old_group_id, .. }) = self.node_map
            .get(&object_id)
            .map(|v| v.value()) else { return; };
        let old_group_id = old_group_id.clone();
        let Some(_) = self.node_map.get(&object_id).map(|o| o.clone()) else { return; };
        if let Some(group_id) = group_id {
            if self.is_ancestor(&object_id, &group_id) { return; }

            // get Fractional index
            let Some(index) = self.get_fractional_index_insert_at(&Some(group_id.clone()), &object_id, index) else { return; };
            let Some(item) = self.node_map.get(&object_id) else { return; };
            let mut item = item.clone();
            item.update_parent_id(Some(group_id.clone()));
            item.update_index(index.clone());
            self.node_map.insert(self.replica_id.clone(), object_id.clone(), item);
            let move_log = MoveLog { new_group_id: Some(group_id), old_group_id, index, object_id, timestamp: now };
            self.send_buffer.push(move_log.clone());
            self.move_history.push(move_log);
            return;
        }
        // get fractional index

        let Some(index) = self.get_fractional_index_insert_at(&None, &object_id, index) else { return; };
        let Some(item) = self.node_map.get(&object_id) else { return; };
        let mut item = item.clone();
        item.update_parent_id(None);
        item.update_index(index.clone());
        self.node_map.insert(self.replica_id.clone(), object_id.clone(), item);
        let move_log = MoveLog { new_group_id: None, old_group_id, index, object_id, timestamp: now };
        self.send_buffer.push(move_log.clone());
        self.move_history.push(move_log);
    }

    fn redo_move(&mut self, MoveLog { new_group_id, index, object_id, .. }: MoveLog) {
        let Some(lww_node_map) = self.node_map
            .get(&object_id)
            else { return; };
        if let Some(new_group_id) = new_group_id.clone() {
            if self.is_ancestor(&object_id, &new_group_id) { return; }
        }
        let item = LWWNodeMapItem {
            object: lww_node_map.object.clone(),
            parent_id: LWWReg { val: new_group_id, time: lww_node_map.parent_id.time },
            index: LWWReg { val: index, time: lww_node_map.index.time }
        };
        self.node_map.insert_novtime_update(object_id, item.into());
    }

    fn undo_move(&mut self, MoveLog { old_group_id, object_id, .. }: MoveLog) {
        let Some(lww_node_map) = self.node_map
            .get(&object_id) else { return; };

        let item = LWWNodeMapItem {
            object: lww_node_map.object.clone(),
            index: LWWReg { val: FractionalIndex::default(), time: lww_node_map.index.time },
            parent_id: LWWReg { val: old_group_id, time: lww_node_map.parent_id.time }
        };
        self.node_map.insert_novtime_update(object_id, item);
    }

    fn add_to_move_log(&mut self, move_log: MoveLog) {
        let mut k = self.move_history.len();
        let move_history = self.move_history.clone();
        let mut move_already_exists = false;
        for (i, hist) in move_history.iter().enumerate().rev() {
            if hist.timestamp == move_log.timestamp { 
                move_already_exists = true;
                break;
             }
            if hist.timestamp < move_log.timestamp {
                break
            }
            self.undo_move(hist.clone());
            k = i;
        }
        if !move_already_exists {
            self.move_history.insert(k, move_log);
        }
        let n = self.move_history.len();
        while k < n {
            self.redo_move(self.move_history[k].clone());
            k += 1;
        }
    }

    fn merge_aux(&mut self, other_node_map: UWMap<NodeID, LWWNodeMapItem>, mut move_logs: Vec<MoveLog>) {
        self.node_map = UWMap::merge(&self.node_map, &other_node_map);
        for log in move_logs.drain(..) {
            self.add_to_move_log(log);
        }
    }

    fn broadcast_aux(&mut self) -> (UWMap<String, LWWNodeMapItem>, Vec<MoveLog>){
        let res = (self.node_map.clone(), self.send_buffer.clone());
        self.send_buffer.clear();
        res
    }

    pub fn broadcast(&mut self) -> String {
        let tup = self.broadcast_aux();
        serde_json::to_string(&tup).unwrap()
    }

    pub fn merge(&mut self, data: String) {
        type Tup = (UWMap<String, LWWNodeMapItem>, Vec<MoveLog>);
        let tup = serde_json::from_str::<Tup>(&data).ok();
        let Some((other_node_map, move_logs)) = tup else { return; };
        self.merge_aux(other_node_map, move_logs);
    }

    pub fn save(&self) -> String {
        type Tup = (UWMap<String, LWWNodeMapItem>, Vec<MoveLog>);
        let node_map = self.node_map.clone();
        let move_history = self.move_history.clone();
        let tup: Tup = (node_map, move_history);
        serde_json::to_string(&tup).unwrap()
    }

    pub fn load(&mut self, data: String) {
        self.clear();
        type Tup = (UWMap<String, LWWNodeMapItem>, Vec<MoveLog>);
        let tup = serde_json::from_str::<Tup>(&data).ok();
        let Some((other_node_map, move_logs)) = tup else { return; };
        self.merge_aux(other_node_map, move_logs);
    }

    fn dfs(&self, object_id: &NodeID, visited: &mut HashMap<NodeID, bool>, res: &mut Vec<NodeID>) {
        let is_visited = match visited.get(object_id) {
            Some(t) => t.clone(),
            None => false,
        };
        if is_visited { return; }
        visited.insert(object_id.clone(), true);
        match self.node_map.get(object_id).map(|v| v.value()) {
            Some(NodeMapItem { parent_id: Some(p), .. }) => {
                self.dfs(&p, visited, res);
                res.push(object_id.clone());
            },
            Some(NodeMapItem { parent_id: None, .. }) => {
                res.push(object_id.clone());
            },
            _ => return,
        };
    }

    fn top_sort_nodes(&self) -> Vec<NodeID> {
        let mut res = Vec::new();
        let mut visited = HashMap::new();
        let nodes = self.node_map.value();
        for object_id in nodes.keys() {
            self.dfs(object_id, &mut visited, &mut res);
        }
        res
    }


    pub fn tree(&self) -> SVGDocTree {
        let mut res = SVGDocTree::new();
        let mut node_map = self.node_map.value();
        let mut nodes = self.top_sort_nodes();
        nodes.reverse();
        console_log!("[BRANCHES] Rendering tree");
        // Apply index according to timestamp
        for node in nodes.iter() {
            let Some(NodeMapItem { parent_id: Some(group_id), index:  idx, .. }) = self.node_map.get(node).map(|v| v.value()) else { continue; };
            match node_map.remove(&group_id).map(|v| v.value()) {
                Some(NodeMapItem { object: SVGObject::Group(g), parent_id: g_parent_id, index: g_index }) => {
                    let mut group = g;
                    let mut i = group.children.len();
                    loop {
                        if i == 0 { break; }
                        let next_id = group.children[i - 1].get_id();
                        let NodeMapItem { index: nxt_idx, .. } = self.node_map.get(&next_id.to_string())
                            .map(|v| v.value())
                            .expect("node is missing in parent");
                        if nxt_idx < idx { break; }
                        i -= 1;
                    }
                    let Some(NodeMapItem { object, .. }) = node_map.remove(node).map(|v| v.value()) else { 
                        let item = NodeMapItem {
                            object: SVGObject::Group(group),
                            parent_id: g_parent_id.clone(),
                            index: g_index.clone()
                        };
                        node_map.insert(group_id.clone(), item.into());
                        continue;
                    };
                    group.children.insert(i, object);
                    let item = NodeMapItem {
                        object: SVGObject::Group(group),
                        parent_id: g_parent_id.clone(),
                        index: g_index.clone()
                    };
                    node_map.insert(group_id.clone(), item.into());
                },
                Some(o) => {
                    node_map.insert(group_id.clone(), o.into());
                },
                _ => {
                    node_map.remove(node);
                },
            }
        };
        console_log!("[BASE] Rendering tree");
        let mut tmp = node_map.drain().map(|(_, o)| o.value()).collect::<Vec<_>>();
        tmp.sort_by(|NodeMapItem { index: index_a, .. }, NodeMapItem { index: index_b, .. }| {
            index_a.cmp(index_b)
        });
        res.children = tmp.drain(..).map(|NodeMapItem { object, .. }| object).collect();
        console_log!("[BASE] Finished");
        res
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    fn merge_docs(doc1: &mut SVGDocCrdt2, doc2: &mut SVGDocCrdt2) {
        let (d1_node_map, d1_move_logs) = doc1.broadcast_aux();
        let (d2_node_map, d2_move_logs) = doc2.broadcast_aux();
        doc2.merge_aux(d1_node_map, d1_move_logs);
        doc1.merge_aux(d2_node_map, d2_move_logs);

        let t1 = doc1.tree();
        let t2 = doc2.tree();

        let t1_str = serde_json::to_string(&t1).unwrap();
        let t2_str = serde_json::to_string(&t2).unwrap();
        assert_eq!(t1_str, t2_str);
    }

    #[test]
    fn test_create_circle() {
        let r1 = "r1";
        let mut doc = SVGDocCrdt2::new(r1.to_string());
        doc.add_circle(None, PartialSVGCircle::empty());
        let tree = doc.tree();
        assert_eq!(tree.children.len(), 1);
        match tree.children.get(0) {
            Some(SVGObject::Circle(_)) => { },
            _ => {
                panic!("Circle should be at index 0");
            }
        }
    }

    #[test]
    fn test_create_group() {
        let r1 = "r1";
        let mut doc = SVGDocCrdt2::new(r1.to_string());
        doc.add_group(None, PartialSVGGroup::empty());
        let tree = doc.tree();
        assert_eq!(tree.children.len(), 1);
        match tree.children.get(0) {
            Some(SVGObject::Group(_)) => {},
            _ => {
                panic!("Group should be at index 0");
            }
        }
    }

    #[test]
    fn test_create_circle_within_group() {
        let r1 = "r1";
        let mut doc = SVGDocCrdt2::new(r1.to_string());
        doc.add_group(None, PartialSVGGroup::empty());
        let tree = doc.tree();
        let group_id = match tree.children.get(0) {
            Some(SVGObject::Group(g)) => &g.id,
            _ => {
                panic!("Group should be at index 0");
            }
        };
        doc.add_circle(Some(group_id.clone()), PartialSVGCircle::empty());
        doc.add_circle(Some(group_id.clone()), PartialSVGCircle::empty());
        let tree = doc.tree();
        let group = match tree.children.get(0) {
            Some(SVGObject::Group(g)) => g,
            _ => {
                panic!("Group should be at index 0")
            }
        };
        match group.children.get(0) {
            Some(SVGObject::Circle(_)) => {},
            _ => {
                panic!("Circle should be within group at index 0");
            }
        };
        match group.children.get(1) {
            Some(SVGObject::Circle(_)) => {},
            _ => {
                panic!("Circle should be within group at index 1")
            }
        }
    }

    #[test]
    fn test_no_cycle() {
        let r1 = "r1";
        let mut doc = SVGDocCrdt2::new(r1.to_string());
        doc.add_group(None, PartialSVGGroup::empty());
        let tree = doc.tree();
        let group_id = match tree.children.get(0) {
            Some(SVGObject::Group(g)) => &g.id,
            _ => panic!("Group should exist at index 0")
        };
        doc.add_group(Some(group_id.clone()), PartialSVGGroup::empty());
        let tree = doc.tree();
        let group = match tree.children.get(0) {
            Some(SVGObject::Group(g)) => g,
            _ => panic!("Group should exist at index 0")
        };
        let child_group_id = match group.children.get(0) {
            Some(SVGObject::Group(g)) => &g.id,
            _ => panic!("Group should exist within group at index 0")
        };
        doc.move_object(Some(child_group_id.to_string()), group_id.clone(), Some(0));
        let tree = doc.tree();
        assert_eq!(tree.children.len(), 1);
        let group = match tree.children.get(0) {
            Some(SVGObject::Group(g)) => g,
            _ => panic!("Group should exist at index 0")
        };
        assert_eq!(group.children.len(), 1);
        match group.children.get(0) {
            Some(SVGObject::Group(_)) => {},
            _ => panic!("Inner group child should exist")
        };
    }

    #[test]
    fn test_move() {
        let r1 = "r1";
        let mut doc = SVGDocCrdt2::new(r1.to_string());
        doc.add_circle(None, PartialSVGCircle::empty());
        doc.add_group(None, PartialSVGGroup::empty());
        let tree = doc.tree();
        assert_eq!(tree.children.len(), 2);
        let circle_id = match tree.children.get(0) {
            Some(SVGObject::Circle(circle)) => &circle.id,
            _ => panic!("Circle should exist at index 0")
        };
        let group_id = match tree.children.get(1) {
            Some(SVGObject::Group(group)) => &group.id,
            _ => panic!("Group should exist at index 1")
        };
        doc.move_object(Some(group_id.clone()), circle_id.clone(), None);
        let tree = doc.tree();
        assert_eq!(tree.children.len(), 1);
        let group = match tree.children.get(0) {
            Some(SVGObject::Group(group)) => group,
            _ => panic!("Group should exist at index 0")
        };
        assert_eq!(group.children.len(), 1);
        match group.children.get(0) {
            Some(SVGObject::Circle(_)) => {},
            _ => panic!("Circle should exist at index 0")
        };
    }

    #[test]
    fn test_move_order() {
        let r1 = "r1";
        let mut doc = SVGDocCrdt2::new(r1.to_string());
        doc.add_circle(None, PartialSVGCircle::empty());
        doc.add_circle(None, PartialSVGCircle::empty());
        let tree = doc.tree();
        assert_eq!(tree.children.len(), 2);
        let first_id = match tree.children.get(0) {
            Some(SVGObject::Circle(circle)) => &circle.id,
            _ => panic!("Circle should exist at index 0")
        };
        let second_id = match tree.children.get(1) {
            Some(SVGObject::Circle(circle)) => &circle.id,
            _ => panic!("Circle should exist at index 1")
        };
        assert_ne!(first_id, second_id);
        doc.move_object(None, first_id.clone(), Some(1));
        let tree = doc.tree();
        assert_eq!(tree.children.len(), 2);
        let act_first_id = match tree.children.get(0) {
            Some(SVGObject::Circle(circle)) => &circle.id,
            _ => panic!("Circle should exist at index 0")
        };
        let act_second_id = match tree.children.get(1) {
            Some(SVGObject::Circle(circle)) => &circle.id,
            _ => panic!("Circle should exist at index 1"),
        };
        assert_eq!(second_id, act_first_id);
        assert_eq!(first_id, act_second_id);
    }

    #[test]
    fn test_move_order_multiple() {
        let r1 = "r1";
        let mut doc = SVGDocCrdt2::new(r1.to_string());
        doc.add_circle(None, PartialSVGCircle::empty());
        doc.add_circle(None, PartialSVGCircle::empty());
        doc.add_circle(None, PartialSVGCircle::empty());
        doc.add_circle(None, PartialSVGCircle::empty());
        doc.add_circle(None, PartialSVGCircle::empty());
        let tree = doc.tree();
        let first_id = match tree.children.get(0) {
            Some(SVGObject::Circle(circle)) => &circle.id,
            _ => panic!("Circle should exist at index 0")
        };
        let second_id = match tree.children.get(1) {
            Some(SVGObject::Circle(circle)) => &circle.id,
            _ => panic!("Circle should exist at index 1")
        };
        let third_id = match tree.children.get(2) {
            Some(SVGObject::Circle(circle)) => &circle.id,
            _ => panic!("Circle should exist at index 2")
        };
        let fourth_id = match tree.children.get(3) {
            Some(SVGObject::Circle(circle)) => &circle.id,
            _ => panic!("Circle should exist at index 3")
        };
        let fifth_id = match tree.children.get(4) {
            Some(SVGObject::Circle(circle)) => &circle.id,
            _ => panic!("Circle should exist at index 4")
        };
        doc.move_object(None, first_id.clone(), Some(1));
        doc.move_object(None, third_id.clone(), Some(0));
        doc.move_object(None, fifth_id.clone(), Some(1));
        doc.move_object(None, fourth_id.clone(), Some(1));
        let tree = doc.tree();
        let act_first_id = match tree.children.get(0) {
            Some(SVGObject::Circle(circle)) => &circle.id,
            _ => panic!("Circle should exist at index 0")
        };
        let act_second_id = match tree.children.get(1) {
            Some(SVGObject::Circle(circle)) => &circle.id,
            _ => panic!("Circle should exist at index 1")
        };
        let act_third_id = match tree.children.get(2) {
            Some(SVGObject::Circle(circle)) => &circle.id,
            _ => panic!("Circle should exist at index 2")
        };
        let act_fourth_id = match tree.children.get(3) {
            Some(SVGObject::Circle(circle)) => &circle.id,
            _ => panic!("Circle should exist at index 3")
        };
        let act_fifth_id = match tree.children.get(4) {
            Some(SVGObject::Circle(circle)) => &circle.id,
            _ => panic!("Circle should exist at index 4")
        };
        assert_eq!(third_id, act_first_id);
        assert_eq!(fourth_id, act_second_id);
        assert_eq!(fifth_id, act_third_id);
        assert_eq!(second_id, act_fourth_id);
        assert_eq!(first_id, act_fifth_id);
    }

    #[test]
    fn test_merge_create_one_circle() {
        let r1 = "r1";
        let r2 = "r2";
        let mut doc1 = SVGDocCrdt2::new(r1.to_string());
        let mut doc2 = SVGDocCrdt2::new(r2.to_string());
        doc1.add_circle(None, PartialSVGCircle::empty());
        doc2.add_circle(None, PartialSVGCircle::empty());
        let (d1_node_map, d1_move_logs) = doc1.broadcast_aux();
        let (d2_node_map, d2_move_logs) = doc2.broadcast_aux();
        doc2.merge_aux(d1_node_map, d1_move_logs);
        doc1.merge_aux(d2_node_map, d2_move_logs);
        let t1 = doc1.tree();
        let t2 = doc2.tree();
        let t1 = serde_json::to_string(&t1).unwrap();
        let t2 = serde_json::to_string(&t2).unwrap();
        assert_eq!(t1, t2);
    }

    #[test]
    fn test_merge_move_order_multiple() {
        let r1 = "r1";
        let r2 = "r2";
        let mut doc1 = SVGDocCrdt2::new(r1.to_string());
        let mut doc2 = SVGDocCrdt2::new(r2.to_string());
        doc1.add_circle(None, PartialSVGCircle::empty());
        doc1.add_circle(None, PartialSVGCircle::empty());
        doc1.add_circle(None, PartialSVGCircle::empty());
        let first_id = match doc1.tree().children.get(0) {
            Some(SVGObject::Circle(circle)) => circle.id.clone(),
            _ => panic!("Circle should exist at index 0")
        };
        let second_id = match doc1.tree().children.get(1) {
            Some(SVGObject::Circle(circle)) => circle.id.clone(),
            _ => panic!("Circle should exist at index 1")
        };
        let third_id = match doc1.tree().children.get(2) {
            Some(SVGObject::Circle(circle)) => circle.id.clone(),
            _ => panic!("Circle should exist at index 2")
        };

        let (r1_node_map, r1_move_history) = doc1.broadcast_aux();
        doc2.merge_aux(r1_node_map, r1_move_history);
        doc2.move_object(None, first_id.clone(), Some(2));
        doc1.move_object(None, second_id.clone(), Some(0));

        let (r1_node_map, r1_move_history) = doc1.broadcast_aux();
        let (r2_node_map, r2_move_history) = doc2.broadcast_aux();
        doc1.merge_aux(r2_node_map, r2_move_history);
        doc2.merge_aux(r1_node_map, r1_move_history);

        let t1 = doc1.tree();
        let t2 = doc2.tree();
        let act_first_id = match doc1.tree().children.get(0) {
            Some(SVGObject::Circle(circle)) => circle.id.clone(),
            _ => panic!("Circle should be at index 0")
        };
        let act_second_id = match doc1.tree().children.get(1) {
            Some(SVGObject::Circle(circle)) => circle.id.clone(),
            _ => panic!("Circle should be at index 1")
        };
        let act_third_id = match doc1.tree().children.get(2) {
            Some(SVGObject::Circle(circle)) => circle.id.clone(),
            _ => panic!("Circle should be at index 2")
        };
        assert_eq!(act_first_id, second_id);
        assert_eq!(act_second_id, third_id);
        assert_eq!(act_third_id, first_id);
        let t1_str = serde_json::to_string(&t1).unwrap();
        let t2_str = serde_json::to_string(&t2).unwrap();
        assert_eq!(t1_str, t2_str);
    }

    #[test]
    fn test_move_conflict() {
        let r1 = "r1";
        let r2 = "r2";
        let mut doc1 = SVGDocCrdt2::new(r1.to_string());
        let mut doc2 = SVGDocCrdt2::new(r2.to_string());
        doc1.add_circle(None, PartialSVGCircle::empty());
        doc1.add_circle(None, PartialSVGCircle::empty());
        doc1.add_circle(None, PartialSVGCircle::empty());
        let first_id = match doc1.tree().children.get(0) {
            Some(SVGObject::Circle(circle)) => circle.id.clone(),
            _ => panic!("Circle should exist at index 0")
        };
        let second_id = match doc1.tree().children.get(1) {
            Some(SVGObject::Circle(circle)) => circle.id.clone(),
            _ => panic!("Circle should exist at index 1")
        };
        let third_id = match doc1.tree().children.get(2) {
            Some(SVGObject::Circle(circle)) => circle.id.clone(),
            _ => panic!("Circle should exist at index 2")
        };

        let (r1_node_map, r1_move_history) = doc1.broadcast_aux();
        doc2.merge_aux(r1_node_map, r1_move_history);
        let mut edits = PartialSVGCircle::empty();
        edits.opacity = Some(0.5);
        doc2.move_object(None, first_id.clone(), Some(2));
        doc2.edit_circle(first_id.clone(), edits);
        doc1.move_object(None, first_id.clone(), Some(1));

        let (r1_node_map, r1_move_history) = doc1.broadcast_aux();
        let (r2_node_map, r2_move_history) = doc2.broadcast_aux();
        doc1.merge_aux(r2_node_map, r2_move_history);
        doc2.merge_aux(r1_node_map, r1_move_history);

        let t1 = doc1.tree();
        let t2 = doc2.tree();
        let act_first_id = match doc1.tree().children.get(0) {
            Some(SVGObject::Circle(circle)) => circle.id.clone(),
            _ => panic!("Circle should be at index 0")
        };
        let act_second_id = match doc1.tree().children.get(1) {
            Some(SVGObject::Circle(circle)) => circle.id.clone(),
            _ => panic!("Circle should be at index 1")
        };
        let act_third_id = match doc1.tree().children.get(2) {
            Some(SVGObject::Circle(circle)) => circle.id.clone(),
            _ => panic!("Circle should be at index 2")
        };
        assert_eq!(act_first_id, second_id);
        assert_eq!(act_second_id, first_id);
        assert_eq!(act_third_id, third_id);
        match doc1.get_circle(first_id.clone()) {
            Some(circle) => {
                assert_eq!(circle.opacity, 0.5);
            },
            _ => panic!("Circle should exist")
        };
        let t1_str = serde_json::to_string(&t1).unwrap();
        let t2_str = serde_json::to_string(&t2).unwrap();
        assert_eq!(t1_str, t2_str);
    }

    #[test]
    fn test_delete() {
        let r1 = "r1".to_string();
        let mut doc1 = SVGDocCrdt2::new(r1.clone());
        doc1.add_circle(None, PartialSVGCircle::empty());
        let tree = doc1.tree();
        assert_eq!(tree.children.len(), 1);
        let circle_id = match tree.children.get(0) {
            Some(SVGObject::Circle(circle)) => circle.id.clone(),
            _ => panic!("Circle should exist at index 0")
        };
        doc1.remove_object(circle_id);
        let tree = doc1.tree();
        assert_eq!(tree.children.len(), 0);
    }

    #[test]
    fn test_merge_delete_move() {
        let r1 = "r1".to_string();
        let r2 = "r2".to_string();

        let mut doc1 = SVGDocCrdt2::new(r1.clone());
        let mut doc2 = SVGDocCrdt2::new(r2.clone());

        doc1.add_circle(None, PartialSVGCircle::empty());

        merge_docs(&mut doc1, &mut doc2);

        let t2 = doc2.tree();
        let circle_id = match t2.children.get(0) {
            Some(SVGObject::Circle(circle)) => circle.id.clone(),
            _ => panic!("Circle should exist at index 0")
        };

        let mut edits = PartialSVGCircle::empty();
        edits.opacity = Some(0.5);
        doc2.edit_circle(circle_id.clone(), edits);
        doc1.remove_object(circle_id);

        merge_docs(&mut doc1, &mut doc2);

        let t1 = doc1.tree();
        let t2 = doc2.tree();

        match t1.children.get(0) {
            Some(SVGObject::Circle(circle)) => {
                assert_eq!(circle.opacity, 0.5, "Circle opacity should be 0.5");
            },
            _ => panic!("Circle should exist at index 0")
        };

        let t1 = serde_json::to_string(&t1).unwrap();
        let t2 = serde_json::to_string(&t2).unwrap();

        assert_eq!(t1, t2);
    }

    #[test]
    fn test_delete_group_with_children() {
        let r1 = "r1".to_string();

        let mut doc1 = SVGDocCrdt2::new(r1);
        doc1.add_group(None, PartialSVGGroup::empty());
        let tree = doc1.tree();
        let group_id = match tree.children.get(0) {
            Some(SVGObject::Group(g)) => g.id.clone(),
            _ => panic!("Group should exist")
        };
        doc1.add_circle(Some(group_id.clone()), PartialSVGCircle::empty());

        // ensure that circle is inside group.
        let tree = doc1.tree();
        let group = match tree.children.get(0) {
            Some(SVGObject::Group(g)) => g,
            _ => panic!("Group should exist")
        };
        let _ = match group.children.get(0) {
            Some(SVGObject::Circle(c)) => c.id.clone(),
            _ => panic!("Circle should exist")
        };
        doc1.remove_object(group_id.clone());
        let tree = doc1.tree();
        assert_eq!(tree.children.len(), 0);
    }

    #[test]
    fn test_case_1_edit_different_attributes_circle() {
        let r1 = "r1".to_string();
        let r2 = "r2".to_string();

        let mut doc1 = SVGDocCrdt2::new(r1);
        let mut doc2 = SVGDocCrdt2::new(r2);

        doc1.add_circle(None, PartialSVGCircle::empty());
        merge_docs(&mut doc1, &mut doc2);

        let t1 = doc1.tree();
        let circle_id = match t1.children.get(0) {
            Some(SVGObject::Circle(c)) => &c.id,
            _ => panic!("Circle should exist")
        };

        let mut edits1 = PartialSVGCircle::empty();
        edits1.opacity = Some(0.5);
        doc1.edit_circle(circle_id.to_string(), edits1);

        let mut edits2 = PartialSVGCircle::empty();
        edits2.radius = Some(100);

        doc2.edit_circle(circle_id.to_string(), edits2);

        merge_docs(&mut doc1, &mut doc2);

        let tree = doc1.tree();
        match tree.children.get(0) {
            Some(SVGObject::Circle(c)) => {
                assert_eq!(c.opacity, 0.5);
                assert_eq!(c.radius, 100);
            },
            _ => panic!("Circle should exist")
        };
    }

    #[test]
    fn test_case_1_edit_different_attributes_rect() {
        let r1 = "r1".to_string();
        let r2 = "r2".to_string();

        let mut doc1 = SVGDocCrdt2::new(r1);
        let mut doc2 = SVGDocCrdt2::new(r2);

        doc1.add_rectangle(None, PartialSVGRectangle::empty());
        merge_docs(&mut doc1, &mut doc2);

        let t1 = doc1.tree();
        let rect_id = match t1.children.get(0) {
            Some(SVGObject::Rectangle(r)) => &r.id,
            _ => panic!("Rectangle should exist")
        };

        let mut edits1 = PartialSVGRectangle::empty();
        edits1.opacity = Some(0.5);
        doc1.edit_rectangle(rect_id.to_string(), edits1);

        let mut edits2 = PartialSVGRectangle::empty();
        edits2.width = Some(100);

        doc2.edit_rectangle(rect_id.to_string(), edits2);
        merge_docs(&mut doc1, &mut doc2);

        let tree = doc1.tree();
        match tree.children.get(0) {
            Some(SVGObject::Rectangle(c)) => {
                assert_eq!(c.opacity, 0.5);
                assert_eq!(c.width, 100);
            },
            _ => panic!("Rectangle should exist")
        };
    }

    #[test]
    fn test_case_1_edit_different_attributes_path() {
        let r1 = "r1".to_string();
        let r2 = "r2".to_string();

        let mut doc1 = SVGDocCrdt2::new(r1);
        let mut doc2 = SVGDocCrdt2::new(r2);

        doc1.add_path(None, PartialSVGPath::empty());
        merge_docs(&mut doc1, &mut doc2);

        let t1 = doc1.tree();

        let path_id = t1.children.get(0)
            .map(|it| { let SVGObject::Path(o) = it else { return None; };
                Some(&o.id)
            })
            .flatten()
            .expect("Path should exist");
        
        let mut edits1 = PartialSVGPath::empty();
        edits1.opacity = Some(0.5);
        doc1.edit_path(path_id.to_string(), edits1);

        let mut edits2 = PartialSVGPath::empty();
        edits2.stroke_width = Some(100);
        doc2.edit_path(path_id.to_string(), edits2);

        merge_docs(&mut doc1, &mut doc2);

        let tree = doc1.tree();
        let path = tree.children.get(0)
            .map(|it| {
                let SVGObject::Path(o) = it else { return None; };
                Some(o)  
            })
            .flatten()
            .expect("Path should exist");
        assert_eq!(path.opacity, 0.5);
        assert_eq!(path.stroke_width, 100);
    }

    #[test]
    fn test_case_1_edit_different_attributes_group() {
        let r1 = "r1".to_string();
        let r2 = "r2".to_string();

        let mut doc1 = SVGDocCrdt2::new(r1);
        let mut doc2 = SVGDocCrdt2::new(r2);

        doc1.add_group(None, PartialSVGGroup::empty());
        merge_docs(&mut doc1, &mut doc2);

        let t1 = doc1.tree();

        let group_id = t1.children.get(0)
            .map(|it| {
                let SVGObject::Group(o) = it else { return None; };
                Some(&o.id)
            })
            .flatten()
            .expect("Group should exist");
        
        let mut edits1 = PartialSVGGroup::empty();
        edits1.opacity = Some(JSNullable::Some { item: 0.5 });
        doc1.edit_group(group_id.to_string(), edits1);

        let mut edits2 = PartialSVGGroup::empty();
        edits2.stroke_width = Some(JSNullable::Some { item: 100 });
        doc2.edit_group(group_id.to_string(), edits2);

        merge_docs(&mut doc1, &mut doc2);

        let tree = doc1.tree();
        let group = tree.children.get(0)
            .map(|it| {
                let SVGObject::Group(o) = it else { return None; };
                Some(o)  
            })
            .flatten()
            .expect("Group should exist");
        assert_eq!(group.opacity, Some(0.5));
        assert_eq!(group.stroke_width, Some(100));
    }

    #[test]
    fn test_case_2_concurrent_edit_and_delete() {
        let r1 = "r1".to_string();
        let r2 = "r2".to_string();

        let mut doc1 = SVGDocCrdt2::new(r1);
        let mut doc2 = SVGDocCrdt2::new(r2);

        doc1.add_circle(None, PartialSVGCircle::empty());

        merge_docs(&mut doc1, &mut doc2);

        let t1 = doc1.tree();
        let circle_id = t1.children.get(0)
            .map(|it| if let SVGObject::Circle(o) = it {
                Some(&o.id)
            } else {
                None 
            })
            .flatten()
            .expect("Circle should exist");


        // Client 1 is editing the circle
        let mut edits = PartialSVGCircle::empty();
        edits.opacity = Some(0.5);
        doc1.edit_circle(circle_id.clone(), edits);

        // Client 2 is removing the circle
        doc2.remove_object(circle_id.clone());

        merge_docs(&mut doc1, &mut doc2);

        let t1 = doc2.tree();
        let circle = t1.children.get(0)
            .map(|it| if let SVGObject::Circle(o) = it {
                Some(o)
            } else {
                None 
            })
            .flatten()
            .expect("Circle should exist");
        assert_eq!(circle.opacity, 0.5);
    }

    #[test]
    fn test_case_2_concurrent_move_and_delete() {
        let r1 = "r1".to_string();
        let r2 = "r2".to_string();

        let mut doc1 = SVGDocCrdt2::new(r1);
        let mut doc2 = SVGDocCrdt2::new(r2);

        doc1.add_group(None, PartialSVGGroup::empty());
        doc1.add_circle(None, PartialSVGCircle::empty());

        merge_docs(&mut doc1, &mut doc2);

        let tree = doc1.tree();

        let group_id = tree.children.get(0)
            .map(|it| if let SVGObject::Group(o) = it {
                Some(&o.id)
            } else {
                None 
            })
            .flatten()
            .expect("Group should exist");
        
        let circle_id = tree.children.get(1)
            .map(|it| if let SVGObject::Circle(o) = it {
                Some(&o.id)
            } else {
                None 
            })
            .flatten()
            .expect("Circle should exist");

        // client 1 is moving the object into a group
        doc1.move_object(Some(group_id.to_string()), circle_id.to_string(), None);

        // client 2 is deleting the circle
        doc2.remove_object(circle_id.to_string());

        merge_docs(&mut doc1, &mut doc2);

        let tree = doc1.tree();

        let group = tree.children.get(0)
            .map(|it| if let SVGObject::Group(o) = it {
                Some(o)
            } else {
                None
            })
            .flatten()
            .expect("Group should exist");
            
        let circle = group.children.get(0)
            .map(|it| if let SVGObject::Circle(o) = it {
                Some(o)
            } else {
                None
            })
            .flatten()
            .expect("Circle should exist");

        assert_eq!(circle_id, &circle.id)
    }

    #[test]
    fn test_case_3_concurrent_move_to_different_groups() {
        let r1 = "r1".to_string();
        let r2 = "r2".to_string();

        let mut doc1 = SVGDocCrdt2::new(r1);
        let mut doc2 = SVGDocCrdt2::new(r2);

        doc1.add_group(None, PartialSVGGroup::empty());
        doc1.add_group(None, PartialSVGGroup::empty());
        doc1.add_rectangle(None, PartialSVGRectangle::empty());

        merge_docs(&mut doc1, &mut doc2);

        let tree = doc1.tree();
        let group1_id = tree.children.get(0)
            .map(|it| if let SVGObject::Group(o) = it {
                Some(&o.id)
            } else {
                None
            })
            .flatten()
            .expect("Group should exist");

        let group2_id = tree.children.get(1)
            .map(|it| if let SVGObject::Group(o) = it {
                Some(&o.id)
            } else {
                None
            })
            .flatten()
            .expect("Group should exist");

        let rect_id = tree.children.get(2)
            .map(|it| if let SVGObject::Rectangle(o) = it {
                Some(&o.id)
            } else {
                None
            })
            .flatten()
            .expect("Rectangle should exist");

        doc1.move_object(Some(group1_id.to_string()), rect_id.to_string(), None);
        doc2.move_object(Some(group2_id.to_string()), rect_id.to_string(), None);

        merge_docs(&mut doc1, &mut doc2);

        let tree = doc1.tree();

        let group1 = tree.children.get(0)
            .map(|it| if let SVGObject::Group(o) = it {
                Some(o)
            } else {
                None
            })
            .flatten()
            .expect("Group should exist");
        let group2 = tree.children.get(1)
            .map(|it| if let SVGObject::Group(o) = it {
                Some(o)
            } else {
                None
            })
            .flatten()
            .expect("Group should exist");


        let rect = group2.children.get(0)
            .map(|it| if let SVGObject::Rectangle(o) = it {
                Some(o)
            } else {
                None
            })
            .flatten()
            .expect("Rectangle should exist");
        
        assert_eq!(group1.children.len(), 0);
        assert_eq!(group2.children.len(), 1);
        assert_eq!(&rect.id, rect_id);
        assert_eq!(tree.children.len(), 2);
    }

}