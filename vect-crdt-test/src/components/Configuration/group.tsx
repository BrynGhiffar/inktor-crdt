import { FC, MutableRefObject, useCallback } from "react";
import { SelectedSVG } from "../../types";
import { SVGDoc } from "@brynghiffar/vect-crdt-rs";
import { ConfigurationContainer } from "./ConfigurationContainer";
import { ConfigurationTitle } from "./ConfigurationTitle";
import { ConfigurationButton } from "./ConfigurationButton";
import { TwoColumnGrid } from "../../utility/components";
import { ColorButton, TrashButton } from "./Button";
import { useShowHideColorPicker } from "./hook";
import { RgbaColor, RgbaColorPicker } from "react-colorful";
import { px } from "../../utility/methods";

type ConfigurationGroupProps = {
    data: SelectedSVG,
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: () => void,
}

export const ConfigurationGroup: FC<ConfigurationGroupProps> = (props) => {
    const group = props.docRef.current.get_group(props.data.id);
    const fillPicker = useShowHideColorPicker();
    const strokePicker = useShowHideColorPicker();
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

    const onClickDelete = useCallback(() => {
        props.docRef.current.remove_object(props.data.id);
        props.fetchSVGDoc();
    }, [props]);

    const onClickToggleStroke = useCallback(() => {
        if (!group) return;
        if (group.stroke === null) {
            props.docRef.current.edit_group(props.data.id, { stroke: { type: "Some", item: [ 0, 0, 0, 1 ] }, stroke_width: { type: "Some", item: 1 } })
        } else {
            props.docRef.current.edit_group(props.data.id, { stroke: { type: "None" }, stroke_width: { type: "None" } });
        }
        props.fetchSVGDoc();
    }, [props, group]);

    const onClickToggleFill = useCallback(() => {
        if (!group) return;
        if (group.fill === null) {
            props.docRef.current.edit_group(props.data.id, { fill: { type: "Some", item: [ 255, 255, 255, 1 ] }});
        } else {
            props.docRef.current.edit_group(props.data.id, { fill: { type: "None" } });
        }
        props.fetchSVGDoc();
    }, [props, group]);

    const onChangeFill = useCallback((color: RgbaColor) => {
        if (!group) return;
        if (group.fill === null) return;
        props.docRef.current.edit_group(props.data.id, { fill: { type: "Some", item: [ color.r, color.g, color.b, color.a ]}})
        props.fetchSVGDoc();
    }, [props, group]);

    const onChangeStroke = useCallback((color: RgbaColor) => {
        if (!group) return;
        if (group.stroke === null) return;
        props.docRef.current.edit_group(props.data.id, { stroke: { type: "Some", item: [ color.r, color.g, color.b, color.a ]}});
        props.fetchSVGDoc();
    }, [props, group]);

    const onChangeStrokeWidth = useCallback((s: string) => {
        if (!group) return;
        if (group.stroke_width === null) return;
        const strokeWidth = parseInt(s);
        if (Number.isNaN(strokeWidth)) return;
        props.docRef.current.edit_group(props.data.id, { stroke_width: { type: "Some", item: strokeWidth }});
        props.fetchSVGDoc();
    }, [props, group]);

    const onClickToggleOpacity = useCallback(() => {
        if (!group) return;
        if (group.opacity === null) {
            props.docRef.current.edit_group(props.data.id, { opacity: { type: "Some", item: 1 }});
        } else {
            props.docRef.current.edit_group(props.data.id, { opacity: { type: "None" }})
        }
        props.fetchSVGDoc();
    }, [props, group]);

    const onChangeOpacity = useCallback((s: string) => {
        if (!group) return;
        if (group.opacity === null) return;
        const opacity = parseFloat(s);
        if (Number.isNaN(opacity)) return;
        props.docRef.current.edit_group(props.data.id, { opacity: { type: "Some", item: opacity }});
        props.fetchSVGDoc();
    }, [props, group]);

    if (!group) return (<></>);
    return (
        <ConfigurationContainer
            className="flex flex-col gap-2"
            onClick={() => {
                fillPicker.hideColorPicker();
                strokePicker.hideColorPicker();
            }}
        >
            <ConfigurationTitle 
                title="Group"
                onClickDeleteButton={onClickDelete}
            />
            <TwoColumnGrid style={{ gap: `${px(5)} ${px(20)}`}}>
                <div className="flex gap-2">
                    <TrashButton 
                        add={group.opacity === null}
                        onClick={onClickToggleOpacity}
                    />
                    <label className="text-white">opacity</label>
                </div>
                <input min={0} max={1} step={0.01}
                    value={group.opacity ?? 0} 
                    type="range"
                    disabled={group.opacity === null}
                    onChange={e => onChangeOpacity(e.target.value)}
                />
                <div className="flex gap-2">
                    <TrashButton 
                        add={group.stroke_width === null}
                        onClick={onClickToggleStroke}
                    />
                    <label className="text-white">stroke width</label>
                </div>
                <input min={0} max={100} 
                    value={group.stroke_width ?? 0} 
                    type="range"
                    disabled={group.stroke_width === null}
                    onChange={e => onChangeStrokeWidth(e.target.value)}
                />
                <div className="flex gap-2">
                    <TrashButton
                        add={group.fill === null}
                        onClick={onClickToggleFill}
                    />
                    <label className="text-white">fill</label>
                </div>
                <div className="flex gap-2">
                    <TrashButton 
                        add={group.stroke === null}
                        onClick={onClickToggleStroke}
                    />
                    <label className="text-white">stroke</label>
                </div>
                {group.fill !== null ? (
                    <ColorButton
                        pickerColor={group.fill}
                        onClick={() => {
                            fillPicker.toggleColorPicker();
                            strokePicker.hideColorPicker();
                        }}
                    >
                        {fillPicker.show && (
                            <RgbaColorPicker
                                style={{ position: "absolute", zIndex: 5 }} 
                                onChange={onChangeFill}
                                color={{ r: group.fill[0], g: group.fill[1], b: group.fill[2], a: group.fill[3] }}
                            />
                        )}
                    </ColorButton>
                ) : (<div/>)}
                {group.stroke !== null ? (
                    <ColorButton
                        pickerColor={group.stroke}
                        onClick={() => {
                            fillPicker.hideColorPicker();
                            strokePicker.toggleColorPicker();
                        }}
                    >
                        {strokePicker.show && (
                            <RgbaColorPicker
                                style={{ position: "absolute", zIndex: 5 }} 
                                onChange={onChangeStroke}
                                color={{ r: group.stroke[0], g: group.stroke[1], b: group.stroke[2], a: group.stroke[3] }}
                            />
                        )}
                    </ColorButton>
                ) : (<div/>)}
            </TwoColumnGrid>
            <div className="grid grid-cols-4 gap-2">
                <ConfigurationButton
                    onClick={onClickAddCircle}
                >
                    Add Circle
                </ConfigurationButton>
                <ConfigurationButton
                    onClick={onClickAddRectangle}
                >
                    Add Rectangle
                </ConfigurationButton>
                <ConfigurationButton
                    onClick={onClickAddGroup}
                >
                    Add Group
                </ConfigurationButton>
                <ConfigurationButton
                    onClick={onClickAddPath}
                >Add Path</ConfigurationButton>
            </div>
        </ConfigurationContainer>
    );
}
