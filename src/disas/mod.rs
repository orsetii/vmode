mod ins;

use super::{Mode, Mode::*};
use ins::*;
use std::fs;

/// Disassembles a file from `path`, either in `Raw` or `ELF` mode.
/// `Raw` mode parses the file from the start of the buffer as opcodes.
/// `ELF` parses code sections as opcodes.
pub fn disassemble<'a>(path: &'a str, mode: Mode) -> std::io::Result<()> {
    if mode == Raw {
        let f = fs::read(path)?;
        parse_until_err(&f[..])?;
    } else if mode == ELF {
        todo!();
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
        parse_ins(
            i,
            u32::from_le_bytes([buf[i], buf[i + 1], buf[i + 2], buf[i + 3]]),
        );
        if i + 4 >= len {
            break;
        }
        i += 4;
    }
    Ok(())
}

#[inline]
fn parse_ins(i: usize, ins: u32) {
    let opcode = Opcode::from(ins);
    match opcode {
        Opcode::MATHX => {
            let decoded_ins = RType::from(ins);
            let op = MATHX_OPS::from(decoded_ins.funct3);
            let op_name = match op {
                MATHX_OPS::add_sub => match decoded_ins.funct7 {
                    0b0000000 => "add".to_string(),
                    0b0100000 => "sub".to_string(),
                    _ => unreachable!(),
                },
                MATHX_OPS::srl_sra => match decoded_ins.funct7 {
                    0b0000000 => "srl".to_string(),
                    0b0100000 => "sra".to_string(),
                    _ => unreachable!(),
                },
                _ => format!("{:?}", op),
            };

            print_ins(
                i,
                ins,
                op_name,
                &[decoded_ins.rd, decoded_ins.rs1, decoded_ins.rs2],
                &[decoded_ins.funct7],
            );
        }
        Opcode::MATHI => {
            let decoded_ins = IType::from(ins);
            let op = MATHI_OPS::from(decoded_ins.funct3);
            if op == MATHI_OPS::srli_srai {
                let decoded_ins = RType::from(ins);
                print_ins(
                    i,
                    ins,
                    match decoded_ins.funct7 {
                        0b0000000 => "srli".to_string(),
                        0b0100000 => "srai".to_string(),
                        _ => unreachable!(),
                    },
                    &[decoded_ins.rd, decoded_ins.rs1, decoded_ins.rs2],
                    &[decoded_ins.funct7],
                );
            } else {
                print_ins(
                    i,
                    ins,
                    format!("{:?}", op),
                    &[decoded_ins.rd, decoded_ins.rs1],
                    &[decoded_ins.imm],
                );
            }
        }
        Opcode::JAL | Opcode::JALR => {
            print_jmp(i, ins, opcode);
        }
        _ => {}
    }
}

#[inline]
fn print_ins<S, U>(pc_val: usize, full_ins: u32, ins_name: S, regs: &[Register], operands: &[U])
where
    S: Into<String> + std::fmt::Display,
    U: std::ops::Add + std::fmt::Debug + std::fmt::LowerHex,
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
        s = format!("{},{:#x?}", s, i);
    }
    println!("{:>4x}:   {:>8x}   {:>8} {}", pc_val, full_ins, ins_name, s)
}

/// printing jumps is done in a seperate function so we can easily not print
/// the register if it is zero
fn print_jmp(pc_val: usize, ins: u32, opcode: Opcode) {
    match opcode {
        Opcode::JAL => {
            let decoded_ins = UType::from(ins);
            // TODO check for the pseudo-instructions as specficed in p.110 of the spec
            print_ins(pc_val, ins, "jal", &[decoded_ins.rd], &[decoded_ins.imm]);
        }
        Opcode::JALR => {
            let decoded_ins = IType::from(ins);
            // check for and print `ret`
            if decoded_ins.rd == Register::Zero
                && decoded_ins.rs1 == Register::Ra
                && decoded_ins.imm == 0
            {
                return println!("{:>4x}:   {:>8x}   {:>8}", pc_val, ins, "ret");
            }

            print_ins(
                pc_val,
                ins,
                "jalr",
                &[decoded_ins.rd, decoded_ins.rs1],
                &[decoded_ins.imm],
            );
        }
        _ => return,
    }
}
