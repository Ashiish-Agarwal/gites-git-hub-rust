use anyhow::{Context,  Result};
use flate2::{ Compression, read::ZlibDecoder  ,write:: ZlibEncoder};
use clap::{Parser, Subcommand, builder::Str};
use std::{fmt::format, fs::{self, File, create_dir_all}, io::{self, BufRead, BufReader, Read, Stdout, Write}, path::{Path, PathBuf}, ptr::hash, vec};
use std::ffi::CStr;
use sha1::{ Digest, Sha1};

#[derive(Parser)]
// #[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    
    Init ,
    Catfile {
        #[clap(short='p')]
        prettyprint:bool,
        objecthash:String
    },
    HashFile {
        #[clap(short='w')]
        write :bool,
        file:PathBuf


    }
}
enum Kind {
    blob
}

fn main()-> Result<()>{
    let cli = Cli::parse();
    
   
match cli.command {
   Commands::Init =>{

   fs::create_dir(".gites").unwrap();
   fs::create_dir(".gites/objects").unwrap();
   fs::create_dir(".gites/refs").unwrap();
   fs::write(".gites/HEAD", "ref:refs/heads/main\n").unwrap()
   


}
Commands::Catfile { prettyprint , objecthash}=>{
    anyhow::ensure!(prettyprint,"mode must be wihtout -p , and we dont support mode");
    let  f = std::fs::File::open(format!("./gites/objects/{}/{}", &objecthash[..2], &objecthash[2..])).context("open in .gites")?;

    let mut z = ZlibDecoder::new(f);
    let mut z = BufReader::new(z);
    let mut buff= Vec::new();
    z.read_until(0,&mut buff).expect("read header from gites");
    let header = CStr::from_bytes_until_nul(&buff).expect("know this is exactly null ");
    let header = header.to_str().expect("git object file is not valid");
    let Some((Kind, size))= header.split_once(" ")else {
        anyhow::bail!("git object file header is not known type: ' {header}'  ")
    };
    
   
    let kind = match Kind {
        "blob"=>Kind::blob,
        _=>anyhow::bail!("kwe do not yet know how to print a '{Kind}'")
};


    let size = size.parse::<usize>().context("get header file is invalid size ")?;

    
    buff.clear();
     buff.resize(size,0);
    z.read_exact(&mut buff[..]).context("read true content of .gites/object file")?;
    let n = z.read(&mut [0]).context("validate eof in .git/object file had {n} trailing bytes")?;
    let mut  z = Ratelimitor{
        reader:z,
        limit:size,
    };
   
    match kind {
       Kind::blob =>{
           let stdout = std::io::stdout();
           let mut stdout = stdout.lock();
           let n = std::io::copy(&mut z,&mut stdout).context("write .git/object file to stdout")?;
       }
       
   }
   
    let stdout = std::io::stdout();
    let mut Stdout = stdout.lock();



    


    
}
   
    
Commands::HashFile { write , file  }=>{
   


   
     let read = fs::read(&file)?;


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
    
    let hash = write_blob(&file, fs::File::create(tamp).context("creatng tamp file")? )?;
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






}
}
   
Ok(())

}

struct Ratelimitor <R>{

    reader:R,
    limit:usize


}
 
impl<R> Read for  Ratelimitor<R> where R:Read  {


    fn read(&mut self,mut  buf: &mut [u8]) -> std::io::Result<usize> {
       if buf.len()>self.limit{
        buf = &mut buf[..self.limit +1]
       }
       let n = self.reader.read(buf)?;
       if n > self.limit {
        return Err(io::Error::new(io::ErrorKind::Other,"too many bites"));
        
           
       }
       self.limit-=n;
       
        return Err(io::Error::new(io::ErrorKind::Other,"too many bites"));

    }
    

    
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
