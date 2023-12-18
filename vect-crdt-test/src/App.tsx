import { CSSProperties, Dispatch, FC, MutableRefObject, SetStateAction, forwardRef, useCallback, useEffect, useRef, useState } from 'react';
import { SVGDoc, SVGCircle, SVGDocInner, SVGObject, SVGRectangle, SVGGroup, SVGPath, SVGPathCommand } from "vect-crdt-rs";
import './App.css';
import { Configuration } from './components/Configuration';
import { DivProps } from './types';
import { DndContext, DragEndEvent, PointerSensor, closestCenter, useSensor, useSensors } from '@dnd-kit/core';
import { SortableContext, useSortable, verticalListSortingStrategy } from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';
import { AllFeatures } from './components/Tree/Tree.story';

type ReactSetSVGObject = Dispatch<SetStateAction<SVGObject | "root">>;
type ReactSVGObjectState = [SVGObject | "root", ReactSetSVGObject];



const isObjectSelected = (id: string, obj: SVGObject | "root") => {
  if (obj === "root") return id === "root";
  return obj.id === id;
}

const selectedColor = "lightgray";
const unselectedColor = "transparent";

const PaddedDiv = forwardRef<HTMLDivElement, DivProps>((props, ref) => {
  const { children, style: propsStyle, ...divProps } = props;
  const style = { paddingLeft: "20px", ...propsStyle };
  return (
    <div
      ref={ref}
      {...divProps}
      style={style}
    >{children}</div>
  )
});

const mapper = (
  docRef: MutableRefObject<SVGDoc>,
  fetchSVGDoc: () => void,
  selectedObjectState: ReactSVGObjectState
) => {
  return (obj: SVGObject) => {
    switch (obj.type) {
      case "CIRCLE":
        return <CircleCode
          data={obj}
          key={obj.id}
          selectedObjectState={selectedObjectState}
        />
      case "RECTANGLE":
        return <RectangleCode
          data={obj}
          key={obj.id}
          selectedObjectState={selectedObjectState}
        />
      case "GROUP":
        return <GroupCode
          key={obj.id}
          docRef={docRef}
          data={obj}
          fetchSVGDoc={fetchSVGDoc}
          selectedObjectState={selectedObjectState}
        />
      case "PATH":
        return <PathCode
          key={obj.id}
          data={obj}
          selectedObjectState={selectedObjectState}
        />
      default:
        return <></>
    }
  }

}

type CircleCodeProps = {
  data: SVGCircle,
  selectedObjectState: ReactSVGObjectState,
}

const numToHex = (num: number) => {
  const hex = num.toString(16);
  return `${hex.length === 1 ? "0" : ""}${hex}`;
}

const rgbToHex = (red: number, green: number, blue: number) => {
  return `#${numToHex(red)}${numToHex(green)}${numToHex(blue)}`;
}

const CircleCode: FC<CircleCodeProps> = (props) => {
  const { 
    attributes, 
    listeners, 
    setNodeRef,
    transform,
    transition
  } = useSortable({ id: props.data.id });
  const [fillRed, fillGreen, fillBlue, fillOpacity] = props.data.fill;
  const opacity = props.data.opacity;
  const [strokeRed, strokeGreen, strokeBlue, strokeOpacity ] = props.data.stroke;
  const [selectedObject, setSelectedObject] = props.selectedObjectState;
  const background = isObjectSelected(props.data.id, selectedObject) ? selectedColor : unselectedColor;
  const style: CSSProperties = { 
    background, 
    cursor: "pointer",
  };
  const divStyle: CSSProperties = {
    transition, 
    transform: CSS.Transform.toString(transform)
  };
  return (
    <PaddedDiv
      style={divStyle}
        ref={setNodeRef}
        {...attributes}
        {...listeners}
    >
      <code
        style={style}
        onClick={() => setSelectedObject({ type: "CIRCLE", ...props.data })}
      >{`<circle 
        cx="${props.data.pos.x}"
        cy="${props.data.pos.y}" 
        r="${props.data.radius}"
        fill="rgba(${fillRed}, ${fillGreen}, ${fillBlue}, ${fillOpacity})"
        stroke="rgba(${strokeRed}, ${strokeGreen}, ${strokeBlue}, ${strokeOpacity})"  
        stroke-width="${props.data.stroke_width}"
        opacity="${opacity}"/>`
      }
      </code>
    </PaddedDiv>
  )
}

type RectangleCodeProps = {
  data: SVGRectangle,
  selectedObjectState: ReactSVGObjectState
}

