import { useCallback, FC, MutableRefObject } from 'react';
import { SVGDoc } from "@brynghiffar/vect-crdt-rs";
import { SelectedSVG } from '../../types';
import { RgbaColor, RgbaColorPicker } from 'react-colorful';
import { TwoColumnGrid } from '../../utility/components';
import { px } from '../../utility/methods';
import { ConfigurationContainer } from './ConfigurationContainer';
import { ConfigurationTitle } from './ConfigurationTitle';
import { useShowHideColorPicker } from './hook';
import { ColorButton } from './ColorButton';

type ConfigurationCircleProps = {
    data: SelectedSVG,
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: (initial?: boolean) => void,
}


export const ConfigurationCircle: FC<ConfigurationCircleProps> = (props) => {
    const docRef = props.docRef;
    const fetchSVGDoc = props.fetchSVGDoc;
    const circle = docRef.current.get_circle(props.data.id);
    const fillPicker = useShowHideColorPicker();
    const strokePicker = useShowHideColorPicker();

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

    const onClickDelete = useCallback(() => {
        if (circle === undefined) return;
        docRef.current.remove_object(circle.id);
        fetchSVGDoc();
    }, [docRef, circle, fetchSVGDoc]);

    if (circle === undefined) { return (<></>)}
    const [ fillRed, fillGreen, fillBlue, fillOpacity ] = circle.fill;
    const [ strokeRed, strokeGreen, strokeBlue, strokeOpacity ] = circle.stroke;

    return (
        <ConfigurationContainer onClick={() => {
            fillPicker.hideColorPicker();
            strokePicker.hideColorPicker();
        }}>
            <ConfigurationTitle title="Circle" onClickDeleteButton={onClickDelete}/>
            <TwoColumnGrid style={{ gap: `${px(3)} ${px(20)}`}}>
                <label htmlFor="" className="text-white">cx</label>
                <input min={-100} max={100} value={circle.pos.x} type="range" onChange={e => onChangeCx(e.target.value)}/>
                <label htmlFor="" className="text-white">cy</label>
                <input min={-100} max={100} value={circle.pos.y} type="range" onChange={e => onChangeCy(e.target.value)}/>
                <label htmlFor="" className="text-white">radius</label>
                <input min={0} max={150} value={circle.radius} type="range" onChange={e => onChangeRadius(e.target.value)}/>
                <label htmlFor="" className="text-white">stroke width</label>
                <input min={0} max={150} value={circle.stroke_width} type="range" onChange={e => onChangeStrokeWidth(e.target.value)}/>
                <label htmlFor="" className="text-white">opacity</label>
                <input type="range" min={0} max={1} step={0.01} onChange={ e => onChangeOpacity(e.target.value)} value={circle.opacity}/>
                <label className="text-white">fill</label>
                <label className="text-white">stroke</label>
                <ColorButton
                    pickerColor={circle.fill}
                    onClick={() => {
                        fillPicker.toggleColorPicker();
                        strokePicker.hideColorPicker();
                    }}
                >
                    {fillPicker.show && (
                        <RgbaColorPicker
                            style={{ position: "absolute", zIndex: 5 }}
                            onChange={onChangeFill}
                            color={{ r: fillRed, g: fillGreen, b: fillBlue, a: fillOpacity }}
                        />
                    )}
                </ColorButton>
                <ColorButton
                    pickerColor={circle.stroke}
                    onClick={() => {
                        strokePicker.toggleColorPicker();
                        fillPicker.hideColorPicker();                        
                    }}
                >
                    {strokePicker.show && (
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