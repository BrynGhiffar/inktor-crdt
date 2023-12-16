import { CSSProperties, Dispatch, FC, MutableRefObject, SetStateAction, useCallback, useEffect, useRef, useState } from 'react';
import { SVGDoc, SVGCircle, SVGDocInner, SVGObject, SVGRectangle, SVGGroup, SVGPath, SVGPathCommand } from "vect-crdt-rs";
import './App.css';
import { Configuration } from './components/Configuration';
import { DivProps } from './types';

type ReactSetSVGObject = Dispatch<SetStateAction<SVGObject | "root">>;
type ReactSVGObjectState = [SVGObject | "root", ReactSetSVGObject];



const isObjectSelected = (id: string, obj: SVGObject | "root") => {
  if (obj === "root") return id === "root";
  return obj.id === id;
}

const selectedColor = "lightgray";
const unselectedColor = "transparent";

const PaddedDiv: FC<DivProps> = (props) => {
  const { children, ...divProps } = props;
  return (
    <div
      {...divProps}
      style={{ paddingLeft: "20px" }}
    >{children}</div>
  )
}

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
          docRef={docRef}
          data={obj}
          fetchSVGDoc={fetchSVGDoc}
          selectedObjectState={selectedObjectState}
        />
      case "PATH":
        return <PathCode
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
  const [red, green, blue, __opacity] = props.data.fill;
  const [strokeRed, strokeGreen, strokeBlue, __strokeOpacity ] = props.data.stroke;
  const hex = rgbToHex(red, green, blue);
  const strokeColor = rgbToHex(strokeRed, strokeGreen, strokeBlue);
  const opacity = __opacity / 100;
  const [selectedObject, setSelectedObject] = props.selectedObjectState;
  const background = isObjectSelected(props.data.id, selectedObject) ? selectedColor : unselectedColor;
  const style = { background, cursor: "pointer" };
  return (
    <PaddedDiv>
      <code
        style={style}
        onClick={() => setSelectedObject({ type: "CIRCLE", ...props.data })}
      >{`<circle 
        cx="${props.data.pos.x}"
        cy="${props.data.pos.y}" 
        r="${props.data.radius}"
        fill="${hex}"
        stroke="${strokeColor}"  
        stroke-width="${props.data.stroke_width}"
        opacity="${opacity}"/>`}
      </code>
    </PaddedDiv>
  )
}

type RectangleCodeProps = {
  data: SVGRectangle,
  selectedObjectState: ReactSVGObjectState
}

const RectangleCode: FC<RectangleCodeProps> = props => {
  const [redFill, greenFill, blueFill, __opacity] = props.data.fill;
  const hexFill = rgbToHex(redFill, greenFill, blueFill);
  const opacity = __opacity / 100;
  const [selectedObject, setSelectedObject] = props.selectedObjectState;
  const background = isObjectSelected(props.data.id, selectedObject) ? selectedColor : unselectedColor;
  const style = { background, cursor: "pointer" };
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
      fill="${hexFill}"
      stroke-width="${props.data.stroke_width}"
      stroke="${props.data.stroke}"
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

  const [selectedObject, setSelectedObject] = props.selectedObjectState;
  const background = isObjectSelected(props.data.id, selectedObject) ? selectedColor : unselectedColor;
  const style = { background, cursor: "pointer" };
  const onClick = useCallback(() => setSelectedObject({ type: "GROUP", ...props.data }), [props, setSelectedObject]);

  return (
    <PaddedDiv
    >
      <code
        style={style}
        onClick={onClick}
      >{`<g id=${props.data.id}>`}</code>
      {props.data.children.map(mapper(props.docRef, props.fetchSVGDoc, props.selectedObjectState))}
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
  const path = props.data.points.map((p, i) => `${i == 0 ? "" : " "}${toPathString(p)}`);
  return (
    <PaddedDiv>
      <code>{"<path d=\""}</code>
      {
        path.map(p => (
          <code>{p}</code>
        ))
      }
      <code>{"\"/>"}</code>
    </PaddedDiv>
  )
}


function App() {
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
  const background = isObjectSelected("root", selectedObject) ? selectedColor : unselectedColor;
  const style: CSSProperties = { background, cursor: "pointer" };
  return (
    <>
      <div>
        <code style={style} onClick={() => setSelectedObject("root")}>{"<svg>"}</code>
        {inner.children.map(mapper(
          SVGDocRef,
          fetchSVGDoc,
          [selectedObject, setSelectedObject]
        ))
        }
        <code style={style} onClick={() => setSelectedObject("root")}>{"</svg>"}</code>
      </div>
      <Configuration
        docRef={SVGDocRef}
        fetchSVGDoc={fetchSVGDoc}
        data={selectedObject}
      />
    </>
  )
}

export default App
