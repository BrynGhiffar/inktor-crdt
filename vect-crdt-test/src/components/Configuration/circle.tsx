import { useCallback, FC, MutableRefObject } from 'react';
import { SVGDoc } from "@brynghiffar/vect-crdt-rs";
import { SelectedSVG } from '../../types';
import { RgbaColor, RgbaColorPicker } from 'react-colorful';
import { TwoColumnGrid } from '../../utility/components';
import { px } from '../../utility/methods';

type ConfigurationCircleProps = {
    data: SelectedSVG,
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: (initial?: boolean) => void,
}


export const ConfigurationCircle: FC<ConfigurationCircleProps> = (props) => {
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