import { CSSProperties, FC, MutableRefObject, useCallback } from "react";
import { SVGCircle, SVGDoc, SVGGroup, SVGObject, SVGPath, SVGRectangle } from "vect-crdt-rs";
import { DivProps } from "../../types";

type ConfigurationRootProps = {
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: () => void,
}

const ConfigurationRoot: FC<ConfigurationRootProps> = (props) => {
    const onClickAddCircle = useCallback(() => {
        props.docRef.current.add_circle(undefined, {});
        props.fetchSVGDoc();
    }, [props]);

    const onClickAddRectangle = useCallback(() => {
        props.docRef.current.add_rectangle(undefined, {});
        props.fetchSVGDoc();
    }, [props]);

    const onClickAddGroup = useCallback(() => {
        props.docRef.current.add_group(undefined, {});
        props.fetchSVGDoc();
    }, [props]);

    return (
        <>
            <button
                onClick={onClickAddCircle}
            >Add Circle</button>
            <button
                onClick={onClickAddRectangle}
            >
                Add Rectangle
            </button>
            <button
                onClick={onClickAddGroup}
            >
                Add Group
            </button>
        </>
    )
}

type ConfigurationCircleProps = {
    data: SVGCircle,
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: (id?: string) => void,
}

const TwoColumnGrid: FC<DivProps> = (props) => {
    const { children, style: propsStyle, ...otherProps } = props;
    const defaultStyle: CSSProperties = { display: "grid", gridTemplateColumns: "1fr 1fr" };
    const style = { ...defaultStyle, ...propsStyle };
    return (
        <div style={style} { ...otherProps }>{ children }</div>
    )
}

const ConfigurationCircle: FC<ConfigurationCircleProps> = (props) => {
    const onChangeCx = useCallback((s: string) => { 
        const x = parseInt(s);
        if (!x) return;
        
        const pos = structuredClone(props.data.pos);
        pos.x = x;
        props.docRef.current.edit_circle(props.data.id, { pos });
        props.fetchSVGDoc(props.data.id);
    }, [props]);

    const onChangeCy = useCallback((s: string) => {
        const y = parseInt(s);
        if (!y) return;
        const pos = structuredClone(props.data.pos);
        pos.y = y;
        props.docRef.current.edit_circle(props.data.id, { pos });
        props.fetchSVGDoc(props.data.id);
    }, [props]);

    const onChangeRadius = useCallback((s: string) => {
        const radius = parseInt(s);
        if (!radius) return;
        props.docRef.current.edit_circle(props.data.id, { radius });
        props.fetchSVGDoc(props.data.id);
    }, [props]);

    const onChangeStrokeWidth = useCallback((s: string) => {
        const strokeWidth = parseInt(s);
        if (!strokeWidth) return;
        props.docRef.current.edit_circle(props.data.id, { stroke_width: strokeWidth });
        props.fetchSVGDoc(props.data.id);
    }, [props]);

    return (
        <>
            <div>Circle</div>
            <TwoColumnGrid style={{width: "20%"}}>
                <label htmlFor="">cx</label>
                <input value={props.data.pos.x} type="number" onChange={e => onChangeCx(e.target.value)}/>
                <label htmlFor="">cy</label>
                <input value={props.data.pos.y} type="number" onChange={e => onChangeCy(e.target.value)}/>
                <label htmlFor="">radius</label>
                <input value={props.data.radius} type="number" onChange={e => onChangeRadius(e.target.value)}/>
                <label htmlFor="">Stroke Width</label>
                <input value={props.data.stroke_width} type="number" onChange={e => onChangeStrokeWidth(e.target.value)}/>
            </TwoColumnGrid>
        </>
    );
}

type ConfigurationRectangleProps = {
    data: SVGRectangle
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: () => void,
}

const ConfigurationRectangle: FC<ConfigurationRectangleProps> = () => {
    return (
        <>rectangle selected</>
    );
}

type ConfigurationGroupProps = {
    data: SVGGroup,
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: () => void,
}

const ConfigurationGroup: FC<ConfigurationGroupProps> = (props) => {

    const onClickAddGroup = useCallback(() => {
        props.docRef.current.add_group(props.data.id, {});
        props.fetchSVGDoc();
    }, [props]);


    const onClickAddCircle = useCallback(() => {
        props.docRef.current.add_circle(props.data.id, {});
        props.fetchSVGDoc();
    }, [props]);

    const onClickAddRectangle = useCallback(() => {
        props.docRef.current.add_rectangle(props.data.id, {});
        props.fetchSVGDoc();
    }, [props]);
    return (
        <>
            <button
                onClick={onClickAddCircle}
            >
                Add Circle
            </button>
            <button
                onClick={onClickAddRectangle}
            >
                Add Rectangle
            </button>
            <button
                onClick={onClickAddGroup}
            >
                Add Group
            </button>
        </>
    );
}

type ConfigurationPathProps = {
    data: SVGPath
}

const ConfigurationPath: FC<ConfigurationPathProps> = () => {
    return (
        <>Path Selected</>
    );
}

type ConfigurationProps = {
    data: SVGObject | "root";
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: (id?: string) => void,
};

export const Configuration: FC<ConfigurationProps> = (props) => {
    return (
        <div>
            {configurationMapper(props.data, props.docRef, props.fetchSVGDoc)}
        </div>
    )
}

// utility functions
const configurationMapper = (
    data: SVGObject | "root",
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: (id?: string) => void,
) => {
    if (data === "root") {
        return (
            <ConfigurationRoot
                docRef={docRef}
                fetchSVGDoc={fetchSVGDoc}
            />
        )
    }
    switch (data.type) {
        case "CIRCLE":
            return <ConfigurationCircle
                fetchSVGDoc={fetchSVGDoc}
                docRef={docRef}
                data={data}
            />
        case "RECTANGLE":
            return <ConfigurationRectangle
                fetchSVGDoc={fetchSVGDoc}
                docRef={docRef}
                data={data}
            />
        case "GROUP":
            return <ConfigurationGroup
                docRef={docRef}
                fetchSVGDoc={fetchSVGDoc}
                data={data}
            />
        case "PATH":
            return <ConfigurationPath
                data={data}
            />
    }
}