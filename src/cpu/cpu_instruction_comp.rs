use std::collections::HashMap;

use crate::cpu::cpu::{Cpu, Xlen};

pub struct CompressedOpecode {
    pub operation: fn(cpu: &Cpu, word: u16) -> Result<&'static CompressedInstruction, ()>,
}

pub struct CompressedInstruction {
    pub mnemonic: &'static str,
    pub decompress: fn(cpu: &Cpu, word: u16) -> Result<u32, ()>,
    pub disassemble: fn(cpu: &Cpu, mnemonic: &str, word: u16) -> String,
}

lazy_static! {
    static ref COMPRESSED_OPECODES: HashMap<u8, CompressedOpecode> = {
        let mut m = HashMap::new();
        m.insert(0x0, CompressedOpecode {operation: opecode_0});
        m.insert(0x1, CompressedOpecode {operation: opecode_1});
        m.insert(0x2, CompressedOpecode {operation: opecode_2});
        m
    };

    static ref COMPRESSED_INSTRUCTIONS_GROUP0: HashMap<u8, CompressedInstruction> = {
        let mut m = HashMap::new();
        m.insert(0, CompressedInstruction {
            mnemonic: "c.addi4spn",
            decompress: c_addi4spn,
            disassemble: disassemble_mnemonic,
        });
        m.insert(1, CompressedInstruction {
            mnemonic: "c.fld",
            decompress: c_fld,
            disassemble: disassemble_mnemonic,
        });
        m.insert(2, CompressedInstruction {
            mnemonic: "c.lw",
            decompress: c_lw,
            disassemble: disassemble_mnemonic,
        });
        m.insert(5, CompressedInstruction {
            mnemonic: "c.fsd",
            decompress: c_fsd,
            disassemble: disassemble_mnemonic,
        });
        m.insert(6, CompressedInstruction {
            mnemonic: "c.sw",
            decompress: c_sw,
            disassemble: disassemble_mnemonic,
        });
        m
    };
    static ref COMPRESSED_INSTRUCTIONS_GROUP0_SUB: HashMap<(u8, u8), CompressedInstruction> = {
        let mut m = HashMap::new();
        m.insert((0, 3), CompressedInstruction { // FV32FC only.
            mnemonic: "c.flw",
            decompress: c_flw,
            disassemble: disassemble_mnemonic,
        });
        m.insert((1, 3), CompressedInstruction { // FC64IC only.
            mnemonic: "c.ld",
            decompress: c_ld,
            disassemble: disassemble_mnemonic,
        });
        m.insert((0, 7), CompressedInstruction { // FV32FC only.
            mnemonic: "c.fsw",
            decompress: c_fsw,
            disassemble: disassemble_mnemonic,
        });
        m.insert((1, 7), CompressedInstruction { // FC64IC only.
            mnemonic: "c.sd",
            decompress: c_sd,
            disassemble: disassemble_mnemonic,
        });
        m
    };

    static ref COMPRESSED_INSTRUCTIONS_GROUP1: HashMap<u8, CompressedInstruction> = {
        let mut m = HashMap::new();
        m.insert(2, CompressedInstruction {
            mnemonic: "c.li",
            decompress: c_li,
            disassemble: disassemble_mnemonic,
        });
        m.insert(5, CompressedInstruction {
            mnemonic: "c.j",
            decompress: c_j,
            disassemble: disassemble_mnemonic,
        });
        m.insert(6, CompressedInstruction {
            mnemonic: "c.beqz",
            decompress: c_beqz,
            disassemble: disassemble_mnemonic,
        });
        m.insert(7, CompressedInstruction {
            mnemonic: "c.bnez",
            decompress: c_bnez,
            disassemble: disassemble_mnemonic,
        });
        m
    };

    static ref COMPRESSED_INSTRUCTIONS_GROUP2: HashMap<u8, CompressedInstruction> = {
        let mut m = HashMap::new();
        m.insert(0, CompressedInstruction {
            mnemonic: "c.slli",
            decompress: c_slli,
            disassemble: disassemble_mnemonic,
        });
        m.insert(1, CompressedInstruction {
            mnemonic: "c.fldsp",
            decompress: c_fldsp,
            disassemble: disassemble_mnemonic,
        });
        m.insert(2, CompressedInstruction {
            mnemonic: "c.lwsp",
            decompress: c_lwsp,
            disassemble: disassemble_mnemonic,
        });
        m.insert(5, CompressedInstruction {
            mnemonic: "c.fsdsp",
            decompress: c_fsdsp,
            disassemble: disassemble_mnemonic,
        });
        m.insert(6, CompressedInstruction {
            mnemonic: "c.swsp",
            decompress: c_swsp,
            disassemble: disassemble_mnemonic,
        });
        m
    };}
