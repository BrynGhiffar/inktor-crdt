use std::collections::hash_map::Entry;

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveLog {
    old_group_id: Option<NodeID>,
    new_group_id: Option<NodeID>,
    object_id: NodeID,
    index: usize,
    timestamp: UnixEpochTimeNanos
}

static NEW_NODE_ROOT_ID: &'static str = "NEW_NODES_ROOT_ID";

// pub struct MoveOp {
//     group_id: Option<NodeID>,
//     object_id: NodeID,
//     timestamp: UnixEpochTimeNanos,
//     index: usize,
// }

pub struct SVGDocCrdt2 {
    replica_id: ReplicaId,
    node_map: UWMap<NodeID, SVGObject>,
    parent: HashMap<NodeID, (Option<NodeID>, usize, UnixEpochTimeNanos)>,
    ordering: HashMap<Option<NodeID>, Vec<NodeID>>,
    move_history: Vec<MoveLog>,
    send_buffer: Vec<MoveLog>,
}

impl SVGDocCrdt2 {
    pub fn new(replica_id: ReplicaId) -> Self {
        Self { 
            replica_id,
            node_map: UWMap::new(), 
            parent: HashMap::new(), 
            ordering: HashMap::new(),
            move_history: Vec::new(),
            send_buffer: Vec::new()
        }
    }

    pub fn clear(&mut self) {
        self.node_map = UWMap::new();
        self.parent = HashMap::new();
        self.move_history = Vec::new();
        self.send_buffer = Vec::new();
    }
    
    fn is_ancestor(&self, object1_id: &str, object2_id: &str) -> bool{
        // Is object1 an ancestor of object2
        let Some((Some(parent), _, _)) = self.parent.get(object2_id) else { return false; };
        if parent == object1_id { return true; }
        return self.is_ancestor(object1_id, parent);
    }

    pub fn get_group(&self, group_id: NodeID) -> Option<SVGGroup> {
        self.node_map.get(&group_id).map(|r| 
            match r.clone() {
                SVGObject::Group(g) => Some(g),
                _ => None
            }
        )
        .flatten()
    }

    pub fn get_circle(&self, circle_id: NodeID) -> Option<SVGCircle>{
        self.node_map.get(&circle_id).map(|r| 
            match r.clone() {
                SVGObject::Circle(circle) => Some(circle),
                _ => None
            }
        )
        .flatten()
    }
    
    pub fn get_rectangle(&self, rectangle_id: NodeID) -> Option<SVGRectangle> {
        self.node_map.get(&rectangle_id)
            .map(|r| 
                match r.clone() {
                    SVGObject::Rectangle(r) => Some(r),
                    _ => None
                }
            )
            .flatten()
    }

    pub fn get_path(&self, path_id: NodeID) -> Option<SVGPath> {
        self.node_map.get(&path_id)
            .map(|r| match r.clone() {
                SVGObject::Path(p) => Some(p),
                _ => None
            })
            .flatten()
    }

    pub fn add_group(
        &mut self, 
        group_id: Option<String>, 
        partial_group: PartialSVGGroup
    ) {
        let mut group = SVGGroup::default();
        group.apply_some(partial_group);
        let new_group_id = gen_str_id();
        group.id = new_group_id.clone();
        self.node_map.insert(self.replica_id.clone(), new_group_id.clone(), SVGObject::Group(group));
        self.parent.insert(new_group_id.clone(), (Some(NEW_NODE_ROOT_ID.to_string()), 0, epoch_now_nanos()));
        self.move_object(group_id, new_group_id, None);
    }

    pub fn add_circle(
        &mut self, 
        group_id: Option<String>, 
        partial_circle: PartialSVGCircle
    ) {
        let mut circle = SVGCircle::default();
        circle.apply_some(partial_circle);
        let circle_id = gen_str_id();
        circle.id = circle_id.clone();
        self.node_map.insert(self.replica_id.clone(), circle_id.clone(), SVGObject::Circle(circle));
        self.parent.insert(circle_id.clone(), (Some(NEW_NODE_ROOT_ID.to_string()), 0, epoch_now_nanos()));
        self.move_object(group_id, circle_id, None);
    }

    pub fn add_rectangle(
        &mut self,
        group_id: Option<String>,
        partial_rectangle: PartialSVGRectangle
    ) {
        let mut rectangle = SVGRectangle::default();
        rectangle.apply_some(partial_rectangle);
        let rect_id = gen_str_id();
        rectangle.id = rect_id.clone();
        self.node_map.insert(self.replica_id.clone(), rect_id.clone(), SVGObject::Rectangle(rectangle));
        self.parent.insert(rect_id.clone(), (Some(NEW_NODE_ROOT_ID.to_string()), 0, epoch_now_nanos()));
        self.move_object(group_id, rect_id, None);
    }

