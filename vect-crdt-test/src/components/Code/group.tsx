import { CSSProperties, FC, MutableRefObject, useCallback } from "react";
import { SVGDoc } from "@brynghiffar/vect-crdt-rs";
import { ReactSVGObjectState } from "../../types";
import { useSortable } from "@dnd-kit/sortable";
import { isObjectSelected } from "../../utility/methods";
import { CSS } from "@dnd-kit/utilities";
import { PaddedDiv } from "../../utility/components";
import { focusColor, unfocusColor } from "../../palette/color";

type GroupCodeProps = {
  id: string,
  depth: number,
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
  const background = isObjectSelected(props.id, selectedObject) ? focusColor : unfocusColor;
  const onClick = useCallback(() => setSelectedObject({ type: "GROUP", id: props.id }), [props, setSelectedObject]);
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
      <code
        style={style}
        onClick={onClick}
      >{`<g id="${props.id}">`}</code>
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
  const background = isObjectSelected(realId, selectedObject) ? focusColor : unfocusColor;
  const onClick = useCallback(() => setSelectedObject({ type: "GROUP", id: realId }), [realId, setSelectedObject]);
  const style = { background, cursor: "pointer" };
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
      <code
        style={style}
        onClick={onClick}
      >{`</g>`}</code>
    </PaddedDiv>
  )
};