fn opecode_0(cpu: &Cpu, word: u16) -> Result<&'static CompressedInstruction, ()> {
    let funct3 = ((word >> 13) & 0x7) as u8;
    match funct3 {
        3 | 7 => match COMPRESSED_INSTRUCTIONS_GROUP0_SUB.get(&(cpu.xlen.clone() as u8, funct3)) {
            Some(instruction) => Ok(&instruction),
            None => panic!("Not found instruction!"),
        },
        _ => match COMPRESSED_INSTRUCTIONS_GROUP0.get(&funct3) {
            Some(instruction) => Ok(&instruction),
            None => panic!("Not found instruction!"),
        },
    }
}

fn opecode_1(cpu: &Cpu, word: u16) -> Result<&'static CompressedInstruction, ()> {
    let funct3 = ((word >> 13) & 0x7) as u8;
    match funct3 {
        0 => match word == 1 {
            true => Ok(&CompressedInstruction {
                mnemonic: "c.nop",
                decompress: c_nop,
                disassemble: disassemble_mnemonic,
            }),
            false => Ok(&CompressedInstruction {
                mnemonic: "c.addi",
                decompress: c_addi,
                disassemble: disassemble_mnemonic,
            }),
        },
        1 => match cpu.xlen {
            Xlen::X32 => Ok(&CompressedInstruction {
                mnemonic: "c.jal",
                decompress: c_jal,
                disassemble: disassemble_mnemonic,
            }),
            _ => Ok(&CompressedInstruction {
                mnemonic: "c.addiw",
                decompress: c_addiw,
                disassemble: disassemble_mnemonic,
            }),
        },
        3 => match word & 0xf00 {
            1 => Ok(&CompressedInstruction {
                mnemonic: "c.addi16sp",
                decompress: c_addi16sp,
                disassemble: disassemble_mnemonic,
            }),
            _ => Ok(&CompressedInstruction {
                mnemonic: "c.lui",
                decompress: c_lui,
                disassemble: disassemble_mnemonic,
            }),
        },
        4 => match (word >> 10) & 0x3 {
            0 => Ok(&CompressedInstruction {
                mnemonic: "c.srli",
                decompress: c_srli,
                disassemble: disassemble_mnemonic,
            }),
            1 => Ok(&CompressedInstruction {
                mnemonic: "c.srai",
                decompress: c_srai,
                disassemble: disassemble_mnemonic,
            }),
            2 => Ok(&CompressedInstruction {
                mnemonic: "c.andi",
                decompress: c_andi,
                disassemble: disassemble_mnemonic,
            }),
            _ => match (word >> 5) & 0x3 {
                0 => match (word >> 12) & 0x1 {
                    0 => Ok(&CompressedInstruction {
                        mnemonic: "c.sub",
                        decompress: c_sub,
                        disassemble: disassemble_mnemonic,
                    }),
                    _ => Ok(&CompressedInstruction {
                        mnemonic: "c.subw",
                        decompress: c_subw,
                        disassemble: disassemble_mnemonic,
                    }),
                },
                1 => match (word >> 12) & 0x1 {
                    0 => Ok(&CompressedInstruction {
                        mnemonic: "c.xor",
                        decompress: c_xor,
                        disassemble: disassemble_mnemonic,
                    }),
                    _ => Ok(&CompressedInstruction {
                        mnemonic: "c.addw",
                        decompress: c_addw,
                        disassemble: disassemble_mnemonic,
                    }),
                },
                2 => Ok(&CompressedInstruction {
                    mnemonic: "c.or",
                    decompress: c_or,
                    disassemble: disassemble_mnemonic,
                }),
                _ => Ok(&CompressedInstruction {
                    mnemonic: "c.and",
                    decompress: c_and,
                    disassemble: disassemble_mnemonic,
                }),
            },
        },
        _ => match COMPRESSED_INSTRUCTIONS_GROUP1.get(&funct3) {
            Some(instruction) => Ok(&instruction),
            None => panic!("Not found instruction!"),
        },
    }
}

