use anyhow::{Context};
use flate2::{ Compression ,write:: ZlibEncoder};
use std::{ fs::{self,}, io::{self, Write}, path::{Path, PathBuf},};
use sha1::{ Digest, Sha1};



pub(crate)fn invoke(file:&PathBuf,write: bool)->anyhow::Result<()>{

    let read = fs::read(&file);


     fn write_blob<W>(file: &Path, write: W) -> anyhow::Result<String>  where W:Write {

let stat = std::fs::metadata(&file)
    .with_context(|| format!("reading the file {}", file.display())).expect("fail the metadata reading a file ");
    
   let mut writer = ZlibEncoder::new(Vec::new(), Compression::default());
   

   // what he gave basically we putting read binarry then the zlib iin writer just create a compression on 
   //it then sha1 giving new sha of 20 bytes so we not have a billion bytes we only have 20 bytes 
   let mut writer = Hashwrite{
       writer: writer,
       hasher:Sha1::new()
    };
    
    write!(writer,"blob ").expect("got an issue in writing blob");
    write!(writer,"{}\0 " ,&stat.len() ).expect("got the issue in wrtiing blob");

    let mut file= std::fs::File::open(&file).with_context(||format!("df {}",&file.display()))?;
    
    std::io::copy(&mut file, &mut writer);
    let _ = writer.writer.finish();
    let hash = writer.hasher.finalize();

    
    Ok(hex::encode(hash))
    
    
}
let hash = if write {
    let tamp = "tempraory";
    
    let hash = write_blob(&file, fs::File::create(tamp).context("creatng tamp file")?)?;
    fs::create_dir_all(format!(".gites/objects/{}",&hash[..2])).context("creating files ")?;
    fs::rename(&tamp, format!(".gites/objects/{}/{}", &hash[..2], &hash[2..]))?;
    hash

    
} else {
    write_blob(&file, std::io::sink()).context("context")?
    
};
   println!("{hash}");
     
     
//      let mut blob:Vec<u8>= Vec::new();
//      //rough code 
//      write!(blob ,"blob {}\0",&read.len());
//      let object =blob.extend_from_slice(&read);
//      let shahash = Sha1::digest(&blob);
//      let  hex= hex::encode(shahash);
//      let path = format!(".gites/objects/{}/{}",&hex[..2],&hex[2..]);
//      let mut zlib_en = ZlibEncoder::new(Vec::new(), Compression::default());
//   zlib_en.write_all(&blob);
  
// let compressed = zlib_en.finish()?;
// create_dir_all(&path);
// fs::write(&path, compressed);





Ok(())

}

   



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
