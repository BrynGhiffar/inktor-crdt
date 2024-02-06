/* tslint:disable */
/* eslint-disable */
/**
*/
export enum SVGPathCommandType {
  START = 0,
  LINE = 1,
  CLOSE = 2,
  BEZIER = 3,
  BEZIER_REFLECT = 4,
  BEZIER_QUAD = 5,
  BEZIER_QUAD_REFLECT = 6,
}
export type SVGObject = ({ type: "CIRCLE" } & SVGCircle) | ({ type: "RECTANGLE" } & SVGRectangle) | ({ type: "PATH" } & SVGPath) | ({ type: "GROUP" } & SVGGroup);

export interface SVGDocTree {
    children: SVGObject[];
}

export type SVGPathCommand = { type: "START"; id: string; pos: Vec2 } | { type: "LINE"; id: string; pos: Vec2 } | { type: "CLOSE"; id: string } | { type: "BEZIER"; id: string; handle1: Vec2; handle2: Vec2; pos: Vec2 } | { type: "BEZIER_REFLECT"; id: string; handle: Vec2; pos: Vec2 } | { type: "BEZIER_QUAD"; id: string; handle: Vec2; pos: Vec2 } | { type: "BEZIER_QUAD_REFLECT"; id: string; pos: Vec2 };

export interface SVGPath {
    id: string;
    fill: Color;
    stroke_width: number;
    stroke: Color;
    points: SVGPathCommand[];
    opacity: number;
}

export interface PartialSVGPath {
    fill?: Color;
    stroke_width?: number;
    stroke?: Color;
    opacity?: number;
}

export interface SVGRectangle {
    id: string;
    pos: Vec2;
    height: number;
    width: number;
    fill: Color;
    stroke_width: number;
    stroke: Color;
    opacity: number;
}

export interface PartialSVGRectangle {
    pos?: Vec2;
    height?: number;
    width?: number;
    fill?: Color;
    stroke_width?: number;
    stroke?: Color;
    opacity?: number;
}

export interface Vec2 {
    x: number;
    y: number;
}

export type Color = [number, number, number, number];

export interface SVGCircle {
    id: string;
    pos: Vec2;
    radius: number;
    fill: Color;
    stroke_width: number;
    stroke: Color;
    opacity: number;
}

export interface PartialSVGCircle {
    pos?: Vec2;
    radius?: number;
    fill?: Color;
    stroke_width?: number;
    stroke?: Color;
    opacity?: number;
}

export interface SVGGroup {
    id: string;
    fill: Color | null;
    stroke: Color | null;
    stroke_width: number | null;
    children: SVGObject[];
}

export interface PartialSVGGroup {
    fill?: Color | null;
    stroke?: Color | null;
    stroke_width?: number | null;
}

/**
*/
export class SVGDoc {
  free(): void;
/**
* @returns {SVGDoc}
*/
  static new(): SVGDoc;
/**
* @param {string} group_id
* @returns {SVGGroup | undefined}
*/
  get_group(group_id: string): SVGGroup | undefined;
/**
* @param {string | undefined} group_id
* @param {PartialSVGGroup} partial_group
*/
  add_group(group_id: string | undefined, partial_group: PartialSVGGroup): void;
/**
* @param {string} circle_id
* @returns {SVGCircle | undefined}
*/
  get_circle(circle_id: string): SVGCircle | undefined;
/**
* @param {string | undefined} group_id
* @param {PartialSVGCircle} partial_circle
*/
  add_circle(group_id: string | undefined, partial_circle: PartialSVGCircle): void;
/**
* @param {string} circle_id
* @param {PartialSVGCircle} edits
*/
  edit_circle(circle_id: string, edits: PartialSVGCircle): void;
/**
* @param {string} rectangle_id
* @returns {SVGRectangle | undefined}
*/
  get_rectangle(rectangle_id: string): SVGRectangle | undefined;
/**
* @param {string | undefined} group_id
* @param {PartialSVGRectangle} partial_rectangle
*/
  add_rectangle(group_id: string | undefined, partial_rectangle: PartialSVGRectangle): void;
/**
* @param {string} rectangle_id
* @param {PartialSVGRectangle} edits
*/
  edit_rectangle(rectangle_id: string, edits: PartialSVGRectangle): void;
/**
* @param {string} path_id
* @returns {SVGPath | undefined}
*/
  get_path(path_id: string): SVGPath | undefined;
/**
* @param {string | undefined} group_id
* @param {PartialSVGPath} partial_path
*/
  add_path(group_id: string | undefined, partial_path: PartialSVGPath): void;
/**
* @param {string} path_id
* @param {PartialSVGPath} partial_path
*/
  edit_path(path_id: string, partial_path: PartialSVGPath): void;
/**
* @param {string} path_id
* @param {string} point_id
* @param {SVGPathCommandType} command_type
*/
  edit_path_point_type(path_id: string, point_id: string, command_type: SVGPathCommandType): void;
/**
* @param {string} path_id
* @param {string} point_id
* @param {Vec2} new_pos
*/
  edit_path_point_pos(path_id: string, point_id: string, new_pos: Vec2): void;
/**
* @param {string} path_id
* @param {string} point_id
* @param {Vec2} new_handle1
*/
  edit_path_point_handle1(path_id: string, point_id: string, new_handle1: Vec2): void;
/**
* @param {string} path_id
* @param {string} point_id
* @param {Vec2} new_handle2
*/
  edit_path_point_handle2(path_id: string, point_id: string, new_handle2: Vec2): void;
/**
* @param {string} path_id
* @param {SVGPathCommandType} command
* @param {Vec2} pos
*/
  add_point_to_path(path_id: string, command: SVGPathCommandType, pos: Vec2): void;
/**
* @param {string} object_id
* @param {string} group_id
* @param {number} index
*/
  move_object_to_group(object_id: string, group_id: string, index: number): void;
/**
* @param {string} object_id
* @param {number} index
*/
  move_object_to_root(object_id: string, index: number): void;
/**
* @param {string} object_id
*/
  remove_object(object_id: string): void;
/**
* @param {string} path_id
* @param {string} point_id
*/
  remove_path_point(path_id: string, point_id: string): void;
/**
* @returns {string | undefined}
*/
  save(): string | undefined;
/**
* @param {string} oplog
*/
  merge(oplog: string): void;
/**
* @returns {SVGDocTree}
*/
  children(): SVGDocTree;
/**
* @returns {string}
*/
  repr(): string;
}