fn opecode_2(cpu: &Cpu, word: u16) -> Result<&'static CompressedInstruction, ()> {
    let funct3 = ((word >> 13) & 0x7) as u8;
    match funct3 {
        3 => match cpu.xlen {
            Xlen::X32 => Ok(&CompressedInstruction { // RV32FC only.
                mnemonic: "c.flwsp",
                decompress: c_flwsp,
                disassemble: disassemble_mnemonic,
            }),
            _ => Ok(&CompressedInstruction { // RV64IC only.
                mnemonic: "c.ldsp",
                decompress: c_ldsp,
                disassemble: disassemble_mnemonic,
            }),
        },
        4 => match (word >> 12) & 0x1 {
            0 => match (word >> 2) & 0x1f {
                0 => Ok(&CompressedInstruction {
                    mnemonic: "c.jr",
                    decompress: c_jr,
                    disassemble: disassemble_mnemonic,
                }),
                _ => Ok(&CompressedInstruction {
                    mnemonic: "c.mv",
                    decompress: c_mv,
                    disassemble: disassemble_mnemonic,
                }),
            },
            _ => match (word >> 2) & 0x3ff {
                0 => Ok(&CompressedInstruction {
                    mnemonic: "c.ebreak",
                    decompress: c_ebreak,
                    disassemble: disassemble_mnemonic,
                }),
                _ => match (word >> 2) & 0x1f {
                    0 => Ok(&CompressedInstruction {
                        mnemonic: "c.jalr",
                        decompress: c_jalr,
                        disassemble: disassemble_mnemonic,
                    }),
                    _ => Ok(&CompressedInstruction {
                        mnemonic: "c.add",
                        decompress: c_add,
                        disassemble: disassemble_mnemonic,
                    }),
                },
            },
        },
        7 => match cpu.xlen {
            Xlen::X32 => Ok(&CompressedInstruction { // RV32FC only.
                mnemonic: "c.fswsp",
                decompress: c_fswsp,
                disassemble: disassemble_mnemonic,
            }),
            _ => Ok(&CompressedInstruction { // RV64IC only.
                mnemonic: "c.sdsp",
                decompress: c_sdsp,
                disassemble: disassemble_mnemonic,
            }),
        },
        _ => match COMPRESSED_INSTRUCTIONS_GROUP2.get(&funct3) {
            Some(instruction) => Ok(&instruction),
            None => panic!("Not found instruction!"),
        },
    }
}

pub fn instruction_decompress(cpu: &Cpu, instruction_addr: u64, word: u32) -> Result<u32, ()> {
    let compressed_word = (word & 0xffff) as u16;

    let opecodes = match COMPRESSED_OPECODES.get(&((word & 0x3) as u8)) {
        Some(ops) => ops,
        None => panic!("Not found opecode: {:016x}", word),
    };

    match (opecodes.operation)(cpu, compressed_word) {
        Ok(instruction) => (instruction.decompress)(cpu, compressed_word),
        Err(()) => panic!("Not found instruction: {:016x}", instruction_addr),
    }
}

fn disassemble_mnemonic(_cpu: &Cpu, mnemonic: &str, _word: u16) -> String {
    let mut s = String::new();
    s += &format!("{}", mnemonic);
    s
}

/// [c.addi4spn rd’,uimm]
fn c_addi4spn(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 2) & 0x7) as u32;
    let uimm =
        (((word >> 7) & 0x30) | ((word >> 1) & 0x3c0) | ((word >> 4) & 0x4) | ((word >> 2) & 0x8))
            as u32;
    match uimm {
        0 => Err(()),
        _ => {
            // addi rd, x2, uimm
            let op = 0x13 as u32;
            let imm = uimm << 20;
            let rs1 = (2 << 15) as u32;
            let rd = (rd_ + 8) << 7;
            Ok(imm | rs1 | rd | op)
        }
    }
}

