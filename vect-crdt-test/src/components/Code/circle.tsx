import { useSortable } from "@dnd-kit/sortable";
import { CSSProperties, FC, MutableRefObject } from "react";
import { DroppableSVG, ReactSVGObjectState } from "../../types";
import { SVGDoc } from "@brynghiffar/vect-crdt-rs";
import { isObjectSelected } from "../../utility/methods";
import { CSS } from '@dnd-kit/utilities';
import { PaddedDiv } from "../../utility/components";
import { CodeFragment } from "./CodeFragment";

type CircleCodeProps = {
  depth: number,
  data: DroppableSVG,
  selectedObjectState: ReactSVGObjectState,
  docRef: MutableRefObject<SVGDoc>
}


export const CircleCode: FC<CircleCodeProps> = (props) => {
  const { 
    attributes, 
    listeners, 
    setNodeRef,
    transform,
    transition
  } = useSortable({ id: props.data.id });
  const circle = props.docRef.current.get_circle(props.data.id);
  if (circle === undefined) return (<></>);
  const [fillRed, fillGreen, fillBlue, fillOpacity] = circle.fill;
  const opacity = circle.opacity;
  const [strokeRed, strokeGreen, strokeBlue, strokeOpacity ] = circle.stroke;
  const [selectedObject, setSelectedObject] = props.selectedObjectState;
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
      <CodeFragment
        selected={isObjectSelected(props.data.id, selectedObject)}
        onClick={() => setSelectedObject({ type: "CIRCLE", id: props.data.id })}
      >{`<circle 
        cx="${circle.pos.x}"
        cy="${circle.pos.y}" 
        r="${circle.radius}"
        fill="rgba(${fillRed}, ${fillGreen}, ${fillBlue}, ${fillOpacity})"
        stroke="rgba(${strokeRed}, ${strokeGreen}, ${strokeBlue}, ${strokeOpacity})"  
        stroke-width="${circle.stroke_width}"
        opacity="${opacity}"/>`
      }
      </CodeFragment>
    </PaddedDiv>
  )
}