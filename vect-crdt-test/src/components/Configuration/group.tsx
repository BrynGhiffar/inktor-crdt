import { FC, MutableRefObject, useCallback } from "react";
import { SelectedSVG } from "../../types";
import { SVGDoc } from "@brynghiffar/vect-crdt-rs";

type ConfigurationGroupProps = {
    data: SelectedSVG,
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: () => void,
}

export const ConfigurationGroup: FC<ConfigurationGroupProps> = (props) => {

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
            <button
                onClick={onClickDelete}
            >Delete</button>
        </>
    );
}
