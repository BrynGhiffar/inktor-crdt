import { MutableRefObject, FC, useCallback } from "react";
import { SelectedSVG } from "../../types";
import { SVGDoc } from "@brynghiffar/vect-crdt-rs";
import { TwoColumnGrid } from "../../utility/components";
import { RgbaColor, RgbaColorPicker } from "react-colorful";
import { px } from "../../utility/methods";

type ConfigurationRectangleProps = {
    data: SelectedSVG
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: () => void,
}

export const ConfigurationRectangle: FC<ConfigurationRectangleProps> = (props) => {
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

    const onChangeFill = useCallback((color: RgbaColor) => {
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
