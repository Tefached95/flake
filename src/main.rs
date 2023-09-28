pub mod instr;

use byteorder::{BigEndian, ReadBytesExt};
use std::{self, io::Write, path::Path};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        return Err(String::from(
            r#"Invalid usage.
            Usage: `cargo run <path/to/input.ch8> <path/to/output/dir>`
            The program takes a `.ch8` file, and outputs an `.asm` file with the same base name as the input.
            Example: `cargo run /home/me/Documents/maze.ch8 /home/me/Documents/` will produce a `maze.asm` file in `/home/me/Documents`"#,
        ));
    }

    let program_path: &str = &args[1];
    let output_file_name: String;
    if let Some(base_name) = Path::new(program_path).file_stem() {
        output_file_name = String::from(format!("{}.asm", base_name.to_str().unwrap()))
    } else {
        return Err(String::from(format!(
            "Failed to read base name from input path: {}",
            program_path
        )));
    }
    let output_path: String = format!(
        "{}/{}",
        String::from(&args[2]).trim_end_matches("/"),
        output_file_name
    );

    let file_contents = std::fs::read(program_path)
        .map_err(|err| format!("Failed to open {program_path}: {err}"))?;
    let file_contents_len = file_contents.len();

    let output_buffer = std::fs::File::create(&output_path)
        .map_err(|err| format!("Failed creating file at {output_path}: {err}"))?;

    let mut cursor = std::io::Cursor::new(file_contents);
    let mut writer = std::io::BufWriter::new(output_buffer);

    while !(cursor.position() == file_contents_len.try_into().unwrap()) {
        if let Ok(opcode) = cursor.read_u16::<BigEndian>() {
            let instr = instr::Instruction::from_u16(opcode);

            match instr.msb {
                0x0 => {}
                0x1 => {
                    writer
                        .write_fmt(format_args!(
                            "JP ${0:03X?} ; Jumps to address ${0:03X?}\n",
                            instr.address
                        ))
                        .unwrap_or_default();
                }
                0x2 => {
                    writer
                        .write_fmt(format_args!(
                            "CALL ${0:03X?} ; Jumps to subroutine at ${0:03X?}\n",
                            instr.address
                        ))
                        .unwrap_or_default();
                }
                0x3 => {
                    writer.write_fmt(format_args!(
                        "SE V{0:X?}, {1:02X?} ;; Skip the next instruction if the value in register V{0:X?} is equal to {1:02X?}\n",
                        instr.x,
                        instr.kk_byte
                    )).unwrap_or_default();
                }
                0x4 => {
                    writer.write_fmt(format_args!(
                        "SNE V{0:X?}, {1:02X?} ;; Skip the next instruction if the value in register V{0:X?} is not equal to {1:02X?}\n",
                        instr.x,
                        instr.kk_byte
                    )).unwrap_or_default();
                }
                0x5 => {
                    writer.write_fmt(format_args!(
                        "SE V{0:X?}, V{1:X?} ;; Skip the next instruction if the values in registers V{0:X?} and V{0:X?} are equal.\n",
                        instr.x,
                        instr.y
                    )).unwrap_or_default();
                }
                0x6 => {
                    writer
                        .write_fmt(format_args!(
                        "LD V{0:X?}, {1:02X?} ;; Set the value of register V{0:X?} to {1:02X?}\n",
                        instr.x,
                        instr.kk_byte
                    ))
                        .unwrap_or_default();
                }
                0x7 => {
                    writer
                        .write_fmt(format_args!(
                        "ADD V{0:X?}, {1:02X?} ;; Add {1:02X?} to the value in register V{0:X?}\n",
                        instr.x, instr.kk_byte
                    ))
                        .unwrap_or_default();
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
                0xA => {
                    writer
                        .write_fmt(format_args!(
                            "LD I, ${0:03X} ;; Load address ${0:03X} into the special register I\n",
                            instr.address
                        ))
                        .unwrap_or_default();
                }
                0xB => todo!(),
                0xC => todo!(),
                0xD => todo!(),
                0xE => todo!(),
                0xF => todo!(),
                _ => {
                    writer.write(b"Unknown instruction\n").unwrap_or_default();
                }
            }
        }
    }

    Ok(())
}
