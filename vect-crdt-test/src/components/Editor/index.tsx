import { FC, MutableRefObject, useCallback, useEffect, useState } from 'react';
import { SVGDoc } from "@brynghiffar/vect-crdt-rs";
import { Configuration } from '../../components/Configuration';
import { DroppableSVG, ReactSVGObjectState, SelectedObject } from '../../types';
import { DndContext, DragEndEvent, PointerSensor, closestCenter, useSensor, useSensors } from '@dnd-kit/core';
import { SortableContext, verticalListSortingStrategy } from '@dnd-kit/sortable';
import { CircleCode } from '../../components/Code/circle';
import { RectangleCode } from '../../components/Code/rectangle';
import { PathCode } from '../../components/Code/path';
import { GroupCloseCode, GroupOpenCode } from '../../components/Code/group';
import { getGroupId, getIndexInGroup } from '../../utility/methods';
import { CodeFragment } from '../Code/CodeFragment';

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
          data={group}
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




type EditorProps = {
    docId: string,
    fetchSVGDoc: (initial?: boolean) => void,
    droppableSVG: DroppableSVG[],
    doc: MutableRefObject<SVGDoc>
}

export const Editor: FC<EditorProps> = (props) => {
  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 8
      }
    })
  );
  const SVGDocRef = props.doc;
  const { fetchSVGDoc, droppableSVG } = props;
  const [selectedObject, setSelectedObject] = useState<SelectedObject>("root");
  useEffect(() => {
    fetchSVGDoc(true);
  }, [fetchSVGDoc]);
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
    if (overGroupId === "root") {
      SVGDocRef.current.move_object_to_root(activeId, overIndexInGroup);
    } else {
      SVGDocRef.current.move_object_to_group(activeId, overGroupId, overIndexInGroup);
    }
    fetchSVGDoc();
  }, [droppableSVG, fetchSVGDoc, SVGDocRef]);
  const selected = "root" === selectedObject;
  return (
    <div style={{minWidth: "0px", minHeight: "0px"}} className="grid gap-2">
      <div className="border-gray-600 border bg-[#1F1F1F] rounded-lg p-2 text-white font-extrabold text-lg">
        Client {props.docId}
      </div>
      <div className="overflow-x-scroll border-gray-600 border bg-[#1F1F1F] rounded-lg p-2 scroll" style={{ scrollbarWidth: "thin" }}>
        <CodeFragment selected={selected} onClick={() => setSelectedObject("root")}>{"<svg>"}</CodeFragment>
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
        <CodeFragment selected={selected} onClick={() => setSelectedObject("root")}>{"</svg>"}</CodeFragment>
      </div>
      <Configuration
        docRef={SVGDocRef}
        fetchSVGDoc={fetchSVGDoc}
        selectedObject={selectedObject}
      />
    </div>
  )
}