import js from "@brynghiffar/vect-crdt-js";
import rs, { SVGPathCommandType } from "@brynghiffar/vect-crdt-rs";

export type * from "@brynghiffar/vect-crdt-rs";
export { SVGPathCommandType } from  "@brynghiffar/vect-crdt-rs";

export const createSVGDoc = (docId: string, useJs: boolean) => {
    if (useJs) {
        return js.SVGDoc.new(docId)
    } else {
        return rs.SVGDoc.new(docId);
    }
};