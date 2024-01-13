import { FC, MutableRefObject, useCallback } from "react";
import { SVGDoc } from "@brynghiffar/vect-crdt-rs";
import { SelectedObject } from "../../types";
import { ConfigurationCircle } from "./circle";
import { ConfigurationRectangle } from "./rectangle";
import { ConfigurationPath } from "./path";
import { ConfigurationGroup } from "./group";

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

type ConfigurationProps = {
    selectedObject: SelectedObject;
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: (initial?: boolean) => void,
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
    fetchSVGDoc: (initial?: boolean) => void,
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