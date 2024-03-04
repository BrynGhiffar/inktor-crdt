import { CSSProperties, FC } from "react";
import { SVGRectangle } from "@brynghiffar/vect-crdt-rs";
import { ReactSVGObjectState } from "../../types";
import { useSortable } from "@dnd-kit/sortable";
import { isObjectSelected } from "../../utility/methods";
import { CSS } from "@dnd-kit/utilities";
import { PaddedDiv } from "../../utility/components";
import { CodeFragment } from "./CodeFragment";

type RectangleCodeProps = {
  depth: number,
  data: SVGRectangle,
  selectedObjectState: ReactSVGObjectState
}

export const RectangleCode: FC<RectangleCodeProps> = props => {
  const { 
    attributes, 
    listeners, 
    setNodeRef,
    transform,
    transition
  } = useSortable({ id: props.data.id });
  const [fillRed, fillGreen, fillBlue, fillOpacity] = props.data.fill;
  const [strokeRed, strokeGreen, strokeBlue, strokeOpacity] = props.data.stroke;
  const [selectedObject, setSelectedObject] = props.selectedObjectState;
  const opacity = props.data.opacity;
  const divStyle: CSSProperties = {
    whiteSpace: "nowrap",
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
      <CodeFragment
        selected={isObjectSelected(props.data.id, selectedObject)}
        onClick={() => setSelectedObject({ type: "RECTANGLE", ...props.data })}
      >{`<rect
      x="${props.data.pos.x}" 
      y="${props.data.pos.y}" 
      width="${props.data.width}" 
      height="${props.data.height}" 
      fill="rgba(${fillRed}, ${fillGreen}, ${fillBlue}, ${fillOpacity})"
      stroke-width="${props.data.stroke_width}"
      stroke="rgba(${strokeRed}, ${strokeGreen}, ${strokeBlue}, ${strokeOpacity})"
      opacity="${opacity}"/>`}
      </CodeFragment>
    </PaddedDiv>
  )
};
