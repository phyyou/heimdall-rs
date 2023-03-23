use std::str::FromStr;

use ethers::types::U256;

use crate::{utils::strings::encode_hex_reduced, constants::{WORD_REGEX, MEMLEN_REGEX}};

use super::evm::opcodes::*;

fn is_ext_call_precompile(precompile_address: U256) -> bool {
    let address: usize = match precompile_address.try_into() {
        Ok(x) => x,
        Err(_) => usize::MAX,
    };

    matches!(address, 1 | 2 | 3)
}

impl WrappedOpcode {

    // Returns a WrappedOpcode's solidity representation.
    pub fn solidify(&self) -> String {
        let mut solidified_wrapped_opcode = String::new();

        match self.opcode.name {
            "ADD" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} + {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "MUL" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} * {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "SUB" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} - {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "DIV" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} / {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "SDIV" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} / {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "MOD" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} % {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "SMOD" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} % {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "ADDMOD" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} + {} % {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify(),
                            self.inputs[2]._solidify()
                    ).as_str()
                );
            },
            "MULMOD" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "({} * {}) % {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify(),
                            self.inputs[2]._solidify()
                    ).as_str()
                );
            },
            "EXP" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} ** {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "LT" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} < {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "GT" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} > {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "SLT" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} < {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "SGT" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} > {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "EQ" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} == {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "ISZERO" => {
                let solidified_input = self.inputs[0]._solidify();

                match solidified_input.contains(' ') {
                    true => {
                        solidified_wrapped_opcode.push_str(
                            format!(
                                    "!({})",
                                    self.inputs[0]._solidify()
                            ).as_str()
                        );
                    }
                    false => {
                        solidified_wrapped_opcode.push_str(
                            format!(
                                    "!{}",
                                    self.inputs[0]._solidify()
                            ).as_str()
                        );
                    }
                }
                
            },
            "AND" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "({}) & ({})",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "OR" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} | {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "XOR" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} ^ {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "NOT" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "~({})",
                            self.inputs[0]._solidify()
                    ).as_str()
                );
            },
            "SHL" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} << {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "SHR" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} >> {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "SAR" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} >> {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "BYTE" => {
                solidified_wrapped_opcode.push_str(self.inputs[1]._solidify().as_str());
            }
            "SHA3" => {
                
                solidified_wrapped_opcode.push_str(
                    &format!(
                        "keccak256(memory[{}])",
                        self.inputs[0]._solidify().as_str()
                    )
                );
            },
            "ADDRESS" => {
                solidified_wrapped_opcode.push_str("address(this)");
            },
            "BALANCE" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "address({}).balance",
                            self.inputs[0]._solidify()
                    ).as_str()
                );
            },
            "ORIGIN" => {
                solidified_wrapped_opcode.push_str("tx.origin");
            },
            "CALLER" => {
                solidified_wrapped_opcode.push_str("msg.sender");
            },
            "CALLVALUE" => {
                solidified_wrapped_opcode.push_str("msg.value");
            },
            "CALLDATALOAD" => {
                let solidified_slot = self.inputs[0]._solidify();

                // are dealing with a slot that is a constant, we can just use the slot directly
                if WORD_REGEX.is_match(&solidified_slot).unwrap() {
                    
                    // convert to usize
                    match usize::from_str_radix(
                        &solidified_slot.replace("0x", ""),
                        16
                    ) {
                        Ok(slot) => {
                            solidified_wrapped_opcode.push_str(
                                format!( "arg{}", (slot-4)/32 ).as_str()
                            );
                        },
                        Err(_) => {
                            if solidified_slot.contains("0x04 + ") ||
                               solidified_slot.contains("+ 0x04")
                            {
                                solidified_wrapped_opcode.push_str(
                                    solidified_slot.replace("0x04 + ", "").replace("+ 0x04", "").as_str()
                                );
                            }
                            else {
                                solidified_wrapped_opcode.push_str(
                                    format!( "msg.data[{solidified_slot}]").as_str()
                                );
                            }
                        }
                    };

                    

                }
                else {
                    solidified_wrapped_opcode.push_str(
                        format!("msg.data[{solidified_slot}]").as_str()
                    );
                }
            },
            "CALLDATASIZE" => {
                solidified_wrapped_opcode.push_str("msg.data.length",);
            },
            "CODESIZE" => {
                solidified_wrapped_opcode.push_str("this.code.length",);
            },
            "EXTCODESIZE" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "address({}).code.length",
                            self.inputs[0]._solidify()
                    ).as_str()
                );
            },
            "EXTCODEHASH" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "address({}).codehash",
                            self.inputs[0]._solidify()
                    ).as_str()
                );
            },
            "BLOCKHASH" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "blockhash({})",
                            self.inputs[0]._solidify()
                    ).as_str()
                );
            },
            "COINBASE" => {
                solidified_wrapped_opcode.push_str("block.coinbase");
            },
            "TIMESTAMP" => {
                solidified_wrapped_opcode.push_str("block.timestamp");
            },
            "NUMBER" => {
                solidified_wrapped_opcode.push_str("block.number");
            },
            "DIFFICULTY" => {
                solidified_wrapped_opcode.push_str("block.difficulty");
            },
            "GASLIMIT" => {
                solidified_wrapped_opcode.push_str("block.gaslimit");
            },
            "CHAINID" => {
                solidified_wrapped_opcode.push_str("block.chainid");
            },
            "SELFBALANCE" => {
                solidified_wrapped_opcode.push_str("address(this).balance");
            },
            "BASEFEE" => {
                solidified_wrapped_opcode.push_str("block.basefee");
            },
            "GAS" => {
                solidified_wrapped_opcode.push_str("gasleft()");
            },
            "GASPRICE" => {
                solidified_wrapped_opcode.push_str("tx.gasprice");
            },
            "SLOAD" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "storage[{}]",
                            self.inputs[0]._solidify()
                    ).as_str()
                );
            },
            "MLOAD" => {
                let memloc = self.inputs[0]._solidify();
                
                if memloc.contains("memory") {
                    if memloc.contains('+') {
                        let parts = memloc.split(" + ").collect::<Vec<&str>>();

                        solidified_wrapped_opcode.push_str(
                            format!(
                                    "memory[{}][{}]",
                                    parts[0].replace("memory[", "").replace(']', ""),
                                    parts[1].replace("memory[", "").replace(']', ""),
                            ).as_str()
                        );
                    }
                    else {
                        match MEMLEN_REGEX.find(&format!("memory[{memloc}]")).unwrap() {
                            Some(_) => {
                                solidified_wrapped_opcode.push_str(
                                    format!("{memloc}.length").as_str()
                                );
                            },
                            None => {
                                solidified_wrapped_opcode.push_str(
                                    format!("memory[{memloc}]").as_str()
                                );
                            }
                        }
                    }
                }
                else {
                    solidified_wrapped_opcode.push_str(
                        format!("memory[{memloc}]").as_str()
                    );
                }
                

            },
            "MSIZE" => {
                solidified_wrapped_opcode.push_str(
                    "memory.length"
                );
            },
            "CALL" => {
                match U256::from_str(&self.inputs[1]._solidify()) {
                    Ok(addr) => {
                        if is_ext_call_precompile(addr) {
                            solidified_wrapped_opcode.push_str(&format!("memory[{}]", self.inputs[5]._solidify()));
                        }
                        else {
                            solidified_wrapped_opcode.push_str("ret0");

                        }
                    },
                    Err(_) => {
                        solidified_wrapped_opcode.push_str("ret0");
                    }
                };
            }
            "CALLCODE" => {
                match U256::from_str(&self.inputs[1]._solidify()) {
                    Ok(addr) => {
                        if is_ext_call_precompile(addr) {
                            solidified_wrapped_opcode.push_str(&format!("memory[{}]", self.inputs[5]._solidify()));
                        }
                        else {
                            solidified_wrapped_opcode.push_str("ret0");

                        }
                    },
                    Err(_) => {
                        solidified_wrapped_opcode.push_str("ret0");
                    }
                };            }
            "DELEGATECALL" => {
                match U256::from_str(&self.inputs[1]._solidify()) {
                    Ok(addr) => {
                        if is_ext_call_precompile(addr) {
                            solidified_wrapped_opcode.push_str(&format!("memory[{}]", self.inputs[5]._solidify()));
                        }
                        else {
                            solidified_wrapped_opcode.push_str("ret0");

                        }
                    },
                    Err(_) => {
                        solidified_wrapped_opcode.push_str("ret0");
                    }
                };            }
            "STATICCALL" => {
                match U256::from_str(&self.inputs[1]._solidify()) {
                    Ok(addr) => {
                        if is_ext_call_precompile(addr) {
                            solidified_wrapped_opcode.push_str(&format!("memory[{}]", self.inputs[5]._solidify()));
                        }
                        else {
                            solidified_wrapped_opcode.push_str("ret0");

                        }
                    },
                    Err(_) => {
                        solidified_wrapped_opcode.push_str("ret0");
                    }
                };            }
            "RETURNDATASIZE" => {
                solidified_wrapped_opcode.push_str("ret0.length");
            }
            opcode => {

                if opcode.starts_with("PUSH") {
                    solidified_wrapped_opcode.push_str(self.inputs[0]._solidify().as_str());
                }
                else {
                    solidified_wrapped_opcode.push_str(opcode.to_string().as_str());
                }
            }
        }

        solidified_wrapped_opcode
    }


    // creates a new WrappedOpcode from a set of raw inputs
    pub fn new(opcode_int: usize, inputs: Vec<WrappedInput>) -> WrappedOpcode {
        let mut opcode_str = format!("{opcode_int:x}");
        if opcode_str.len() == 1 {
            opcode_str.insert(0, '0');
        };

        WrappedOpcode {
            opcode: opcode(&opcode_str),
            inputs: inputs,
        }
    }

}

