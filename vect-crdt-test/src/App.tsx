import { useEffect, useRef } from 'react';
import { SVGDoc } from "vect-crdt-rs";
import './App.css';

function App() {
  const SVGDocRef = useRef(SVGDoc.new());
  useEffect(() => {
    console.log(SVGDocRef.current.repr());
  }, []);

  return (
    <>
      <pre>{SVGDocRef.current.repr()}</pre>
    </>
  )
}

export default App
