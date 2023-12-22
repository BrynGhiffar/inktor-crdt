import { CSSProperties, Dispatch, FC, MutableRefObject, SetStateAction, forwardRef, useCallback, useEffect, useRef, useState } from 'react';
import { SVGDoc, SVGDocInner, SVGRectangle, SVGGroup, SVGPath, SVGPathCommand } from "vect-crdt-rs";
import './App.css';
import { Configuration } from './components/Configuration';
import { DivProps, SelectedObject } from './types';
import { DndContext, DragEndEvent, PointerSensor, closestCenter, useSensor, useSensors } from '@dnd-kit/core';
import { SortableContext, useSortable, verticalListSortingStrategy } from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';

type ReactSetSVGObject = Dispatch<SetStateAction<SelectedObject>>;
type ReactSVGObjectState = [SelectedObject, ReactSetSVGObject];


const isObjectSelected = (id: string, obj: SelectedObject) => {
  if (obj === "root") return id === "root";
  return obj.id === id;
}

const selectedColor = "lightgray";
const unselectedColor = "transparent";

type PaddedDivProps = DivProps & {
  depth?: number,
}

const PaddedDiv = forwardRef<HTMLDivElement, PaddedDivProps>((props, ref) => {
  const { depth, children, style: propsStyle, ...divProps } = props;
  const style = { paddingLeft: `${20 * (depth ?? 1)}px`, ...propsStyle };
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
  return (obj: DroppableSVG) => {
    switch (obj.type) {
      case "CIRCLE_TAG":
        return <CircleCode
          depth={obj.depth}
          data={obj}
          key={obj.id}
          selectedObjectState={selectedObjectState}
          docRef={docRef}
        />
      case "RECTANGLE_TAG": {
        const rectangle = docRef.current.get_rectangle(obj.id);
        if (rectangle === undefined) return (<></>);
        return <RectangleCode
          depth={obj.depth}
          data={rectangle}
          key={obj.id}
          selectedObjectState={selectedObjectState}
        />
      }
      case "GROUP_START_TAG": {
        const group = structuredClone(docRef.current.get_group(obj.id));
        if (group === undefined) return (<></>);
        return <GroupOpenCode
          key={`START_${obj.id}`}
          depth={obj.depth}
          docRef={docRef}
          id={obj.id}
          fetchSVGDoc={fetchSVGDoc}
          selectedObjectState={selectedObjectState}
        />
      }
      case "GROUP_END_TAG": {
        return <GroupCloseCode
          key={`END_${obj.id}`}
          depth={obj.depth}
          docRef={docRef}
          id={obj.id}
          fetchSVGDoc={fetchSVGDoc}
          selectedObjectState={selectedObjectState}
        />
      }
      case "PATH_TAG": {
        const path = docRef.current.get_path(obj.id);
        if (path === undefined) return (<></>);
        return <PathCode
          depth={obj.depth}
          key={obj.id}
          data={path}
          selectedObjectState={selectedObjectState}
        />
      }
      default:
        return <></>
    }
  }

}

type CircleCodeProps = {
  depth: number,
  data: DroppableSVG,
  selectedObjectState: ReactSVGObjectState,
  docRef: MutableRefObject<SVGDoc>
}


const CircleCode: FC<CircleCodeProps> = (props) => {
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
      depth={props.depth}
      style={divStyle}
        ref={setNodeRef}
        {...attributes}
        {...listeners}
    >
      <code
        style={style}
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
      </code>
    </PaddedDiv>
  )
}

type RectangleCodeProps = {
  depth: number,
  data: SVGRectangle,
  selectedObjectState: ReactSVGObjectState
}

const RectangleCode: FC<RectangleCodeProps> = props => {
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
  const background = isObjectSelected(props.data.id, selectedObject) ? selectedColor : unselectedColor;
  const style = { background, cursor: "pointer" };
  const opacity = props.data.opacity;
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
  id: string,
  depth: number,
  docRef: MutableRefObject<SVGDoc>,
  selectedObjectState: ReactSVGObjectState
  fetchSVGDoc: () => void,
}