const RectangleCode: FC<RectangleCodeProps> = props => {
  const [fillRed, fillGreen, fillBlue, fillOpacity] = props.data.fill;
  const [strokeRed, strokeGreen, strokeBlue, strokeOpacity] = props.data.stroke;
  const [selectedObject, setSelectedObject] = props.selectedObjectState;
  const background = isObjectSelected(props.data.id, selectedObject) ? selectedColor : unselectedColor;
  const style = { background, cursor: "pointer" };
  const opacity = props.data.opacity;
  return (
    <PaddedDiv>
      <code
        onClick={() => setSelectedObject({ type: "RECTANGLE", ...props.data })}
        style={style}
      >{`<rect
      x="${props.data.pos.x}" 
      y="${props.data.pos.y}" 
      width="${props.data.width}" 
      height="${props.data.height}" 
      fill="rgba(${fillRed}, ${fillGreen}, ${fillBlue}, ${fillOpacity})"
      stroke-width="${props.data.stroke_width}"
      stroke="rgba(${strokeRed}, ${strokeGreen}, ${strokeBlue}, ${strokeOpacity})"
      opacity="${opacity}"/>`}
      </code>
    </PaddedDiv>
  )
};

type GroupCodeProps = {
  data: SVGGroup,
  docRef: MutableRefObject<SVGDoc>,
  selectedObjectState: ReactSVGObjectState
  fetchSVGDoc: () => void,
}

const GroupCode: FC<GroupCodeProps> = props => {
  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 8
      }
    })
  );
  const { 
    attributes, 
    listeners, 
    setNodeRef,
    transform,
    transition
  } = useSortable({ id: props.data.id });
  const [selectedObject, setSelectedObject] = props.selectedObjectState;
  const background = isObjectSelected(props.data.id, selectedObject) ? selectedColor : unselectedColor;
  const onClick = useCallback(() => setSelectedObject({ type: "GROUP", ...props.data }), [props, setSelectedObject]);
  const style = { background, cursor: "pointer" };
  const divStyle: CSSProperties = {
    transition, 
    transform: transform ? `translate3d(${transform.x}px, ${transform.y}px, 0)` : undefined
  };

  const onDragEnd = useCallback((event: DragEndEvent) => {
    console.log("Group Drag Event End", event);
  }, [])
  return (
    <PaddedDiv
      ref={setNodeRef}
      style={divStyle}
      {...listeners}
      {...attributes}
    >
      <code
        style={style}
        onClick={onClick}
      >{`<g id=${props.data.id}>`}</code>
        <DndContext
          collisionDetection={closestCenter} 
          onDragEnd={onDragEnd}
          sensors={sensors}
        >
          <SortableContext items={props.data.children} strategy={verticalListSortingStrategy}>
            {props.data.children.map(mapper(props.docRef, props.fetchSVGDoc, props.selectedObjectState))}
          </SortableContext>
        </DndContext>
      <code style={style} onClick={onClick} >{"</g>"}</code>
    </PaddedDiv>
  )
};

type PathCodeProps = {
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

const PathCode: FC<PathCodeProps> = (props) => {
  const path = props.data.points.map((p, i) => [p.id, `${i == 0 ? "" : " "}${toPathString(p)}`]);
  const [selectedObject, setSelectedObject] = props.selectedObjectState;
  const onClick = useCallback(() => {
    setSelectedObject({ type: "PATH", ...props.data});
  }, [setSelectedObject, props]);
  const background = isObjectSelected(props.data.id, selectedObject) ? selectedColor : unselectedColor;
  const style = { background, cursor: "pointer" };
  return (
    <PaddedDiv>
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


function App() {
  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 8
      }
    })
  );
  const SVGDocRef = useRef(SVGDoc.new());
  const [inner, setInner] = useState<SVGDocInner>({ children: [] });
  const [selectedObject, setSelectedObject] = useState<SVGObject | "root">("root");
  const fetchSVGDoc = (id?: string) => {
    const doc = SVGDocRef.current;
    setInner(doc.children());
    if (id) {
      const circle = doc.get_circle(id);
      if (!circle) return;
      setSelectedObject({ type: "CIRCLE", ...circle });
    }
  }
  useEffect(() => {
    fetchSVGDoc();
  }, []);
  const onDragEnd = useCallback((event: DragEndEvent) => {
    console.log("onDragEnd", event);
  }, []);
  const background = isObjectSelected("root", selectedObject) ? selectedColor : unselectedColor;
  const style: CSSProperties = { background, cursor: "pointer" };
  return (
    <>
      <div>
        <code style={style} onClick={() => setSelectedObject("root")}>{"<svg>"}</code>
        <DndContext
          collisionDetection={closestCenter} 
          onDragEnd={onDragEnd}
          sensors={sensors}
        >
          <SortableContext items={inner.children} strategy={verticalListSortingStrategy}>
            {
              inner.children.map(mapper(
                SVGDocRef,
                fetchSVGDoc,
                [selectedObject, setSelectedObject]
              ))
            }
          </SortableContext>
        </DndContext>
        <code style={style} onClick={() => setSelectedObject("root")}>{"</svg>"}</code>
      </div>
      <Configuration
        docRef={SVGDocRef}
        fetchSVGDoc={fetchSVGDoc}
        data={selectedObject}
      />
      <AllFeatures/>
    </>
  )
}

export default App
