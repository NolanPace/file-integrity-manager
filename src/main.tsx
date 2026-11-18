import React,{useState} from 'react';import{createRoot}from'react-dom/client';import{invoke}from'@tauri-apps/api/core';import{open}from'@tauri-apps/plugin-dialog';import './style.css';
type Manifest={root:string,entries:Record<string,string>};
type Diff={added:string[],removed:string[],changed:string[]};
function App(){
 const[folder,setFolder]=useState(''),[manifest,setManifest]=useState<Manifest|null>(null),[diff,setDiff]=useState<Diff|null>(null),[busy,setBusy]=useState(false),[error,setError]=useState('');
 async function choose(){const selection=await open({directory:true,multiple:false});if(typeof selection==='string'){setFolder(selection);setManifest(null);setDiff(null)}}
 async function snapshot(){setBusy(true);setError('');try{const result=await invoke<Manifest>('snapshot',{path:folder});setManifest(result);setDiff(null)}catch(e){setError(String(e))}finally{setBusy(false)}}
 async function verify(){if(!manifest)return;setBusy(true);setError('');try{setDiff(await invoke<Diff>('verify',{manifest}))}catch(e){setError(String(e))}finally{setBusy(false)}}
 return <main><header><span>RUST · TAURI · REACT</span><h1>File Integrity <em>Vault</em></h1><p>Trust the bytes. Track every change with SHA-256.</p></header>
 <section className="toolbar"><button onClick={choose}>Choose folder</button><div className="path">{folder||'No folder selected'}</div><button disabled={!folder||busy} onClick={snapshot}>Create baseline</button><button className="outline" disabled={!manifest||busy} onClick={verify}>Verify changes</button></section>
 {error&&<p className="error" role="alert">{error}</p>}
 <div className="stats"><article><span>Files in baseline</span><strong>{manifest?Object.keys(manifest.entries).length:'—'}</strong></article><article><span>Added</span><strong>{diff?diff.added.length:'—'}</strong></article><article><span>Changed</span><strong>{diff?diff.changed.length:'—'}</strong></article><article><span>Removed</span><strong>{diff?diff.removed.length:'—'}</strong></article></div>
 <section><h2>Integrity report</h2>{!manifest?<p>Create a baseline of a directory to get started.</p>:!diff?<p>Baseline created. Re-run verification after changing files.</p>:<div className="reports">{(['added','changed','removed'] as const).map(kind=><div key={kind}><h3>{kind.toUpperCase()}</h3>{diff[kind].length?diff[kind].map(path=><div className="file" key={path}>{path}</div>):<p>None</p>}</div>)}</div>}</section>
 <footer>Local-only hashing · Symlinks excluded · No network uploads</footer></main>
}
createRoot(document.getElementById('root')!).render(<React.StrictMode><App/></React.StrictMode>);