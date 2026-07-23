use anyhow::{Context,  Result};
use flate2::{ Compression, read::ZlibDecoder, write::{ ZlibEncoder}};
use clap::{Parser, Subcommand};
use std::{fmt::format, fs::{self, File, create_dir_all}, io::{self, BufRead, BufReader, Read, Stdout, Write}, path::{Path, PathBuf}, ptr::hash, vec};
use std::ffi::CStr;
use sha1::{ Digest, Sha1, digest};

use crate::Kind::blob;
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    Blob,
    Tree,
    Commit
}
pub(crate) struct  Object <R> {
    pub kind:Kind,
   pub  expected_size:u64,
   pub  reader:R

}

impl Object<()>{
  pub(crate)   fn read(hash:&str)->anyhow::Result<Object<impl BufRead>>
    {
//open the hash file 

let f = std::fs::File::open(format!(".git/objects/{}/{}",&hash[..2],&hash[2..]))
.context("opening the git file ")?;

// decompressed it 
let z = ZlibDecoder::new(f);
let mut z= BufReader::new(z);

// read it before null oprator 0 
let mut buf: Vec<u8> = Vec::new();
z.read_until(0, &mut buf).context("reading the buf")?;




// check he is a c type string or not with cstr
let mut header = CStr::from_bytes_until_nul(&buf).context("cstr convertion")?;

// convert cstr to str
let header =header.to_str().context("header converting in string")?;

let Some((kind,size)) = header.split_once(' ') else {
            anyhow::bail!(".git/objects file header did not start with a known type: '{header}'")
        };

let kind = match kind {
    "blob"=>Kind::Blob,
    "tree"=>Kind::Tree,
    "commit"=>Kind::Commit,
    _=>anyhow::bail!("what even is {kind}")
    
};
let size = size.parse::<u64>().context("git/objects file header has invalid size: {size}")?;
let z = z.take(size);
Ok(Object{
    expected_size:size,
    reader:z,
    kind:kind
})


}}




// impl Object<()> {
//     pub(crate)fn read(hash:&str)->anyhow::Result<Object<impl BufRead>> {

        
//            let f = std::fs::File::open(format!(".gites/objects/{}/{}", &hash[..2], &hash[2..]))
//             .context("open in .git/objects")?;
//         let z = ZlibDecoder::new(f);












//         let mut z = BufReader::new(z);
//         let mut buf = Vec::new();
//         z.read_until(0, &mut buf)
//             .context("read header from .git/objects")?;


//         let header = CStr::from_bytes_with_nul(&buf)
//             .expect("know there is exactly one nul, and it's at the end");


//         let header = header
//             .to_str()
//             .context(".git/objects file header isn't valid UTF-8")?;
//         let Some((kind, size)) = header.split_once(' ') else {
//             anyhow::bail!(".git/objects file header did not start with a known type: '{header}'");
//         };
//         let kind = match kind {
//             "blob" => Kind::Blob,
//             "tree" => Kind::Tree,
//             "commit" => Kind::Commit,
//             _ => anyhow::bail!("what even is a '{kind}'"),
//         };
//         let size = size
//             .parse::<u64>()
//             .context(".git/objects file header has invalid size: {size}")?;
//         // NOTE: this won't error if the decompressed file is too long, but will at least not
//         // spam stdout and be vulnerable to a zipbomb.
//         let z = z.take(size);
//         Ok(Object {
//             kind,
//             expected_size: size,
//             reader: z,
//         })
    
//     }



// }



struct Hashwrite<W>{
    writer:W,
    hasher:Sha1


}

impl <W>Write for  Hashwrite<W> where W:Write{
   fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
let n = self.writer.write(buf)?;
self.hasher.update(&buf[..n]);
Ok(n)
  
       
   }
   fn flush(&mut self) -> io::Result<()> {
      self.writer.flush()
   }

    
}
