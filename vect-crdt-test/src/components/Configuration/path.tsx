import { MutableRefObject, FC, useCallback } from "react";
import { SVGDoc, SVGPathCommand, SVGPathCommandType } from "@brynghiffar/vect-crdt-rs";
import { z } from "zod";
import { SelectedSVG } from "../../types";
import { NColumnGrid } from "../../utility/components";
import { px } from "../../utility/methods";

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
    if (path === undefined) return (<></>);
    return (
        <>
            <div>Path</div>
            <NColumnGrid nColumns={8} style={{width: "20%", gap: `${px(3)} ${px(20)}`}}>
                <strong>Command</strong>
                <strong>Position x</strong>
                <strong>Position y</strong>
                <strong>Handle 1 x</strong>
                <strong>Handle 1 y</strong>
                <strong>Handle 2 x</strong>
                <strong>Handle 2 y</strong>
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
            <button
                onClick={onClickAddPathCommand}
            >Add Path Command</button>
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
    }, [docRef, data, fetchSVGDoc, pathId]);

    const onClickDelete = useCallback(() => {
        docRef.current.remove_path_point(pathId, data.id);
        fetchSVGDoc();
    }, [docRef, data, fetchSVGDoc, pathId]);

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
            <div><button onClick={onClickDelete}>
                Delete
            </button></div>
        </>
    );
}