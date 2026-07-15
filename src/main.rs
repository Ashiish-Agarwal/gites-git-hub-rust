use anyhow::{Ok, Result,Context};
use flate2::{ read::ZlibDecoder};
use clap::{Parser, Subcommand, builder::Str};
use std::{fs, io::{BufRead, BufReader, Read, Stdout}, vec};
use std::ffi::CStr;
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
    // let Some(size)= header.strip_prefix("blob ")else {
    //     anyhow::bail!("git/object file not statting with blob ")
    // };
    let kind = match Kind {
        "blob"=>Kind::blob,
        _=>anyhow::bail!("kwe do not yet know how to print a '{Kind}'")
};


    let size = size.parse::<usize>().context("get header file is invalid size ")?;
    buff.clear();
    buff.resize(size,0);
    z.read_exact(&mut buff[..]).context("read true content of .gites/object file")?;
    let n = z.read(&mut [0]).context("validate eof in .git/object file had {n} trailing bytes")?;
    anyhow::ensure!(n == 0,"git object file had {n} trait");
    let stdout = std::io::stdout();
    let mut Stdout = stdout.lock();

    


    
}
   
    
}
   
Ok(())

}