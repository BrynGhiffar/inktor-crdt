import { CSSProperties, FC, MutableRefObject, useCallback } from "react";
import { SVGDoc, SVGGroup } from "@brynghiffar/vect-crdt-rs";
import { ReactSVGObjectState } from "../../types";
import { useSortable } from "@dnd-kit/sortable";
import { isObjectSelected } from "../../utility/methods";
import { CSS } from "@dnd-kit/utilities";
import { PaddedDiv } from "../../utility/components";
import { CodeFragment } from "./CodeFragment";
import { toRgbaString } from "./util";

type GroupCodeProps = {
  id: string,
  depth: number,
  data: SVGGroup,
  docRef: MutableRefObject<SVGDoc>,
  selectedObjectState: ReactSVGObjectState
  fetchSVGDoc: () => void,
}

export const GroupOpenCode: FC<GroupCodeProps> = props => {
  const { 
    attributes, 
    listeners, 
    setNodeRef,
    transform,
    transition
  } = useSortable({ id: props.id });
  const [selectedObject, setSelectedObject] = props.selectedObjectState;
  const onClick = useCallback(() => setSelectedObject({ type: "GROUP", id: props.id }), [props, setSelectedObject]);
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
        selected={isObjectSelected(props.id, selectedObject)}
        onClick={onClick}
      >{`<g id="${props.id}"${toRgbaString("fill", props.data.fill)}${toRgbaString("stroke", props.data.stroke)}${props.data.stroke_width !== null ? ` stroke-width="${props.data.stroke_width}"` : ""}${props.data.opacity !== null ? ` opacity="${props.data.opacity}"` : ""}>`}</CodeFragment>
    </PaddedDiv>
  )
};

type GroupCloseCodeProps = {
  id: string,
  depth: number,
  docRef: MutableRefObject<SVGDoc>,
  selectedObjectState: ReactSVGObjectState
  fetchSVGDoc: () => void,
};

export const GroupCloseCode: FC<GroupCloseCodeProps> = (props) => {
  const realId = props.id.replace("END_", "");
  const { 
    attributes, 
    listeners, 
    setNodeRef,
    transform,
    transition
  } = useSortable({ id: props.id, disabled: true });
  const [selectedObject, setSelectedObject] = props.selectedObjectState;
  const onClick = useCallback(() => setSelectedObject({ type: "GROUP", id: realId }), [realId, setSelectedObject]);
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
        selected={isObjectSelected(realId, selectedObject)}
        onClick={onClick}
      >{`</g>`}</CodeFragment>
    </PaddedDiv>
  )
};