    pub fn add_path(
        &mut self,
        group_id: Option<NodeID>,
        partial_path: PartialSVGPath
    ) {
        let mut path = SVGPath::default();
        path.apply_some(partial_path);
        let path_id = gen_str_id();
        path.id = path_id.clone();
        self.node_map.insert(self.replica_id.clone(), path_id.clone(), SVGObject::Path(path));
        self.parent.insert(path_id.clone(), (Some(NEW_NODE_ROOT_ID.to_string()), 0, epoch_now_nanos()));
        self.move_object(group_id, path_id, None);
    }

    pub fn add_point_to_path(
        &mut self,
        path_id: String,
        command_type: SVGPathCommandType,
        pos: Vec2
    ) {
        let Some(mut path) = self.get_path(path_id.clone()) else { return; };
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
        self.node_map.insert(self.replica_id.clone(), path_id, SVGObject::Path(path));
    }

    pub fn edit_circle(&mut self, circle_id: NodeID, edits: PartialSVGCircle) {
        let Some(mut circle) = self.get_circle(circle_id.clone()) else { return; };
        circle.apply_some(edits); 
        self.node_map.insert(self.replica_id.clone(), circle_id, SVGObject::Circle(circle));
    }

    pub fn edit_group(&mut self, group_id: NodeID, edits: PartialSVGGroup) {
        let Some(mut group) = self.get_group(group_id.clone()) else { return; };
        group.apply_some(edits);
        self.node_map.insert(self.replica_id.clone(), group_id, SVGObject::Group(group));
    }

    pub fn edit_rectangle(&mut self, rectangle_id: NodeID, edits: PartialSVGRectangle) {
        let Some(mut rect) = self.get_rectangle(rectangle_id.clone()) else { return; };
        rect.apply_some(edits);
        self.node_map.insert(self.replica_id.clone(), rectangle_id, SVGObject::Rectangle(rect));
    }

    pub fn edit_path(&mut self, path_id: NodeID, edits: PartialSVGPath) {
        let Some(mut path) = self.get_path(path_id.clone()) else { return; };
        path.apply_some(edits);
        self.node_map.insert(self.replica_id.clone(), path_id, SVGObject::Path(path));
    }

    pub fn edit_path_point_type(
        &mut self, 
        path_id: NodeID, 
        point_id: NodeID, 
        command_type: SVGPathCommandType
    ) {
        let Some(mut path) = self.get_path(path_id.clone()) else { return; };
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
        match points.get_mut(idx) {
            Some(point) => {
                *point = command
            },
            _ => {}
        };
        path.points = points;
        self.node_map.insert(self.replica_id.clone(), point_id, SVGObject::Path(path));
    }

