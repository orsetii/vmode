mod ins;

use super::{Mode, Mode::*};
use ins::*;
use std::fs;

pub fn disassemble<'a>(path: &'a str, mode: Mode) -> std::io::Result<()> {
    if mode == Raw {
        let f = fs::read(path)?;
        parse_until_err(&f[..])?;
    } else if mode == ELF {
        // Parse headers
        // then disas
    } else {
        unreachable!();
    }

    println!("Path: {}", path);
    Ok(())
}

/// parses the provided buffer as raw opcodes until it either runs
/// into an error or reaches the end of the buffer.
fn parse_until_err<'a>(buf: &'a [u8]) -> std::io::Result<()> {
    let len = buf.len();
    let mut i = 0_usize;
    loop {
        let ins = u32::from_le_bytes([buf[i], buf[i + 1], buf[i + 2], buf[i + 3]]);
        let opcode = Opcode::from(ins);

        match opcode {
            Opcode::MATHX => {
                let decoded_ins = RType::from(ins);
                print_ins(
                    i,
                    ins,
                    format!("{:?}", MATHX_OPS::from(decoded_ins.funct3)),
                    &[decoded_ins.rd, decoded_ins.rs1, decoded_ins.rs2],
                    &[decoded_ins.funct7],
                );
            }
            _ => {}
        }

        if i + 4 >= len {
            break;
        }
        i += 4;
    }
    Ok(())
}

fn print_ins<S, U>(pc_val: usize, full_ins: u32, ins_name: S, regs: &[Register], operands: &[U])
where
    S: Into<String> + std::fmt::Display,
    U: std::ops::Add + std::fmt::Debug,
{
    let mut s = format!("{:?}", regs[0]);
    for i in regs[1..].iter() {
        s = format!("{},{:?}", s, i);
    }

    for i in operands.iter() {
        // This is a disgusting hack so we don't print out 0 operands,
        // it is most definitely possible, just generics and signed/unsigned integers
        // are a pain, so this will do (very fucking slow)!
        if format!("{:?}", i) == "0" {
            continue;
        }
        s = format!("{},{:x?}", s, i);
    }
    println!("{:>4x}:   {:>8x}   {:>8} {}", pc_val, full_ins, ins_name, s)
}
