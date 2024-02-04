import { useCallback } from 'react';
import './App.css';
import { Editor } from './components/Editor';
import { useSVGDoc } from "./hooks";
import { NColumnGrid } from './utility/components';


function App() {
  const doc1 = useSVGDoc("t0");
  const doc2 = useSVGDoc("t1");

  const onClickMerge = useCallback(() => {
    const h2 = doc2.SVGDocRef.current.broadcast();
    const h1 = doc1.SVGDocRef.current.broadcast();
    if (!h2) return;
    if (!h1) return;
    doc1.SVGDocRef.current.merge(h2);
    doc2.SVGDocRef.current.merge(h1);
    doc1.fetchSVGDoc();
    doc2.fetchSVGDoc();
  }, [doc2, doc1]);


  return (
    <div className="p-4 h-screen">
    <NColumnGrid nColumns={3} style={{ gridTemplateColumns: "1fr 200px 1fr"}}>
      <Editor
        docId={"t0"}
        fetchSVGDoc={doc1.fetchSVGDoc}
        droppableSVG={doc1.droppableSVG}
        doc={doc1.SVGDocRef}
      />
      <div style={{display: "flex", justifyContent: "center", alignItems: "center"}}>
        <button
          onClick={onClickMerge}
        >Merge</button>
      </div>
      <Editor
        docId={"t1"}
        fetchSVGDoc={doc2.fetchSVGDoc}
        droppableSVG={doc2.droppableSVG}
        doc={doc2.SVGDocRef}
      />
    </NColumnGrid>
    </div>
  )
}

export default App
