import { useCallback, useRef, useState } from "react";
import { flattenSVGDocTree, getSVGData, saveSVGData } from "../utility/methods";
// import { SVGDoc } from "@brynghiffar/vect-crdt-rs";
import { SVGDoc as SVGDocRs } from "@brynghiffar/vect-crdt-rs";
import { SVGDoc as SVGDocJs } from "@brynghiffar/vect-crdt-js";
import { DroppableSVG } from "../types";


const createSVGDoc = (docId: string, useJs: boolean = false) => {
  if (!useJs) {
    return SVGDocRs.new(docId)
  } else {
    return  SVGDocJs.new(docId);
  }
};

const docs = new Map<string, SVGDocRs>();

const getSVGDoc = (docId: string, useJs: boolean = false) => {
  const doc = docs.get(docId);
  if (!doc) {
    const doc = createSVGDoc(docId, useJs);
    docs.set(docId, doc);
    return doc
  }
  return doc
}

export const useSVGDoc = (docId: string) => {
  const SVGDocRef = useRef(getSVGDoc(docId));
  const [droppableSVG, setDroppableSVG] = useState<DroppableSVG[]>([]);
  const fetchSVGDoc = useCallback((initial?: boolean) => {
    const doc = SVGDocRef.current;
    if (initial) {
      const data = getSVGData(docId);
      if (data) doc.load(data);
    } else {
      const data = doc.save();
      if (data) saveSVGData(docId, data);
      getSVGData(docId);
    }
    const flattened = flattenSVGDocTree(doc.children());
    setDroppableSVG(flattened);
  }, [SVGDocRef, docId]);

  return { droppableSVG, fetchSVGDoc, SVGDocRef };
};