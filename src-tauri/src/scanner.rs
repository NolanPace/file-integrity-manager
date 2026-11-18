use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path,PathBuf};
use sha2::{Digest,Sha256};
use serde::{Serialize,Deserialize};
use walkdir::WalkDir;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Manifest {pub root:String,pub entries:BTreeMap<String,String>}
#[derive(Debug,Serialize,Deserialize)]
pub struct Diff {pub added:Vec<String>,pub removed:Vec<String>,pub changed:Vec<String>}

fn digest(path:&Path)->Result<String,String> {
 let mut f=File::open(path).map_err(|e|e.to_string())?;
 let mut hasher=Sha256::new();let mut buf=[0u8;65536];
 loop{let len=f.read(&mut buf).map_err(|e|e.to_string())?;if len==0{break;}hasher.update(&buf[..len]);}
 Ok(hex::encode(hasher.finalize()))
}
pub fn snapshot(root:&str)->Result<Manifest,String>{
 let base=PathBuf::from(root);
 if !base.is_dir(){return Err("Choose an existing directory".into())}
 let mut entries=BTreeMap::new();
 for entry in WalkDir::new(&base).follow_links(false).into_iter().filter_map(Result::ok) {
  if !entry.file_type().is_file(){continue}
  let rel=entry.path().strip_prefix(&base).map_err(|e|e.to_string())?.to_string_lossy().replace('\\',"/");
  entries.insert(rel,digest(entry.path())?);
 }
 Ok(Manifest{root:root.to_string(),entries})
}
pub fn verify(previous:&Manifest)->Result<Diff,String>{
 let current=snapshot(&previous.root)?;
 let mut added=Vec::new();let mut removed=Vec::new();let mut changed=Vec::new();
 for (path,hash) in &current.entries {
  match previous.entries.get(path){None=>added.push(path.clone()),Some(old) if old!=hash=>changed.push(path.clone()),_=>()}
 }
 for path in previous.entries.keys(){if !current.entries.contains_key(path){removed.push(path.clone());}}
 Ok(Diff{added,removed,changed})
}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]fn detects_changes(){
  let folder=std::env::temp_dir().join(format!("integrity-test-{}",std::process::id()));
  let _=std::fs::remove_dir_all(&folder);std::fs::create_dir_all(&folder).unwrap();
  let f=folder.join("hello.txt");std::fs::write(&f,b"hello").unwrap();
  let first=snapshot(folder.to_str().unwrap()).unwrap();
  assert_eq!(first.entries.len(),1);
  std::fs::write(&f,b"world").unwrap();
  let diff=verify(&first).unwrap();assert_eq!(diff.changed.len(),1);
  std::fs::remove_dir_all(&folder).unwrap();
 }
}