impl Default for WrappedOpcode {
    fn default() -> Self {
        WrappedOpcode {
            opcode: Opcode {
                name: "unknown",
                mingas: 0,
                inputs: 0,
                outputs: 0,
            },
            inputs: Vec::new(),
        }
    }
}


impl WrappedInput {

    // Returns a WrappedInput's solidity representation.
    fn _solidify(&self) -> String {
        let mut solidified_wrapped_input = String::new();

        match self {
            WrappedInput::Raw(u256) => {
                solidified_wrapped_input.push_str(
                    &encode_hex_reduced(*u256)
                );
            }
            WrappedInput::Opcode(opcode) => {

                let solidified_opcode = opcode.solidify();

                if solidified_opcode.contains(' ') {
                    solidified_wrapped_input.push_str(
                        format!("({solidified_opcode})").as_str()
                    );
                }
                else {
                    solidified_wrapped_input.push_str(
                        solidified_opcode.as_str()
                    );
                }
    
            },
        }

        solidified_wrapped_input
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use ethers::types::U256;

    #[test]
    fn test_solidify_add() {

        // wraps an ADD operation with 2 raw inputs
        let add_operation_wrapped = WrappedOpcode::new(0x01, vec![WrappedInput::Raw(U256::from(1u8)), WrappedInput::Raw(U256::from(2u8))]);
        assert_eq!(add_operation_wrapped.solidify(), "0x01 + 0x02");
        
    }

    #[test]
    fn test_solidify_add_complex() {

        // wraps an ADD operation with 2 raw inputs
        let add_operation_wrapped = WrappedOpcode::new(0x01, vec![WrappedInput::Raw(U256::from(1u8)), WrappedInput::Raw(U256::from(2u8))]);
        let complex_add_operation = WrappedOpcode::new(0x01, vec![WrappedInput::Opcode(add_operation_wrapped), WrappedInput::Raw(U256::from(3u8))]);
        assert_eq!(complex_add_operation.solidify(), "(0x01 + 0x02) + 0x03");
        
    }
}