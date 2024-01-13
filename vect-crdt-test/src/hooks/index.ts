import { useCallback, useRef, useState } from "react";
import { flattenSVGDocTree, getSVGData, saveSVGData } from "../utility/methods";
import { SVGDoc } from "@brynghiffar/vect-crdt-rs";
import { DroppableSVG } from "../types";

export const useSVGDoc = (docId: string) => {
  const SVGDocRef = useRef(SVGDoc.new());
  const [droppableSVG, setDroppableSVG] = useState<DroppableSVG[]>([]);
  const fetchSVGDoc = useCallback((initial?: boolean) => {
    const doc = SVGDocRef.current;
    if (initial) {
      const data = getSVGData(docId);
      if (data) doc.merge(data);
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