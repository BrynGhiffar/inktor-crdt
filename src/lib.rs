#![allow(nonstandard_style)]
pub mod prelude;
pub mod utility;
pub mod element;
pub mod crdt;

use prelude::*;

#[wasm_bindgen]
pub struct SVGDoc {
    tree: SVGDocCrdt2,
}


#[wasm_bindgen]
impl SVGDoc {
    pub fn new(replica_id: ReplicaId) -> Self {
        return SVGDoc { tree: SVGDocCrdt2::new(replica_id) };
    }

    pub fn get_group(&self, group_id: String) -> Option<SVGGroup> {
        self.tree.get_group(group_id)
    }

    pub fn add_group(&mut self, group_id: Option<String>, partial_group: PartialSVGGroup) {
        self.tree.add_group(group_id, partial_group)
    }

    pub fn get_circle(&self, circle_id: String) -> Option<SVGCircle>{
        self.tree.get_circle(circle_id)
    }

    pub fn add_circle(&mut self, group_id: Option<String>, partial_circle: PartialSVGCircle) {
        self.tree.add_circle(group_id, partial_circle);
    }

    pub fn edit_circle(&mut self, circle_id: String, edits: PartialSVGCircle) {
        self.tree.edit_circle(circle_id, edits);
    }

    pub fn get_rectangle(&self, rectangle_id: String) -> Option<SVGRectangle> {
        self.tree.get_rectangle(rectangle_id)
    }

    pub fn add_rectangle(&mut self, group_id: Option<String>, partial_rectangle: PartialSVGRectangle) {
        self.tree.add_rectangle(group_id, partial_rectangle)
    }

    pub fn edit_rectangle(&mut self, rectangle_id: String, edits: PartialSVGRectangle) {
        self.tree.edit_rectangle(rectangle_id, edits)
    }

    pub fn get_path(&self, path_id: String) -> Option<SVGPath> {
        self.tree.get_path(path_id)
    }

    pub fn add_path(&mut self, group_id: Option<String>, partial_path: PartialSVGPath) {
        self.tree.add_path(group_id, partial_path)
    }

    pub fn edit_path(&mut self, path_id: String, partial_path: PartialSVGPath) {
        self.tree.edit_path(path_id, partial_path)
    }

    pub fn edit_group(
        &mut self,
        group_id: String,
        partial_group: PartialSVGGroup
    ) {
        console_log!("Editing group: {:?}", partial_group.fill);
        self.tree.edit_group(group_id, partial_group)
    }

    pub fn edit_path_point_type(
        &mut self, 
        path_id: String, 
        point_id: String, 
        command_type: SVGPathCommandType, 
    ) {
        self.tree.edit_path_point_type(path_id, point_id, command_type)
    }

    pub fn edit_path_point_pos(
        &mut self, 
        path_id: String, 
        point_id: String, 
        new_pos: Vec2
    ) {
        self.tree.edit_path_point_pos(path_id, point_id, new_pos)
    }

    pub fn edit_path_point_handle1(
        &mut self, 
        path_id: String, 
        point_id: String, 
        new_handle1: Vec2
    ) {
        self.tree.edit_path_point_handle1(path_id, point_id, new_handle1)
    }

    pub fn edit_path_point_handle2(
        &mut self, 
        path_id: String, 
        point_id: String, 
        new_handle2: Vec2
    ) {
        self.tree.edit_path_point_handle2(path_id, point_id, new_handle2)
    }

    pub fn add_point_to_path(
        &mut self, 
        path_id: String, 
        command: SVGPathCommandType, 
        pos: Vec2
    ) {
        self.tree.add_point_to_path(path_id, command, pos)
    }

    pub fn move_object_to_group(
        &mut self, 
        object_id: String, 
        group_id: String, 
        index: usize
    ) {
        // self.tree.move_object_to_group(object_id, group_id, index)
        self.tree.move_object(Some(group_id), object_id, Some(index))
    }

    pub fn move_object_to_root(&mut self, object_id: String, index: usize) {
        // self.tree.move_object_to_root(object_id, index)
        self.tree.move_object(None, object_id, Some(index))
    }

    pub fn remove_object(&mut self, object_id: String) {
        self.tree.remove_object(object_id)
    }

    pub fn remove_path_point(
        &mut self, 
        path_id: String,
        point_id: String
    ) {
        self.tree.remove_path_point(path_id, point_id)
    }

    pub fn save(&self) -> Option<String> {
        Some(self.tree.save())
    }

    pub fn load(&mut self, data: String) {
        self.tree.load(data)
    }

    pub fn broadcast(&mut self) -> String {
        self.tree.broadcast()
    }

    pub fn merge(&mut self, oplog: String) {
        self.tree.merge(oplog);
    }

    pub fn children(&self) -> SVGDocTree {
        self.tree.tree()
    }

    pub fn repr(&self) -> String {
        "NO_REPR".to_string()
        // self.tree.repr()
    }
}