/// [c.fld rd’,uimm(rs1’)]
fn c_fld(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.lw rd’,uimm(rs1’)]
fn c_lw(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.flw rd’,uimm(rs1’)]
fn c_flw(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.ld rd’,uimm(rs1’)]
fn c_ld(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.fsd rd’,uimm(rs1’)]
fn c_fsd(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.sw rd’,uimm(rs1’)]
fn c_sw(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.fsw rd’,uimm(rs1’)]
fn c_fsw(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.sd rd’,uimm(rs1’)]
fn c_sd(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.nop]
fn c_nop(cpu: &Cpu, _word: u16) -> Result<u32, ()> {
    // addi x0,x0,0
    Ok(0x13)
}

/// [c.addi rd,u[12:12]|u[6:2]]
fn c_addi(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x1f) as u32;
    let mut imm = ((word & 0x1000) >> 7 | (word & 0xfc >> 2)) as u32;

    // addi rd,rs1,imm
    let op = 0x13 as u32;
    imm = match word & 0x1000 {
        0 => 0,
        _ => 0xfffc0000
    } | imm;
    let rd = rd_ << 7;
    let rs1 = rd_ << 15;
    Ok(imm | rs1 | rd | op)
}

/// [c.jal offset]
fn c_jal(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.addiw rd,imm]
fn c_addiw(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.li rd,uimm]
fn c_li(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.addi16sp imm]
fn c_addi16sp(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.lui rd,uimm]
fn c_lui(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x1f) as u32;
    let mut imm = ((word & 0x1000) << 5 | (word & 0xfc << 10)) as u32;
    match imm {
        0 => Err(()),
        _ => {
            // lui rd,imm
            let op = 0x37 as u32;
            imm = match word & 0x1000 {
                0 => 0,
                _ => 0xfffc0000
            } | imm;
            let rd = rd_ << 7;
            Ok(imm | rd | op)
        }
    }
}

/// [c.srli rd’,uimm]
fn c_srli(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.srai rd’,uimm]
fn c_srai(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.andi rd’,uimm]
fn c_andi(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.sub rd’,rd’]
fn c_sub(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.xor rd’,rd’]
fn c_xor(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.or rd’,rd’]
fn c_or(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.and rd’,rd’]
fn c_and(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.subw rd’,rs2’]
fn c_subw(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.addw rd’,rs2’]
fn c_addw(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.j offset]
fn c_j(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.beqz rs1’,offset]
fn c_beqz(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.bnez rs1’,offset]
fn c_bnez(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.slli rd,uimm]
fn c_slli(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.fldsp rd,uimm(x2)]
fn c_fldsp(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.lwsp rd,uimm(x2)]
fn c_lwsp(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.flwsp rd,uimm(x2)]
fn c_flwsp(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.ldsp rd,uimm(x2)]
fn c_ldsp(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.jr rs1]
fn c_jr(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.mv rd,rs2’]
fn c_mv(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.ebreak]
fn c_ebreak(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.jalr rd]
fn c_jalr(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.add rd,rs2’]
fn c_add(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x1f) as u32;
    let rs2_ = ((word >> 2) & 0x1f) as u32;

    // add rd,rs1,rs2
    match rd_ == 0 || rs2_ == 0 {
        true => Err(()),
        _ => {
            // lui rd,imm
            let op = 0x33 as u32;
            let rd = rd_ << 7;
            let rs1 = rd_ << 15;
            let rs2 = rs2_ << 20;
            Ok(rs2 | rs1 | rd | op)
        }
    }
}

/// [c.fsdsp rs2,uimm(x2)]
fn c_fsdsp(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.swsp rs2,uimm(x2)]
fn c_swsp(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.fswsp rs2,uimm(rs2)]
fn c_fswsp(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}

/// [c.sdsp rs2,uimm(x2)]
fn c_sdsp(cpu: &Cpu, word: u16) -> Result<u32, ()> {
    panic!("TODO");
    Ok(0)
}