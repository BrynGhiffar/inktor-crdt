import { SVGPath, SVGPathCommand } from "@brynghiffar/vect-crdt-rs";
import { ReactSVGObjectState } from "../../types";
import { CSSProperties, FC, useCallback } from "react";
import { useSortable } from "@dnd-kit/sortable";
import { isObjectSelected } from "../../utility/methods";
import { CSS } from "@dnd-kit/utilities";
import { PaddedDiv } from "../../utility/components";
import { CodeFragment } from "./CodeFragment";

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
    case "BEZIER_QUAD":
      return `Q ${point.handle.x} ${point.handle.y} ${point.pos.x} ${point.pos.y}`
  }
}

export const PathCode: FC<PathCodeProps> = (props) => {
  const { 
    attributes, 
    listeners, 
    setNodeRef,
    transform,
    transition
  } = useSortable({ id: props.data.id });
  const path = props.data.points.map((p, i) => [p.id, `${i == 0 ? "" : " "}${toPathString(p)}`]);
  const [selectedObject, setSelectedObject] = props.selectedObjectState;
  const onClick = useCallback(() => {
    setSelectedObject({ type: "PATH", ...props.data});
  }, [setSelectedObject, props]);
  const divStyle: CSSProperties = {
    transition, 
    transform: CSS.Transform.toString(transform)
  };
  const [ fillRed, fillGreen, fillBlue, fillOpacity ] = props.data.fill;
  const [ strokeRed, strokeGreen, strokeBlue, strokeOpacity ] = props.data.stroke;
  return (
    <PaddedDiv
    depth={props.depth}
    style={divStyle}
    ref={setNodeRef}
    {...attributes}
    {...listeners}
    >
      <CodeFragment
        noRoundRight
        noRightPadding
        selected={isObjectSelected(props.data.id, selectedObject)} 
        onClick={onClick}>{`<path d="`}</CodeFragment>
      {
        path.map(([id, p]) => (
          <CodeFragment key={id} 
            noRoundLeft
            noRoundRight
            noLeftPadding
            noRightPadding
            selected={isObjectSelected(props.data.id, selectedObject)}
            onClick={onClick}>{p}</CodeFragment>
        ))
      }
      <CodeFragment 
        noRoundLeft
        noLeftPadding
        selected={isObjectSelected(props.data.id, selectedObject)} 
        onClick={onClick}
      >{`" fill="rgba(${fillRed}, ${fillGreen}, ${fillBlue}, ${fillOpacity})" stroke="rgba(${strokeRed}, ${strokeGreen}, ${strokeBlue}, ${strokeOpacity})" stroke-width="${props.data.stroke_width}" opacity="${props.data.opacity}" />`}</CodeFragment>
    </PaddedDiv>
  )
}