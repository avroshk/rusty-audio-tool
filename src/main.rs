use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;

use structopt::StructOpt;

use sndfile::SndFile;
use sndfile::SndFileIO;
use sndfile::OpenOptions::ReadOnly;
use sndfile::ReadOptions;

fn get_wavefile_reader(file_path_ref: &PathBuf) -> SndFile {
    let snd = match ReadOnly(ReadOptions::Auto).from_path(file_path_ref) {
        Ok(s) => { s },
        Err(e) => { panic!("Can't open file. (Err: {:?})", e); },
    };
    return snd;
}

fn print_wavfile_data(wavfile: SndFile) {
    println!("{:#?}", wavfile);
    println!("samplerate={}", wavfile.get_samplerate());
    println!("channels={}", wavfile.get_channels());
    println!("format={:?}", wavfile.get_major_format());
    println!("type-format={:?}", wavfile.get_subtype_format());
}

fn check_path_exists(path_str: &OsStr) -> Result<PathBuf, OsString> {
    let path = PathBuf::from(path_str);
    match path.exists() {
        true => Ok(path),
        false => Err(OsString::from("Specified os path does not exist."))
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Rusty Audio Tool", about = "Load, process and analyze audio from command line", version = "0.1.0")]
enum CliParams {
    Load {
        /// Input wav file
        #[structopt(parse(try_from_os_str = check_path_exists))]
        file: PathBuf,
        /// Print everything
        #[structopt(short,long)]
        print: bool,
    },
    Process {
        /// Input wav file
        #[structopt(parse(try_from_os_str = check_path_exists))]
        file: PathBuf,
        /// Output wav file
        #[structopt(short, long, default_value = "out.wav")]
        out_file: PathBuf
    },
    Analyze {
        /// Input wav file
        #[structopt(parse(try_from_os_str = check_path_exists))]
        file: PathBuf,
        #[structopt(short,long)]
        analyze_test: bool,
    }
}

fn main() {
    match CliParams::from_args() {
        CliParams::Load{file, print} => {
            println!("Args |\n \
                     file={:?}\n \
                     print={}", file, print);
            let wavfile: SndFile = get_wavefile_reader(&file);
            if print { print_wavfile_data(wavfile); }

            // let len = match wavfile.len() {
            //     Ok(l) => { l },
            //     Err(e) => { panic!("Couldn't read the length of wav file. (Err: {:?})", e); }
            // };

        },
        CliParams::Process{file, out_file} => {
            println!("Args |\n \
                     file={:?}\n \
                     out_file={:?}", file, out_file);

             //     const BLOCK_LENGTH: usize = 512;
             //     let mut audio_block: [i16; BLOCK_LENGTH] = [0; BLOCK_LENGTH];
             //     let audio_slice = &mut audio_block[..];
             //
             //     for i in 1..10 {
             //         match SndFileIO::read_to_slice(&mut wavfile, audio_slice) {
             //             Ok(samples_read) => { println!("Reading... {:?}", samples_read); },
             //             Err(e) => { panic!("Err: {:?}", e); }
             //         }
             //     }
             //
             //     for (i, sample) in audio_slice.iter().enumerate() {
             //         println!("{}:{}", i, sample);
             //     }
        },
        CliParams::Analyze{file, analyze_test} => {
            println!("Args |\n \
                     file={:?}\n \
                     analyze_test={}", file, analyze_test);
        }
    }
}
