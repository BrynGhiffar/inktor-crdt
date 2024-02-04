import { MutableRefObject, FC, useCallback } from "react";
import { SelectedSVG } from "../../types";
import { SVGDoc } from "@brynghiffar/vect-crdt-rs";
import { TwoColumnGrid } from "../../utility/components";
import { RgbaColor, RgbaColorPicker } from "react-colorful";
import { px } from "../../utility/methods";
import { useShowHideColorPicker } from "./hook";
import { ConfigurationTitle } from "./ConfigurationTitle";
import { ConfigurationContainer } from "./ConfigurationContainer";
import { ColorButton } from "./ColorButton";

type ConfigurationRectangleProps = {
    data: SelectedSVG
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: () => void,
}

export const ConfigurationRectangle: FC<ConfigurationRectangleProps> = (props) => {
    const { fetchSVGDoc, docRef, data: { id }} = props;
    const rect = docRef.current.get_rectangle(id);
    const fill = useShowHideColorPicker();
    const stroke = useShowHideColorPicker();
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

    const onChangeStrokeWidth = useCallback((s: string) => {
        const strokeWidth = parseInt(s)
        if (Number.isNaN(strokeWidth)) return;
        if (rect === undefined) return;
        docRef.current.edit_rectangle(rect.id, { stroke_width: strokeWidth });
        fetchSVGDoc();
    }, [fetchSVGDoc, docRef, rect])

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
        <ConfigurationContainer onClick={() => {
            fill.hideColorPicker();
            stroke.hideColorPicker();
        }}>
            <ConfigurationTitle title="Rectangle"/>
            <TwoColumnGrid style={{ gap: `${px(0)} ${px(20)}` }}>
                <label htmlFor="" className="text-white">cx</label>
                <input min={-100} max={100} value={rect.pos.x} type="range" onChange={e => onChangeCx(e.target.value)}/>
                <label htmlFor="" className="text-white">cy</label>
                <input min={-100} max={100} value={rect.pos.y} type="range" onChange={e => onChangeCy(e.target.value)}/>
                <label htmlFor="" className="text-white">stroke width</label>
                <input min={0} max={100} value={rect.stroke_width} type="range" onChange={e => onChangeStrokeWidth(e.target.value)}/>
                <label htmlFor="" className="text-white">width</label>
                <input min={0} max={500} value={rect.width} type="range" onChange={e => onChangeWidth(e.target.value)}/>
                <label htmlFor="" className="text-white">height</label>
                <input min={0} max={500} value={rect.height} type="range" onChange={e => onChangeHeight(e.target.value)}/>
                <label htmlFor="" className="text-white">opacity</label>
                <input type="range" min={0} max={1} step={0.01} onChange={ e => onChangeOpacity(e.target.value)} value={rect.opacity}/>
                <label className="text-white">fill</label>
                <label className="text-white">stroke</label>
                <ColorButton
                    pickerColor={rect.fill}
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
                    pickerColor={rect.stroke} 
                    onClick={() => {
                        stroke.toggleColorPicker(); 
                        fill.hideColorPicker();
                    }}
                >
                    {stroke.show && (
                        <RgbaColorPicker
                            style={{ position: "absolute", zIndex: 5 }}
                            onChange={onChangeStroke}
                            color={{r: strokeRed, g: strokeGreen, b: strokeBlue, a: strokeOpacity }}
                        />
                    )}
                </ColorButton>
            </TwoColumnGrid>
        </ConfigurationContainer>
    );
}
