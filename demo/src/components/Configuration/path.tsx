import { MutableRefObject, FC, useCallback } from "react";
import { SVGDoc, SVGPathCommand, SVGPathCommandType } from "@brynghiffar/vect-crdt-rs";
import { z } from "zod";
import { SelectedSVG } from "../../types";
import { NColumnGrid, TwoColumnGrid } from "../../utility/components";
import { px } from "../../utility/methods";
import { ConfigurationContainer } from "./ConfigurationContainer";
import { ConfigurationTitle } from "./ConfigurationTitle";
import { useShowHideColorPicker } from "./hook";
import { ColorButton } from "./Button";
import { RgbaColor, RgbaColorPicker } from "react-colorful";
import { ConfigurationButton } from "./ConfigurationButton";

type ConfigurationPathProps = {
    data: SelectedSVG;
    docRef: MutableRefObject<SVGDoc>;
    fetchSVGDoc: () => void,
}

export const ConfigurationPath: FC<ConfigurationPathProps> = (props) => {
    const { docRef, fetchSVGDoc, data: { id } } = props;
    const path = docRef.current.get_path(id);
    const onClickAddPathCommand = useCallback(() => {
        docRef.current.add_point_to_path(id, SVGPathCommandType.START, { x: 0, y: 0 });
        fetchSVGDoc();
    }, [docRef, fetchSVGDoc, id]);
    const onChangeFill = useCallback((color: RgbaColor) => {
        if (path === undefined) return;
        docRef.current.edit_path(path.id, { fill: [ color.r, color.g, color.b, color.a ]})
        fetchSVGDoc();
    }, [docRef, fetchSVGDoc, path])
    const onChangeStroke = useCallback((color: RgbaColor) => {
        if (path === undefined) return;
        docRef.current.edit_path(path.id, { stroke: [ color.r, color.g, color.b, color.a ]});
        fetchSVGDoc();
    }, [docRef, fetchSVGDoc, path]);
    const onChangeStrokeWidth = useCallback((s: string) => {
        if (path === undefined) return;
        const strokeWidth = parseInt(s);
        if (Number.isNaN(strokeWidth)) return;
        docRef.current.edit_path(path.id, { stroke_width: strokeWidth });
        fetchSVGDoc();
    }, [docRef, fetchSVGDoc, path]);
    const onChangeOpacity = useCallback((s: string) => {
        if (path === undefined) return;
        const opacity = parseFloat(s);
        if (Number.isNaN(opacity)) return;
        docRef.current.edit_path(path.id, { opacity });
        fetchSVGDoc();
    }, [docRef, fetchSVGDoc, path]);
    const onClickDelete = useCallback(() => {
        if (path === undefined) return;
        docRef.current.remove_object(path.id);
        fetchSVGDoc();
    }, [docRef, fetchSVGDoc, path])
    const fill = useShowHideColorPicker();
    const stroke = useShowHideColorPicker();
    if (path === undefined) return (<></>);
    const [ fillRed, fillGreen, fillBlue, fillOpacity ] = path.fill;
    const [ strokeRed, strokeGreen, strokeBlue, strokeOpacity ] = path.stroke;
    return (
        <ConfigurationContainer
            className="flex flex-col gap-2"
            onClick={() => {
                fill.hideColorPicker();
                stroke.hideColorPicker();
            }}
        >
            <ConfigurationTitle
                title="Path"
                onClickDeleteButton={onClickDelete}
            />
            <TwoColumnGrid>
                <label className="text-white">stroke width</label>
                <input min={0} max={100} type="range" value={path.stroke_width} onChange={e => onChangeStrokeWidth(e.target.value)}/>
                <label className="text-white">opacity</label>
                <input min={0} max={1} step={0.01} type="range" value={path.opacity} onChange={e => onChangeOpacity(e.target.value)}/>
                <label className="text-white">fill</label>
                <label className="text-white">stroke</label>
                <ColorButton
                    pickerColor={path.fill}
                    onClick={() => {
                        fill.toggleColorPicker();
                        stroke.hideColorPicker();
                    }}
                >
                    {fill.show && (
                        <RgbaColorPicker
                            style={{ position: "absolute", zIndex: 5 }}
                            onChange={onChangeFill}
                            color={{ r: fillRed, g: fillGreen, b: fillBlue, a: fillOpacity }}
                        />
                    )}
                </ColorButton>
                <ColorButton
                    pickerColor={path.stroke}
                    onClick={() => {
                        stroke.showColorPicker();
                        fill.hideColorPicker();
                    }}
                >
                    {stroke.show && (
                        <RgbaColorPicker
                            style={{ position: "absolute", zIndex: 5 }}
                            onChange={onChangeStroke}
                            color={{ r: strokeRed, g: strokeGreen, b: strokeBlue, a: strokeOpacity }}
                        />
                    )}
                </ColorButton>
            </TwoColumnGrid>
            <NColumnGrid nColumns={8} style={{width: "20%", gap: `${px(3)} ${px(20)}`}}>
                <strong className="text-white">Command</strong>
                <strong className="text-white min-w-16">X</strong>
                <strong className="text-white min-w-16">Y</strong>
                <strong className="text-white min-w-16">H1 X</strong>
                <strong className="text-white min-w-16">H1 Y</strong>
                <strong className="text-white min-w-16">H2 X</strong>
                <strong className="text-white min-w-16">H2 Y</strong>
                <div></div>
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
            <ConfigurationButton
                onClick={onClickAddPathCommand}
            >
                Add Path Command
            </ConfigurationButton>
        </ConfigurationContainer>
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
    if (command.type === "BEZIER") {
        return command.handle1.x;
    }
    return command.handle.x
}

const getPathCommandHandle1Y = (command: SVGPathCommand) => {
    if (command.type === "CLOSE") return 0;
    if (command.type === "LINE") return 0;
    if (command.type === "START") return 0;
    if (command.type === "BEZIER") {
        return command.handle1.y;
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
            case "BEZIER_QUAD":
                docRef.current.edit_path_point_type(pathId, data.id, SVGPathCommandType.BEZIER_QUAD);
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
    }, [docRef, data, fetchSVGDoc, pathId]);

    const onClickDelete = useCallback(() => {
        docRef.current.remove_path_point(pathId, data.id);
        fetchSVGDoc();
    }, [docRef, data, fetchSVGDoc, pathId]);

    return (
        <>
            <select onChange={e => onChangeSelection(e.target.value)} value={props.data.type} className="rounded-md p-1 bg-white">
                <option value="START">Start</option>
                <option value="LINE">Line</option>
                <option value="CLOSE">Close</option>
                <option value="BEZIER">Bezier</option>
                <option value="BEZIER_QUAD">Bezier Quad</option>
            </select>
            <div><input
                type="number"
                className="w-14 p-1 rounded-md"
                value={getPathCommandPosX(data)}
                onChange={e => onChangePosX(e.target.value)}
            /></div>
            <div><input
                type="number" 
                className="w-14 p-1 rounded-md"
                value={getPathCommandPosY(data)}
                onChange={e => onChangePosY(e.target.value)}
            /></div>
            <div><input
                type="number" 
                className="w-14 p-1 rounded-md"
                value={getPathCommandHandle1X(data)}
                onChange={e => onChangeHandle1X(e.target.value)}
            /></div>
            <div><input 
                type="number" 
                className="w-14 p-1 rounded-md"
                value={getPathCommandHandle1Y(data)}
                onChange={e => onChangeHandle1Y(e.target.value)}
            /></div>
            <div><input
                type="number" 
                className="w-14 p-1 rounded-md"
                value={getPathCommandHandle2X(data)}
                onChange={e => onChangeHandle2X(e.target.value)}
            /></div>
            <div><input 
                type="number" 
                className="w-14 p-1 rounded-md"
                value={getPathCommandHandle2Y(data)}
                onChange={e => onChangeHandle2Y(e.target.value)}
            /></div>
            <div className="h-6 w-6">
                <button
                    className="flex justify-center items-center rounded-md bg-red-400 aspect-square hover:bg-red-500 transition ease-in-out duration-100" 
                    onClick={onClickDelete}
                >
                    <img src="/trash.svg" className="h-6 w-6"/>
                </button>
            </div>
        </>
    );
}