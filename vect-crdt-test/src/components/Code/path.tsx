import { SVGPath, SVGPathCommand } from "@brynghiffar/vect-crdt-rs";
import { ReactSVGObjectState } from "../../types";
import { CSSProperties, FC, useCallback } from "react";
import { useSortable } from "@dnd-kit/sortable";
import { isObjectSelected } from "../../utility/methods";
import { focusColor, unfocusColor } from "../../palette/color";
import { CSS } from "@dnd-kit/utilities";
import { PaddedDiv } from "../../utility/components";

type PathCodeProps = {
  depth: number,
  data: SVGPath,
  selectedObjectState: ReactSVGObjectState
};

const toPathString = (point: SVGPathCommand) => {
  switch (point.type) {
    case "START":
      return `M ${point.pos.x} ${point.pos.y}`
    case "LINE":
      return `L ${point.pos.x} ${point.pos.y}`
    case "CLOSE":
      return "Z"
    case "BEZIER":
      return `C ${point.handle1.x} ${point.handle1.y} ${point.handle2.x} ${point.handle2.y} ${point.pos.x} ${point.pos.y}`
    case "BEZIER_REFLECT":
      return `S ${point.handle.x} ${point.handle.y} ${point.pos.x} ${point.pos.y}`
    case "BEZIER_QUAD":
      return `Q ${point.handle.x} ${point.handle.y} ${point.pos.x} ${point.pos.y}`
    case "BEZIER_QUAD_REFLECT":
      return `T ${point.pos.x} ${point.pos.y}`
  }
}

export const PathCode: FC<PathCodeProps> = (props) => {
  const { 
    attributes, 
    listeners, 
    setNodeRef,
    transform,
    transition
  } = useSortable({ id: props.data.id, data: {something: "path moved"} });
  const path = props.data.points.map((p, i) => [p.id, `${i == 0 ? "" : " "}${toPathString(p)}`]);
  const [selectedObject, setSelectedObject] = props.selectedObjectState;
  const onClick = useCallback(() => {
    setSelectedObject({ type: "PATH", ...props.data});
  }, [setSelectedObject, props]);
  const background = isObjectSelected(props.data.id, selectedObject) ? focusColor : unfocusColor;
  const style: CSSProperties = { 
    overflow: "hidden",
    whiteSpace: "nowrap",
    background, 
    cursor: "pointer"
  };
  const divStyle: CSSProperties = {
    transition, 
    transform: CSS.Transform.toString(transform)
  };
  return (
    <PaddedDiv
    depth={props.depth}
    style={divStyle}
    ref={setNodeRef}
    {...attributes}
    {...listeners}
    >
      <code onClick={onClick} style={style}>{"<path d=\""}</code>
      {
        path.map(([id, p]) => (
          <code key={id} onClick={onClick} style={style}>{p}</code>
        ))
      }
      <code style={style}>{"\"/>"}</code>
    </PaddedDiv>
  )
}