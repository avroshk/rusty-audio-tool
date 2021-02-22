use std::ffi::OsStr;
use std::ffi::OsString;


// use std::io::SeekFrom;
use std::path::PathBuf;

use structopt::StructOpt;

use sndfile::SndFile;
use sndfile::SndFileIO; // Import trait
use sndfile::OpenOptions::{ReadOnly, WriteOnly};
use sndfile::{ReadOptions, WriteOptions, MajorFormat, SubtypeFormat, Endian};

fn get_wavefile_reader(file_path_ref: &PathBuf) -> SndFile {
    let snd = match ReadOnly(ReadOptions::Auto).from_path(file_path_ref) {
        Ok(s) => { s },
        Err(e) => { panic!("Can't open file. (Err: {:?})", e); },
    };
    return snd;
}

fn get_wavefile_writer(file_path_ref: &PathBuf) -> SndFile {
    let snd = match WriteOnly(WriteOptions::new(
        MajorFormat::WAV,
        SubtypeFormat::PCM_16,
        Endian::File,
        44100,
        2
    )).from_path(file_path_ref) {
        Ok(s) => { s },
        Err(e) => { panic!("Can't write file. (Err: {:?})", e); },
    };
    return snd;
}

fn print_wavfile_data(mut wavfile: SndFile) {
    let len = match wavfile.len() {
        Ok(l) => { l },
        Err(e) => { panic!("Couldn't read the length of wav file. (Err: {:?})", e); }
    };
    let samplerate = wavfile.get_samplerate();

    println!("{:#?}", wavfile);
    println!("samplerate={}", samplerate);
    println!("channels={}", wavfile.get_channels());
    println!("format={:?}", wavfile.get_major_format());
    println!("type-format={:?}", wavfile.get_subtype_format());
    println!("duration={:#?}s", len/samplerate as u64);
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
        },
        CliParams::Process{file, out_file} => {
            println!("Args |\n \
                     file={:?}\n \
                     out_file={:?}", file, out_file);
            let mut wavfile: SndFile = get_wavefile_reader(&file);
            let mut out_wavfile: SndFile = get_wavefile_writer(&out_file);

            const BLOCK_LENGTH: usize = 512;

            let mut audio_block: [i16; BLOCK_LENGTH] = [0; BLOCK_LENGTH];
            let audio_slice = &mut audio_block[..];

            // let cursor = match wavfile.seek(SeekFrom::Start(10917750)) {
            //     Ok(c) => c,
            //     _ => panic!("Cannot seek!")
            // };
            // println!("seek_cursor={}", cursor);

            let mut samples_read: usize = 1;
            while samples_read > 0 {
                samples_read = match wavfile.read_to_slice(audio_slice) {
                    Ok(s) => s,
                    _ => panic!("Can't read samples!"),
                };
                println!("Reading... {:?}", samples_read);

                if samples_read > 0 {
                    let _samples_written = match out_wavfile.write_from_slice(audio_slice) {
                        Ok(s) => s,
                        _ => panic!("Can't write samples!"),
                    };
                }
            }
        },
        CliParams::Analyze{file, analyze_test} => {
            println!("Args |\n \
                     file={:?}\n \
                     analyze_test={}", file, analyze_test);
        }
    }
}
