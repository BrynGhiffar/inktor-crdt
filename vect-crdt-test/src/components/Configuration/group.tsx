import { FC, MutableRefObject, useCallback } from "react";
import { SelectedSVG } from "../../types";
import { SVGDoc } from "@brynghiffar/vect-crdt-rs";
import { ConfigurationContainer } from "./ConfigurationContainer";
import { ConfigurationTitle } from "./ConfigurationTitle";
import { ConfigurationButton } from "./ConfigurationButton";
import { TwoColumnGrid } from "../../utility/components";
import { ColorButton } from "./ColorButton";

type ConfigurationGroupProps = {
    data: SelectedSVG,
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: () => void,
}

export const ConfigurationGroup: FC<ConfigurationGroupProps> = (props) => {
    const group = props.docRef.current.get_group(props.data.id);
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
            props.docRef.current.edit_group(props.data.id, { stroke: [ 0, 0, 0, 1 ], stroke_width: 1 })
        } else {
            props.docRef.current.edit_group(props.data.id, { stroke: null, stroke_width: null });
        }
        props.fetchSVGDoc();
    }, [props, group]);

    const onClickToggleFill = useCallback(() => {
        if (!group) return;
        if (group.fill === null) {
            props.docRef.current.edit_group(props.data.id, { fill: [ 255, 255, 255, 0 ]});
        } else {
            props.docRef.current.edit_group(props.data.id, { fill: null });
        }
        props.fetchSVGDoc();
    }, [props, group]);

    if (!group) return (<></>);
    return (
        <ConfigurationContainer
            className="flex flex-col gap-2"
        >
            <ConfigurationTitle 
                title="Group"
                onClickDeleteButton={onClickDelete}
            />
            <TwoColumnGrid>
                <div className="flex gap-2">
                    <button
                        className="flex justify-center items-center rounded-md bg-red-400 aspect-square"
                        onClick={onClickToggleFill}
                    >
                        <img src="/trash.svg" alt="T" className="h-6 w-6"/>    
                    </button>
                    <label className="text-white">fill</label>
                </div>
                <label className="text-white">stroke</label>
                {group.fill !== null ? (
                    <ColorButton
                        pickerColor={group.fill}
                    >
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
