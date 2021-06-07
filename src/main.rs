use clap::{App, Arg};

pub mod disas;

#[derive(Eq, PartialEq)]
pub enum Mode {
    Raw,
    ELF,
}

fn main() -> std::io::Result<()> {
    let matches = App::new("Vmode")
        .version("0.1")
        .author("Tom <tom@orseti.com>")
        .about("RISC-V Disassembler")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(Arg::with_name("raw").short("r").help(
            "Dissasembles a target file raw,\
                parsing from pos 0 of the file as RISC-V opcodes.",
        ))
        .get_matches();

    let mode = if matches.is_present("raw") {
        Mode::Raw
    } else {
        Mode::ELF
    };

    disas::disassemble(matches.value_of("INPUT").unwrap(), mode)?;

    Ok(())
}
