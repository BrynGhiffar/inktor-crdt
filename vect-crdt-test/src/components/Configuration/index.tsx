import { CSSProperties, FC, MutableRefObject, useCallback } from "react";
import { SVGDoc, SVGPathCommand, SVGPathCommandType } from "vect-crdt-rs";
import { DivProps, SelectedObject, SelectedSVG } from "../../types";
import { RgbaColor, RgbaColorPicker } from "react-colorful";
import { z } from "zod";

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

    const onClickAddPath = useCallback(() => {
        props.docRef.current.add_path(undefined, {});
        props.fetchSVGDoc();
    }, [props])

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
            <button
                onClick={onClickAddPath}
            >Add Path</button>
        </>
    )
}

type ConfigurationCircleProps = {
    data: SelectedSVG,
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

const NColumnGrid: FC<DivProps & { nColumns: number }> = (props) => {
    const { children, style: propsStyle, nColumns, ...otherProps } = props;
    const defaultStyle: CSSProperties = { display: "grid", gridTemplateColumns: `repeat(${nColumns}, 1fr)` };
    const style = { ...defaultStyle, ...propsStyle };
    return (
        <div style={style} {...otherProps}>{children}</div>
    )
}

const px = (num: number) => `${num}px`;

const ConfigurationCircle: FC<ConfigurationCircleProps> = (props) => {
    const docRef = props.docRef;
    const fetchSVGDoc = props.fetchSVGDoc;
    const circle = docRef.current.get_circle(props.data.id);

    const onChangeCx = useCallback((s: string) => { 
        const x = parseInt(s);
        if (circle === undefined) return;
        if (Number.isNaN(x)) return;
        
        const pos = structuredClone(circle.pos);
        pos.x = x;
        docRef.current.edit_circle(circle.id, { pos });
        fetchSVGDoc();
    }, [docRef, fetchSVGDoc, circle]);

    const onChangeCy = useCallback((s: string) => {
        const y = parseInt(s);
        if (circle === undefined) return;
        if (Number.isNaN(y)) return;

        const pos = structuredClone(circle.pos);
        pos.y = y;
        docRef.current.edit_circle(circle.id, { pos });
        fetchSVGDoc();
    }, [docRef, fetchSVGDoc, circle]);

    const onChangeRadius = useCallback((s: string) => {
        const radius = parseInt(s);
        if (circle === undefined) return;
        if (Number.isNaN(radius)) return;

        docRef.current.edit_circle(circle.id, { radius });
        fetchSVGDoc();
    }, [docRef, fetchSVGDoc, circle]);

    const onChangeStrokeWidth = useCallback((s: string) => {
        const strokeWidth = parseInt(s);
        if (circle === undefined) return;
        if (Number.isNaN(strokeWidth)) return;
        docRef.current.edit_circle(circle.id, { stroke_width: strokeWidth });
        fetchSVGDoc();
    }, [docRef, fetchSVGDoc, circle]);

    const onChangeOpacity = useCallback((s: string) => {
        const opacity = parseFloat(s);
        if (circle === undefined) return;
        if (Number.isNaN(opacity)) return;

        docRef.current.edit_circle(circle.id, { opacity: opacity });
        fetchSVGDoc()
    }, [docRef, fetchSVGDoc, circle]);

    const onChangeFill = useCallback((color: RgbaColor) => {
        if (circle === undefined) return;
        docRef.current.edit_circle(circle.id, { fill: [color.r, color.g, color.b, color.a]});
        fetchSVGDoc();
    }, [docRef, fetchSVGDoc, circle]);

    const onChangeStroke = useCallback((color: RgbaColor) => {
        if (circle === undefined) return;
        docRef.current.edit_circle(circle.id, { stroke: [ color.r, color.g, color.b, color.a]});
        fetchSVGDoc();
    }, [docRef, fetchSVGDoc, circle]);

    if (circle === undefined) { return (<></>)}
    const [ fillRed, fillGreen, fillBlue, fillOpacity ] = circle.fill;
    const [ strokeRed, strokeGreen, strokeBlue, strokeOpacity ] = circle.stroke;

    return (
        <>
            <div>Circle</div>
            <TwoColumnGrid style={{width: "20%", gap: `${px(3)} ${px(20)}`}}>
                <label htmlFor="">cx</label>
                <input value={circle.pos.x} type="number" onChange={e => onChangeCx(e.target.value)}/>
                <label htmlFor="">cy</label>
                <input value={circle.pos.y} type="number" onChange={e => onChangeCy(e.target.value)}/>
                <label htmlFor="">radius</label>
                <input value={circle.radius} type="number" onChange={e => onChangeRadius(e.target.value)}/>
                <label htmlFor="">Stroke Width</label>
                <input value={circle.stroke_width} type="number" onChange={e => onChangeStrokeWidth(e.target.value)}/>
                <label htmlFor="">Opacity</label>
                <input type="range" min={0} max={1} step={0.01} onChange={ e => onChangeOpacity(e.target.value)} value={circle.opacity}/>
                <label>Fill</label>
                <label>Stroke</label>
                <RgbaColorPicker onChange={onChangeFill} color={{ r: fillRed, g: fillGreen, b: fillBlue, a: fillOpacity }}/>
                <RgbaColorPicker onChange={onChangeStroke} color={{r: strokeRed, g: strokeGreen, b: strokeBlue, a: strokeOpacity }}/>
            </TwoColumnGrid>
        </>
    );
}

type ConfigurationRectangleProps = {
    data: SelectedSVG
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: () => void,
}

const ConfigurationRectangle: FC<ConfigurationRectangleProps> = (props) => {
    const { fetchSVGDoc, docRef, data: { id }} = props;
    const rect = docRef.current.get_rectangle(id);
    const onChangeCx = useCallback((s: string) => {
        const x = parseInt(s);
        if (Number.isNaN(x)) return;
        if (rect === undefined) return;
        const pos = structuredClone(rect.pos);
        docRef.current.edit_rectangle(rect.id, { pos: { ...pos, x }});
        fetchSVGDoc();
    }, [fetchSVGDoc, docRef, rect]);

    const onChangeCy = useCallback((s: string) => {
        const y = parseInt(s);
        if (Number.isNaN(y)) return;
        if (rect === undefined) return;
        const pos = structuredClone(rect.pos);
        docRef.current.edit_rectangle(rect.id, { pos: { ...pos, y }});
        fetchSVGDoc();
    }, [fetchSVGDoc, docRef, rect]);

    const onChangeWidth = useCallback((s: string) => {
        const width = parseInt(s);
        if (Number.isNaN(width)) return;
        if (rect === undefined) return;
        docRef.current.edit_rectangle(rect.id, { width });
        fetchSVGDoc();
    }, [fetchSVGDoc, docRef, rect]);

    const onChangeHeight = useCallback((s: string) => {
        const height = parseInt(s);
        if (Number.isNaN(height)) return;
        if (rect === undefined) return;
        docRef.current.edit_rectangle(rect.id, { height });
        fetchSVGDoc();
    }, [fetchSVGDoc, docRef, rect]);

    const onChangeOpacity = useCallback((s: string) => {
        const opacity = parseFloat(s);
        if (Number.isNaN(opacity)) return;
        if (rect === undefined) return;
        docRef.current.edit_rectangle(rect.id, { opacity });
        fetchSVGDoc();
    }, [fetchSVGDoc, docRef, rect]);

    const onChangeFill = useCallback((color: RgbaColor ) => {
        if (rect === undefined) return;
        docRef.current.edit_rectangle(rect.id, { fill: [ color.r, color.g, color.b, color.a]});
        fetchSVGDoc();
    }, [fetchSVGDoc, docRef, rect]);

    const onChangeStroke = useCallback((color: RgbaColor) => {
        if (rect === undefined) return;
        docRef.current.edit_rectangle(rect.id, { stroke: [ color.r, color.g, color.b, color.a ]});
        fetchSVGDoc();
    }, [fetchSVGDoc, docRef, rect]);

    if (rect === undefined) return (<></>);
    const [ fillRed, fillGreen, fillBlue, fillOpacity ] = rect.fill;
    const [ strokeRed, strokeGreen, strokeBlue, strokeOpacity ] = rect.stroke;
    return (
        <>
            <div>Rectangle</div>
            <TwoColumnGrid style={{width: "20%", gap: `${px(3)} ${px(20)}`}}>
                <label htmlFor="">cx</label>
                <input value={rect.pos.x} type="number" onChange={e => onChangeCx(e.target.value)}/>
                <label htmlFor="">cy</label>
                <input value={rect.pos.y} type="number" onChange={e => onChangeCy(e.target.value)}/>
                <label htmlFor="">width</label>
                <input value={rect.width} type="number" onChange={e => onChangeWidth(e.target.value)}/>
                <label htmlFor="">height</label>
                <input value={rect.height} type="number" onChange={e => onChangeHeight(e.target.value)}/>
                <label htmlFor="">opacity</label>
                <input type="range" min={0} max={1} step={0.01} onChange={ e => onChangeOpacity(e.target.value)} value={rect.opacity}/>
                <label>Fill</label>
                <label>Stroke</label>
                <RgbaColorPicker onChange={onChangeFill} color={{ r: fillRed, g: fillGreen, b: fillBlue, a: fillOpacity }}/>
                <RgbaColorPicker onChange={onChangeStroke} color={{r: strokeRed, g: strokeGreen, b: strokeBlue, a: strokeOpacity }}/>
            </TwoColumnGrid>
        </>
    );
}

type ConfigurationGroupProps = {
    data: SelectedSVG,
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

    const onClickAddPath = useCallback(() => {
        props.docRef.current.add_path(props.data.id, {});
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
            <button
                onClick={onClickAddPath}
            >Add Path</button>
        </>
    );
}

type ConfigurationPathCommandRowProps = {
    pathId: string,
    data: SVGPathCommand,
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: () => void,
}

const getPathCommandPosX = (command: SVGPathCommand) => {
    if (command.type === "CLOSE") return 0;
    return command.pos.x;
};

const getPathCommandPosY = (command: SVGPathCommand) => {
    if (command.type === "CLOSE") return 0;
    return command.pos.y;
};

const getPathCommandHandle1X = (command: SVGPathCommand) => {
    if (command.type === "CLOSE") return 0;
    if (command.type === "LINE") return 0;
    if (command.type === "START") return 0;
    if (command.type === "BEZIER_QUAD_REFLECT") return 0;
    if (command.type === "BEZIER") {
        return command.handle1.x;
    }
    if (command.type === "BEZIER_REFLECT") {
        return command.handle.x;
    }
    return command.handle.x
}

const getPathCommandHandle1Y = (command: SVGPathCommand) => {
    if (command.type === "CLOSE") return 0;
    if (command.type === "LINE") return 0;
    if (command.type === "START") return 0;
    if (command.type === "BEZIER_QUAD_REFLECT") return 0;
    if (command.type === "BEZIER") {
        return command.handle1.y;
    }
    if (command.type === "BEZIER_REFLECT") {
        return command.handle.y;
    }
    return command.handle.y
}

const getPathCommandHandle2X = (command: SVGPathCommand) => {
    if (command.type === "BEZIER") return command.handle2.x;
    return 0;
}

const getPathCommandHandle2Y = (command: SVGPathCommand) => {
    if (command.type === "BEZIER") return command.handle2.y;
    return 0;
}

const PathCommandType = z
    .literal("START")
    .or(z.literal("LINE"))
    .or(z.literal("CLOSE"))
    .or(z.literal("BEZIER"))
    .or(z.literal("BEZIER_REFLECT"))
    .or(z.literal("BEZIER_QUAD"))
    .or(z.literal("BEZIER_QUAD_REFLECT"))

const ConfigurationPathCommandRow: FC<ConfigurationPathCommandRowProps> = (props) => {
    const { pathId, data, docRef, fetchSVGDoc } = props;
    const onChangeSelection = useCallback((v: string) => {
        const parseType = PathCommandType.safeParse(v);
        if (!parseType.success) return;
        const type = parseType.data;
        switch (type) {
            case "START":
                docRef.current.edit_path_point_type(pathId, data.id, SVGPathCommandType.START);
                break
            case "LINE":
                docRef.current.edit_path_point_type(pathId, data.id, SVGPathCommandType.LINE);
                break;
            case "CLOSE":
                docRef.current.edit_path_point_type(pathId, data.id, SVGPathCommandType.CLOSE);
                break;
            case "BEZIER":
                docRef.current.edit_path_point_type(pathId, data.id, SVGPathCommandType.BEZIER);
                break;
            case "BEZIER_REFLECT":
                docRef.current.edit_path_point_type(pathId, data.id, SVGPathCommandType.BEZIER_REFLECT);
                break;
            case "BEZIER_QUAD":
                docRef.current.edit_path_point_type(pathId, data.id, SVGPathCommandType.BEZIER_QUAD);
                break;
            case "BEZIER_QUAD_REFLECT":
                docRef.current.edit_path_point_type(pathId, data.id, SVGPathCommandType.BEZIER_QUAD_REFLECT);
                break;
        }
        fetchSVGDoc();
    }, [docRef, fetchSVGDoc, data, pathId]);

    const onChangePosX = useCallback((value: string) => {
        const x = parseInt(value);
        if (Number.isNaN(x)) return;
        const y = structuredClone(getPathCommandPosY(data));
        const pos = { x, y };
        docRef.current.edit_path_point_pos(pathId, data.id, pos);
        fetchSVGDoc();
    }, [docRef, data, fetchSVGDoc, pathId]);

    const onChangePosY = useCallback((value: string) => {
        const y = parseInt(value);
        if (Number.isNaN(y)) return;
        const x = structuredClone(getPathCommandPosX(data));
        const pos = { x, y }
        docRef.current.edit_path_point_pos(pathId, data.id, pos);
        fetchSVGDoc();
    }, [docRef, data, fetchSVGDoc, pathId]);

    const onChangeHandle1X = useCallback((value: string) => {
        const x = parseInt(value);
        if (Number.isNaN(x)) return;
        const y = structuredClone(getPathCommandHandle1Y(data));
        const handle = { x, y };
        docRef.current.edit_path_point_handle1(pathId, data.id, handle);
        fetchSVGDoc();
    }, [docRef, data, fetchSVGDoc, pathId])

    const onChangeHandle1Y = useCallback((value: string) => {
        const y = parseInt(value);
        if (Number.isNaN(y)) return;
        const x = structuredClone(getPathCommandHandle1X(data));
        const handle = { x, y };
        docRef.current.edit_path_point_handle1(pathId, data.id, handle)
        fetchSVGDoc();
    }, [docRef, data, fetchSVGDoc, pathId])

    const onChangeHandle2X = useCallback((value: string) => {
        const x = parseInt(value);
        if (Number.isNaN(x)) return;
        const y = structuredClone(getPathCommandHandle2Y(data));
        const handle = { x, y };
        docRef.current.edit_path_point_handle2(pathId, data.id, handle);
        fetchSVGDoc();
    }, [docRef, data, fetchSVGDoc, pathId])

    const onChangeHandle2Y = useCallback((value: string) => {
        const y = parseInt(value);
        if (Number.isNaN(y)) return;
        const x = structuredClone(getPathCommandHandle2X(data));
        const handle = { x, y };
        docRef.current.edit_path_point_handle2(pathId, data.id, handle);
        fetchSVGDoc();
    }, [docRef, data, fetchSVGDoc, pathId])

    return (
        <>
            <select onChange={e => onChangeSelection(e.target.value)} value={props.data.type}>
                <option value="START">Start</option>
                <option value="LINE">Line</option>
                <option value="CLOSE">Close</option>
                <option value="BEZIER">Bezier</option>
                <option value="BEZIER_REFLECT">Bezier Reflect</option>
                <option value="BEZIER_QUAD">Bezier Quad</option>
                <option value="BEZIER_QUAD_REFLECT">Bezier Quad Reflect</option>
            </select>
            <div><input
                type="number"
                value={getPathCommandPosX(data)}
                onChange={e => onChangePosX(e.target.value)}
            /></div>
            <div><input
                type="number" 
                value={getPathCommandPosY(data)}
                onChange={e => onChangePosY(e.target.value)}
            /></div>
            <div><input
                type="number" 
                value={getPathCommandHandle1X(data)}
                onChange={e => onChangeHandle1X(e.target.value)}
            /></div>
            <div><input 
                type="number" 
                value={getPathCommandHandle1Y(data)}
                onChange={e => onChangeHandle1Y(e.target.value)}
            /></div>
            <div><input
                type="number" 
                value={getPathCommandHandle2X(data)}
                onChange={e => onChangeHandle2X(e.target.value)}
            /></div>
            <div><input 
                type="number" 
                value={getPathCommandHandle2Y(data)}
                onChange={e => onChangeHandle2Y(e.target.value)}
            /></div>
        </>
    );
}


type ConfigurationPathProps = {
    data: SelectedSVG;
    docRef: MutableRefObject<SVGDoc>;
    fetchSVGDoc: () => void,
}

const ConfigurationPath: FC<ConfigurationPathProps> = (props) => {
    const { docRef, fetchSVGDoc, data: { id } } = props;
    const path = docRef.current.get_path(id);
    const onClickAddPathCommand = useCallback(() => {
        docRef.current.add_point_to_path(id, SVGPathCommandType.START, { x: 0, y: 0 });
        fetchSVGDoc();
    }, [docRef, fetchSVGDoc, id]);
    if (path === undefined) return (<></>);
    return (
        <>
            <div>Path</div>
            <NColumnGrid nColumns={7} style={{width: "20%", gap: `${px(3)} ${px(20)}`}}>
                <strong>Command</strong>
                <strong>Position x</strong>
                <strong>Position y</strong>
                <strong>Handle 1 x</strong>
                <strong>Handle 1 y</strong>
                <strong>Handle 2 x</strong>
                <strong>Handle 2 y</strong>
                {
                    path.points.map(p => <ConfigurationPathCommandRow 
                        data={p} 
                        pathId={id} 
                        key={p.id} 
                        docRef={docRef}
                        fetchSVGDoc={fetchSVGDoc}
                    />)
                }
            </NColumnGrid>
            <button
                onClick={onClickAddPathCommand}
            >Add Path Command</button>
        </>
    );
}

type ConfigurationProps = {
    selectedObject: SelectedObject;
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: (id?: string) => void,
};

export const Configuration: FC<ConfigurationProps> = (props) => {
    return (
        <div>
            {configurationMapper(props.selectedObject, props.docRef, props.fetchSVGDoc)}
        </div>
    )
}

// utility functions
const configurationMapper = (
    selectedObject: SelectedObject,
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: (id?: string) => void,
) => {
    if (selectedObject === "root") {
        return (
            <ConfigurationRoot
                docRef={docRef}
                fetchSVGDoc={fetchSVGDoc}
            />
        )
    }
    switch (selectedObject.type) {
        case "CIRCLE":
            return <ConfigurationCircle
                fetchSVGDoc={fetchSVGDoc}
                docRef={docRef}
                data={selectedObject}
            />
        case "RECTANGLE":
            return <ConfigurationRectangle
                fetchSVGDoc={fetchSVGDoc}
                docRef={docRef}
                data={selectedObject}
            />
        case "GROUP":
            return <ConfigurationGroup
                docRef={docRef}
                fetchSVGDoc={fetchSVGDoc}
                data={selectedObject}
            />
        case "PATH":
            return <ConfigurationPath
                docRef={docRef}
                fetchSVGDoc={fetchSVGDoc}
                data={selectedObject}
            />
    }
}