const GroupOpenCode: FC<GroupCodeProps> = props => {
  const { 
    attributes, 
    listeners, 
    setNodeRef,
    transform,
    transition
  } = useSortable({ id: props.id });
  const [selectedObject, setSelectedObject] = props.selectedObjectState;
  const background = isObjectSelected(props.id, selectedObject) ? selectedColor : unselectedColor;
  const onClick = useCallback(() => setSelectedObject({ type: "GROUP", id: props.id }), [props, setSelectedObject]);
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

const GroupCloseCode: FC<GroupCloseCodeProps> = (props) => {
  const realId = props.id.replace("END_", "");
  const { 
    attributes, 
    listeners, 
    setNodeRef,
    transform,
    transition
  } = useSortable({ id: props.id, disabled: true });
  const [selectedObject, setSelectedObject] = props.selectedObjectState;
  const background = isObjectSelected(realId, selectedObject) ? selectedColor : unselectedColor;
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

const PathCode: FC<PathCodeProps> = (props) => {
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
  const background = isObjectSelected(props.data.id, selectedObject) ? selectedColor : unselectedColor;
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

export type DroppableSVG = ({ 
    type: "CIRCLE_TAG", 
    depth: number,
    id: string,
  }) | 
  ({ 
    type: "RECTANGLE_TAG",
    depth: number,
    id: string,
  }) | 
  ({ 
    type: "PATH_TAG",
    depth: number,
    id: string,
  })| 
  ({
    type: "GROUP_START_TAG",
    depth: number,
    id: string,
  }) |
  ({
    type: "GROUP_END_TAG",
    depth: number,
    id: string,
  })
  ;

const flattenSVGGroup = (group: SVGGroup, depth: number): DroppableSVG[] => {
  const result: DroppableSVG[] = [];
  const childDepth = depth + 1;
  for (const child of group.children) {
    switch (child.type) {
      case "CIRCLE":
        result.push({ type: "CIRCLE_TAG", depth: childDepth, id: child.id })
        break;
      case "RECTANGLE":
        result.push({ type: "RECTANGLE_TAG", depth: childDepth, id: child.id })
        break;
      case "PATH":
        result.push({ type: "PATH_TAG", depth: childDepth, id: child.id })
        break;
      case "GROUP":
        result.push({ type: "GROUP_START_TAG", depth: childDepth, id: child.id })
        result.push(...flattenSVGGroup(child, childDepth));
        result.push({ type: "GROUP_END_TAG", depth: childDepth, id: `END_${child.id}` });
        break;
    }
  }
  return result;
}

const flattenSVGDocInner = (inner: SVGDocInner) => {
  const result: DroppableSVG[] = []
  const depth = 1;
  for (const child of inner.children) {
    const { id } = child;
    switch (child.type) {
      case "CIRCLE":
        result.push({ type: "CIRCLE_TAG", depth, id });
        break;
      case "RECTANGLE":
        result.push({ type: "RECTANGLE_TAG", depth, id });
        break;
      case "PATH":
        result.push({ type: "PATH_TAG", depth, id });
        break;
      case "GROUP":
        result.push({ type: "GROUP_START_TAG", depth, id: `${id}` });
        result.push(...flattenSVGGroup(child, depth))
        result.push({ type: "GROUP_END_TAG", depth, id: `END_${id}` });
    }
  }
  return result;
};

const getGroupId = (id: string, flattenTree: DroppableSVG[]): string | null => {
  // returns "root" when element not in group.
  // if null is returned that means id does not exist.
  const index = flattenTree.findIndex(v => v.id === id);
  if (index === -1) return null;
  const n = flattenTree.length;
  let i = index + 1;
  while ( i < n ) {
    const current = flattenTree[i];
    if (current.type === "GROUP_END_TAG" && current.id === `END_${id}`) {
      i++;
      continue;
    }
    if (flattenTree[i].type === "GROUP_END_TAG") {
      return flattenTree[i].id.replace("END_", "");
    }
    if (current.type === "GROUP_START_TAG") {
      i++;
      const groupId = current.id;
      while ( i < n ) {
        const next = flattenTree[i];
        if (next.type === "GROUP_END_TAG" && next.id === `END_${groupId}`) {
          i++;
          break;
        }
        i++;
      }
    } else {
      i++;
    }

  }
  return "root";
}

// const getGroupIdFromIndex = (index: number, flattenTree: DroppableSVG): number | null => {

//   return null;
// }

const getIndexInGroup = (
  groupId: string, 
  id: string, 
  flattenTree: DroppableSVG[]
): number | null => {
  const groupIdIndex = groupId === "root" ? -1 : flattenTree.findIndex(item => item.id === groupId);
  const groupIdEndIndex = groupId === "root" ? flattenTree.length :flattenTree.findIndex(item => item.id === `END_${groupId}`);
  if (groupIdIndex === -1 && groupId !== "root") return null;
  if (groupIdEndIndex === undefined) return null;
  let itemsBehind = 0;
  let curr = groupIdIndex + 1;
  while (curr < groupIdEndIndex) {
    const current = flattenTree[curr];
    if (current.id === id.replace("END_", "")) { return itemsBehind; }
    if (current.type === "GROUP_START_TAG") {
      const next = flattenTree.findIndex(v => v.id === `END_${current.id}`);
      if (next === -1) return null;
      curr = next + 1;
    } else {
      curr++;
    }
    itemsBehind++;
  }
  return itemsBehind;
};

function App() {
  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 8
      }
    })
  );
  const SVGDocRef = useRef(SVGDoc.new());
  // const [inner, setInner] = useState<SVGDocInner>({ children: [] });
  const [droppableSVG, setDroppableSVG] = useState<DroppableSVG[]>([]);
  const [selectedObject, setSelectedObject] = useState<SelectedObject>("root");
  const fetchSVGDoc = () => {
    const doc = SVGDocRef.current;
    // setInner(doc.children());
    const flattened = flattenSVGDocInner(doc.children());
    setDroppableSVG(flattened);
  }
  useEffect(() => {
    fetchSVGDoc();
  }, []);
  const onDragEnd = useCallback((event: DragEndEvent) => {
    if (event.over === null) return;
    const activeId = event.active.id;
    const overId = event.over.id;
    if (typeof activeId !== 'string') return;
    if (typeof overId !== 'string') return;
    const oldActiveIndex = droppableSVG.findIndex(it => it.id === activeId);
    const oldOverIndex = droppableSVG.findIndex(it => it.id === overId);

    let oldActiveEndIndex = droppableSVG.findIndex(it => it.id === `END_${activeId}`);
    if (oldActiveEndIndex === -1) {
      oldActiveEndIndex = oldActiveIndex;
    }
    let mockA = [...droppableSVG];
    if (oldActiveIndex < oldOverIndex) {
      mockA = [
        ...mockA.slice(0, oldActiveIndex),
        ...mockA.slice(oldActiveEndIndex + 1, oldOverIndex + 1),
        { type: "CIRCLE_TAG", id: "TARGET", depth: 0 },
        ...Array(oldActiveEndIndex - oldActiveIndex).fill({ type: "CIRCLE_TAG", id: "_", depth: 0 }),
        ...mockA.slice(oldOverIndex + 1)
      ]
    } else {
      mockA = [
        ...mockA.slice(0, oldOverIndex),
        { type: "CIRCLE_TAG", id: "TARGET", depth: 0 },
        ...Array(oldActiveEndIndex - oldActiveIndex).fill({ type: "CIRCLE_TAG", id: "_", depth: 0 }),
        ...mockA.slice(oldOverIndex, oldActiveIndex),
        ...mockA.slice(oldActiveEndIndex + 1)
      ]
    }
    const mockArray = mockA;
    const overGroupId = getGroupId("TARGET", mockArray);
    if (overGroupId === null) return;
    const overIndexInGroup = getIndexInGroup(overGroupId, "TARGET", mockArray);
    if (overIndexInGroup === null) {
      return;
    }
    // console.log(`Move ${activeId} to group ${overGroupId}, index ${overIndexInGroup}`);
    if (overGroupId === "root") {
      SVGDocRef.current.move_object_to_root(activeId, overIndexInGroup);
    } else {
      SVGDocRef.current.move_object_to_group(activeId, overGroupId, overIndexInGroup);
    }
    fetchSVGDoc();
  }, [droppableSVG]);
  const background = "root" === selectedObject ? selectedColor : unselectedColor;
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
          <SortableContext items={droppableSVG} strategy={verticalListSortingStrategy}>
            {
              droppableSVG.map(mapper(
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
        selectedObject={selectedObject}
      />
    </>
  )
}

export default App
