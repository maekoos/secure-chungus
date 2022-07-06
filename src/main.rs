use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use schungus::{combine_chunks, generate_chunks, Chunk, Error};

fn main() {
    if let Err(e) = run() {
        println!("Error: {}", e);
    }
}

fn run() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Expected a [s]plit or [j]oin command: \"{} s|j\"", args[0]);
        return Ok(());
    }

    match &*args[1] {
        "s" => {
            let print_usage = || {
                println!("Split usage:");
                println!(
                    "\t{} s <INPUT FILE PATH> <NUMBER OF CHUNKS> <CHUNK PREFIX>",
                    args[0]
                );
            };

            if args.len() != 2 + 3 {
                print_usage();
                return Ok(());
            }

            let input_fp = &*args[2];
            let n_chunks: u64 = match args[3].parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("Invalid number of chunks.\n");
                    print_usage();
                    return Ok(());
                }
            };
            let chunk_prefix = &*args[4];

            println!("Reading input file: '{}'", input_fp);
            let mut input = Vec::new();
            {
                let mut file = File::open(input_fp)?;
                file.read_to_end(&mut input)?;
            }

            let chunks = generate_chunks(n_chunks, &input)?;
            for chunk in chunks {
                let out_path = format!("{}{}", chunk_prefix, chunk.index);
                println!("Writing to '{}'", out_path);
                let mut file = File::create(out_path)?;
                file.write_all(&chunk.to_buf())?;
            }

            Ok(())
        }
        "j" => {
            let print_usage = || {
                println!("Join usage:");
                println!(
                    "\t{} j <OUTPUT PATH> <CHUNK FILE 1> [CHUNK FILE 2] [CHUNK FILE N]",
                    args[0]
                );
            };

            if args.len() < 2 + 1 + 1 {
                print_usage();
                return Ok(());
            }

            let out_fp = &args[2];
            let chunk_fps = args[3..].to_owned();

            let mut chunks = Vec::new();
            for fp in &chunk_fps {
                println!("Reading '{}'...", fp);
                let mut file = File::open(fp)?;
                let mut buf: Vec<u8> = Vec::new();
                file.read_to_end(&mut buf)?;
                chunks.push(Chunk::from_buf(&buf)?);
            }

            println!("All chunk-files were read and parsed successfully!");
            println!("Merging chunks...");

            let out = combine_chunks(chunks)?;
            println!("Writing the output file to '{}'", out_fp);

            {
                let mut file = File::create(out_fp)?;
                file.write_all(&out)?;
            }

            Ok(())
        }
        c => {
            println!("Unknown command: {}", c);
            println!("Expected one of [s, j]");
            Ok(())
        }
    }
}
