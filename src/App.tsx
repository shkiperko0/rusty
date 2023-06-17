import { DependencyList, useEffect, useMemo, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import styles from "./App.module.scss";

type Promised<T> = T |Promise<T>

async function parse_content(text: string): Promise<string[]> {
  await invoke("parse_codes", { text })
  return []
}

const Barcode = (props: { filename: string }) => {
  const { filename } = props
  const url = `http://localhost:8080/${filename}.pdf`

  console.log({barcode: filename})

  return <object title={filename} data={url} type="application/pdf" width="auto" height="auto">
    <p>{filename} not found</p>
  </object>
}

function useMemoAsync<T>(factory: () => Promise<T>, dep: DependencyList): (T | undefined) {
  const [ state, setState ] = useState<T | undefined>() 
  useEffect(() => { factory().then(setState) }, [...dep, factory])
  return state
}

const ArticleParser = (props: { text: string }) => {
  const barcodes = useMemoAsync(async () => parse_content(props.text), [props.text])

  if(barcodes === undefined){
    return <>Loading...</>
  }

  return <>
    { barcodes.map(filename => <Barcode key={filename} filename={filename} />) }
  </>
}

function App() {
  const [text, setText] = useState("");
  console.log(text)
  return (
    <div className="container">
      <h1>Скопируйте диапазон артикулов из файла и вставьте в поле</h1>

      <textarea
        className="row"
        id="greet-input"
        // rows={}
        onChange={({currentTarget: { value }}) => { setText(value) }}
        placeholder="Диаказон артикулов..."
      />

      <div className={styles.barcodes}>
        { 
        text.split("\n").filter(code => code.length).map(
            (filename, index) => <>
              <Barcode key={index} filename={filename} />
              <pre>{filename}</pre>
            </>
          ) 
        }
      </div>

      <ArticleParser text={text} />

    </div>
  );
}

export default App;

/*
      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
*/