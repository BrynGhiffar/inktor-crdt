import { FC, MutableRefObject, useCallback } from "react";
import { SVGDoc } from "@brynghiffar/vect-crdt-rs";
import { SelectedObject } from "../../types";
import { ConfigurationCircle } from "./circle";
import { ConfigurationRectangle } from "./rectangle";
import { ConfigurationPath } from "./path";
import { ConfigurationGroup } from "./group";
import { ConfigurationContainer } from "./ConfigurationContainer";
import { ConfigurationTitle } from "./ConfigurationTitle";
import { ConfigurationButton } from "./ConfigurationButton";

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
        <ConfigurationContainer className="flex gap-2 flex-col">
            <ConfigurationTitle title="Document" hideDeleteButton/>
            <div className="grid grid-cols-4 gap-2">
                <ConfigurationButton
                    onClick={onClickAddCircle}
                >Add Circle</ConfigurationButton>
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
    )
}

type ConfigurationProps = {
    selectedObject: SelectedObject;
    docRef: MutableRefObject<SVGDoc>,
    fetchSVGDoc: (initial?: boolean) => void,
};

export const Configuration: FC<ConfigurationProps> = (props) => {
    return (
        <>
            {configurationMapper(props.selectedObject, props.docRef, props.fetchSVGDoc)}
        </>
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