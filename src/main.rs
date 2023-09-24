pub mod instr;

use byteorder::{BigEndian, ReadBytesExt};
use std::{self, io::Write};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        return Err(String::from(
            "Invalid usage.\nUsage: cargo run <path/to/input.ch8> <path/to/output/dir>",
        ));
    }

    let program_path: &str = &args[0];
    let output_path: &str = &args[1];

    match std::fs::read(program_path) {
        Ok(file_contents) => {
            if let Ok(output_buffer) = std::fs::File::create(output_path) {
                let length = file_contents.len();

                let mut cursor = std::io::Cursor::new(file_contents);
                let mut writer = std::io::BufWriter::new(output_buffer);

                while !(cursor.position() == length.try_into().unwrap()) {
                    if let Ok(opcode) = cursor.read_u16::<BigEndian>() {
                        let instr = instr::Instruction::from_u16(opcode);

                        match instr.msb {
                            0x0 => {}
                            0x1 => {
                                let _ = writer.write_fmt(format_args!(
                                    "JP ${0:03X?} ; Jumps to address ${0:03X?}\n",
                                    instr.address
                                ));
                            }
                            0x2 => {
                                let _ = writer.write_fmt(format_args!(
                                    "CALL ${0:03X?} ; Jumps to subroutine at ${0:03X?}\n",
                                    instr.address
                                ));
                            }
                            0x3 => {
                                let _ = writer.write_fmt(format_args!(
                                    "SE V{0:X?}, {1:02X?} ; Skip the next instruction if the value in register V{0:X?} is equal to {1:02X?}\n", instr.x, instr.kk_byte
                                ));
                            }
                            0x4 => {
                                let _ = writer.write_fmt(format_args!(
                                    "SNE V{0:X?}, {1:02X?} ; Skip the next instruction if the value in register V{0:X?} is not equal to {1:02X?}\n", instr.x, instr.kk_byte
                                ));
                            }
                            0x5 => {
                                let _ = writer.write_fmt(format_args!(
                                    "SE V{0:X?}, V{1:X?} ; Skip the next instruction if the values in registers V{0:X?} and V{0:X?} are equal.\n", instr.x, instr.y
                                ));
                            }
                            0x6 => {
                                let _ = writer.write_fmt(format_args!(
                                    "LD V{0:X?}, {1:02X?} ; Set the value of register V{0:X?} to {1:02X?}\n", instr.x, instr.kk_byte
                                ));
                            }
                            0x7 => {
                                let _ = writer.write_fmt(format_args!(
                                    "ADD V{0:X?}, {1:02X?} ; Add {1:02X?} to the value in register V{0:X?}\n", instr.x, instr.kk_byte
                                ));
                            }
                            0x8 => {
                                match instr.nibble {
                                    0x0 => todo!(),
                                    0x1 => todo!(),
                                    0x2 => todo!(),
                                    0x3 => todo!(),
                                    0x4 => todo!(),
                                    0x5 => todo!(),
                                    0x6 => todo!(),
                                    0x7 => todo!(),
                                    0xE => todo!(),
                                    _ => {
                                        unreachable!(
                                            "No instruction starting with 0x8 can end with {:X}",
                                            instr.nibble
                                        );
                                    }
                                };
                            }
                            0x9 => todo!(),
                            0xA => todo!(),
                            0xB => todo!(),
                            0xC => todo!(),
                            0xD => todo!(),
                            0xE => todo!(),
                            0xF => todo!(),
                            _ => {
                                let _ = writer.write(b"Unknown instruction\n");
                            }
                        }
                    }
                }
            } else {
                return Err(format!("Failed creating file {}", output_path));
            }
        }
        Err(e) => return Err(e.to_string()),
    }
    Ok(())
}