    pub fn edit_path_point_pos(
        &mut self,
        path_id: NodeID,
        point_id: NodeID,
        new_pos: Vec2
    ) {
        let Some(mut path) = self.get_path(path_id) else { return; };
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
                    SVGPathCommand::BezierQuadReflect { pos, .. } => {
                        *pos = new_pos;
                    },
                    _ => ()
                };
            },
            _ => {}
        };
        path.points = points;
        self.node_map.insert(self.replica_id.clone(), point_id, SVGObject::Path(path));
    }

    pub fn edit_path_point_handle1(
        &mut self,
        path_id: NodeID,
        point_id: NodeID,
        new_handle1: Vec2
    ) {
        let Some(mut path) = self.get_path(path_id) else { return; };
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
                    SVGPathCommand::BezierReflect { handle, .. } => {
                        *handle = new_handle1;
                    },
                    SVGPathCommand::BezierQuad { handle, .. } => {
                        *handle = new_handle1;
                    },
                    _ => ()
                }
            },
            _ => {}
        };
        path.points = points;
        self.node_map.insert(self.replica_id.clone(), point_id, SVGObject::Path(path));
    }

    pub fn edit_path_point_handle2(
        &mut self,
        path_id: NodeID,
        point_id: NodeID,
        new_handle2: Vec2
    ) {
        let Some(mut path) = self.get_path(path_id) else { return; };
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
        path.points = points;
        self.node_map.insert(self.replica_id.clone(), point_id, SVGObject::Path(path));
    }

    pub fn remove_object(&mut self, node_id: NodeID) {
        self.node_map.remove(self.replica_id.clone(), node_id.clone());
    }

    pub fn remove_path_point(
        &mut self,
        path_id: NodeID,
        point_id: NodeID
    ) {
        let Some(mut path) = self.get_path(path_id.clone()) else { return; };
        let index = path.points.iter()
            .position(|o| o.get_id().eq(&point_id));
        let Some(index) = index else { return; };
        path.points.remove(index);
        self.node_map.insert(self.replica_id.clone(), path_id, SVGObject::Path(path));
    }

    fn update_ordering(
        &mut self, 
        old_group_id: Option<NodeID>, 
        new_group_id: Option<NodeID>,
        object_id: NodeID,
        index: Option<usize>
    ) -> usize {
        match self.ordering.entry(old_group_id) {
            Entry::Occupied(mut o) => {
                let index = o.get().iter().position(|o| o == &object_id);
                if let Some(index) = index {
                    o.get_mut().remove(index);
                }
            },
            Entry::Vacant(v) => {
                v.insert(Vec::new());
            }
        };

        match self.ordering.entry(new_group_id) {
            Entry::Occupied(mut o) => {
                let vecc = o.get_mut();
                if let Some(index) = index {
                    vecc.insert(index, object_id.clone());
                    return index;
                } else {
                    vecc.push(object_id.clone());
                    return vecc.len();
                }
            },
            Entry::Vacant(v) => {
                let mut vecc = Vec::new();
                vecc.push(object_id.clone());
                let length = vecc.len();
                v.insert(vecc);
                return length;
            }
        }
    }
    pub fn move_object(&mut self, group_id: Option<NodeID>, object_id: String, index: Option<usize>) {
        let now = epoch_now_nanos();
        let Some((old_group_id, _, _)) = self.parent.get(&object_id) else { return; };
        let old_group_id = old_group_id.clone();
        let Some(_) = self.node_map.get(&object_id).map(|o| o.clone()) else { return; };
        if let Some(group_id) = group_id {
            if self.is_ancestor(&object_id, &group_id) { return; }

            let index = self.update_ordering(old_group_id.clone(), Some(group_id.clone()), object_id.clone(), index);
            self.parent.insert(object_id.clone(), (Some(group_id.clone()), index, now));
            self.node_map.inc_vtime(self.replica_id.clone(), object_id.clone());
            let move_log = MoveLog { new_group_id: Some(group_id), old_group_id, index, object_id, timestamp: now };
            self.send_buffer.push(move_log.clone());
            self.move_history.push(move_log);
            return;
        }
        let index = self.update_ordering(old_group_id.clone(), None, object_id.clone(), index);
        self.parent.insert(object_id.clone(), (None, index, now));
        self.node_map.inc_vtime(self.replica_id.clone(), object_id.clone());
        let move_log = MoveLog { new_group_id: None, old_group_id, index, object_id, timestamp: now };
        self.send_buffer.push(move_log.clone());
        self.move_history.push(move_log);
    }

    fn redo_move(&mut self, MoveLog { new_group_id, index, object_id, timestamp, old_group_id, .. }: MoveLog) {
        if let Some(new_group_id) = new_group_id.clone() {
            if self.is_ancestor(&object_id, &new_group_id) { return; }
        }
        self.update_ordering(old_group_id, new_group_id.clone(), object_id.clone(), Some(index));
        self.parent.insert(object_id, (new_group_id, index, timestamp));
    }

    fn undo_move(&mut self, MoveLog { old_group_id, object_id, new_group_id, .. }: MoveLog) {
        self.update_ordering(new_group_id, old_group_id.clone(), object_id.clone(), None);
        self.parent.insert(object_id, (old_group_id, 0, 0));
    }

    fn add_to_move_log(&mut self, move_log: MoveLog) {
        let mut k = self.move_history.len();
        let move_history = self.move_history.clone();
        for (i, hist) in move_history.iter().enumerate().rev() {
            if hist.timestamp < move_log.timestamp {
                break
            }
            self.undo_move(hist.clone());
            k = i;
        }
        self.move_history.insert(k, move_log);
        let n = self.move_history.len();
        while k < n {
            self.redo_move(self.move_history[k].clone());
            k += 1;
        }
    }

    fn merge_aux(&mut self, other_node_map: UWMap<NodeID, SVGObject>, mut move_logs: Vec<MoveLog>) {
        self.node_map = UWMap::merge(&self.node_map, &other_node_map);
        for log in move_logs.drain(..) {
            self.add_to_move_log(log);
        }
    }

    fn broadcast_aux(&mut self) -> (UWMap<String, SVGObject>, Vec<MoveLog>){
        let res = (self.node_map.clone(), self.send_buffer.clone());
        self.send_buffer.clear();
        res
    }

    pub fn broadcast(&mut self) -> String {
        let tup = self.broadcast_aux();
        serde_json::to_string(&tup).unwrap()
    }

    pub fn merge(&mut self, data: String) {
        type Tup = (UWMap<String, SVGObject>, Vec<MoveLog>);
        let tup = serde_json::from_str::<Tup>(&data).ok();
        let Some((other_node_map, move_logs)) = tup else { return; };
        self.merge_aux(other_node_map, move_logs);
    }

    pub fn save(&self) -> String {
        type Tup = (UWMap<String, SVGObject>, Vec<MoveLog>);
        let node_map = self.node_map.clone();
        let move_history = self.move_history.clone();
        let tup: Tup = (node_map, move_history);
        serde_json::to_string(&tup).unwrap()
    }

    pub fn load(&mut self, data: String) {
        self.clear();
        type Tup = (UWMap<String, SVGObject>, Vec<MoveLog>);
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
        match self.parent.get(object_id) {
            Some((Some(p), _, _)) => {
                self.dfs(p, visited, res);
                res.push(object_id.clone());
            },
            Some((None, _, _)) => {
                res.push(object_id.clone());
            },
            _ => return,
        };
    }

    fn top_sort_nodes(&self) -> Vec<NodeID> {
        let mut res = Vec::new();
        let mut visited = HashMap::new();
        let mut keys = self.node_map.value()
            .keys()
            .map(|k| k.clone())
            .collect::<Vec<_>>();
        keys
            .sort_by(|a, b| {
                let (_, _, timestamp_a) = self.parent.get(a).unwrap();
                let (_, _, timestamp_b) = self.parent.get(b).unwrap();
                timestamp_a.cmp(timestamp_b)
            });
        // keys.reverse();
        for object_id in keys.iter() {
            self.dfs(object_id, &mut visited, &mut res);
        }
        res
    }

    pub fn tree(&self) -> SVGDocTree {
        let mut res = SVGDocTree::new();
        let mut node_map = self.node_map.value();
        let mut nodes = self.top_sort_nodes();
        nodes.reverse();

        // Apply index according to timestamp
        for node in nodes.iter() {
            let Some((Some(group_id), _, _)) = self.parent.get(node) else { continue; };
            let Some(ord_list) = self.ordering.get(&Some(group_id.clone())) else { continue; };
            let Some(ord_ind) = ord_list.iter().position(|o| o == node) else { continue; };
            match node_map.remove(group_id) {
                Some(SVGObject::Group(g)) => {
                    let mut group = g;
                    let mut i = group.children.len();
                    loop {
                        if i == 0 { break; }
                        let next_id = group.children[i - 1].get_id();
                        let Some(nxt_ord_ind) = ord_list.iter().position(|o| o == next_id) else { continue; };
                        if ord_ind > nxt_ord_ind { break; }
                        i -= 1;
                    }
                    let Some(object) = node_map.remove(node) else { 
                        node_map.insert(group_id.clone(), SVGObject::Group(group));
                        continue;
                    };
                    group.children.insert(i, object);
                    node_map.insert(group_id.clone(), SVGObject::Group(group));
                },
                Some(o) => {
                    node_map.insert(group_id.clone(), o);
                },
                _ => {
                    node_map.remove(node);
                },
            }
        }
        let Some(ord_list_root) = self.ordering.get(&None) else { return res };
        res.children = node_map.drain().map(|(_, o)| o).collect();
        res.children.sort_by_key(|o| {
            ord_list_root.iter().position(|ss| ss == o.get_id()).unwrap()
        });
        res
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

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
        // println!("{} {}", first_id, act_first_id);
        // println!("{} {}", second_id, act_second_id);
        // println!("{} {}", third_id, act_third_id);
        // println!("{} {}", fourth_id, act_fourth_id);
        // println!("{} {}", fifth_id, act_fifth_id);
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
        let (r1_node_map, r1_move_logs) = doc1.broadcast_aux();
        doc2.merge_aux(r1_node_map, r1_move_logs);
        let t2 = doc2.tree();
        let circle_id = match t2.children.get(0) {
            Some(SVGObject::Circle(circle)) => circle.id.clone(),
            _ => panic!("Circle should exist at index 0")
        };
        let mut edits = PartialSVGCircle::empty();
        edits.opacity = Some(0.5);
        doc2.edit_circle(circle_id.clone(), edits);
        doc1.remove_object(circle_id);
        let (r1_node_map, r1_move_logs) = doc1.broadcast_aux();
        let (r2_node_map, r2_move_logs) = doc2.broadcast_aux();
        doc1.merge_aux(r2_node_map, r2_move_logs);
        doc2.merge_aux(r1_node_map, r1_move_logs);
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
        // println!("{:?}", tree);
        assert_eq!(tree.children.len(), 0);
    }
}