use clap::{AppSettings, Parser};
use derive_builder::Builder;
use heimdall_common::{
    ether::{bytecode::get_bytecode_from_target, evm::core::opcodes::Opcode},
    utils::{
        io::logging::{set_logger_env, Logger},
        strings::{decode_hex, encode_hex},
    },
};

#[derive(Debug, Clone, Parser, Builder)]
#[clap(about = "Disassemble EVM bytecode to Assembly",
       after_help = "For more information, read the wiki: https://jbecker.dev/r/heimdall-rs/wiki",
       global_setting = AppSettings::DeriveDisplayOrder,
       override_usage = "heimdall disassemble <TARGET> [OPTIONS]")]
pub struct DisassemblerArgs {
    /// The target to disassemble, either a file, bytecode, contract address, or ENS name.
    #[clap(required = true)]
    pub target: String,

    /// Set the output verbosity level, 1 - 5.
    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    /// The RPC provider to use for fetching target bytecode.
    #[clap(long = "rpc-url", short, default_value = "", hide_default_value = true)]
    pub rpc_url: String,

    /// Whether to use base-10 for the program counter.
    #[clap(long = "decimal-counter", short = 'd')]
    pub decimal_counter: bool,

    /// Name of the output file.
    #[clap(long, short, default_value = "", hide_default_value = true)]
    pub name: String,

    /// The output directory to write the output to or 'print' to print to the console
    #[clap(long = "output", short = 'o', default_value = "output", hide_default_value = true)]
    pub output: String,
}

impl DisassemblerArgsBuilder {
    pub fn new() -> Self {
        Self {
            target: Some(String::new()),
            verbose: Some(clap_verbosity_flag::Verbosity::new(0, 1)),
            rpc_url: Some(String::new()),
            decimal_counter: Some(false),
            name: Some(String::new()),
            output: Some(String::new()),
        }
    }
}

/// Disassemble the given target's bytecode to assembly.
pub async fn disassemble(args: DisassemblerArgs) -> Result<String, Box<dyn std::error::Error>> {
    use std::time::Instant;
    let now = Instant::now();

    set_logger_env(&args.verbose);

    // get a new logger
    let (logger, _) = Logger::new(match args.verbose.log_level() {
        Some(level) => level.as_str(),
        None => "SILENT",
    });

    let contract_bytecode = get_bytecode_from_target(&args.target, &args.rpc_url).await?;

    let mut program_counter = 0;
    let mut output: String = String::new();

    // Iterate over the bytecode, disassembling each instruction.
    let byte_array = decode_hex(&contract_bytecode.replacen("0x", "", 1))?;
    while program_counter < byte_array.len() {
        let operation = Opcode::new(byte_array[program_counter]);
        let mut pushed_bytes: String = String::new();

        if operation.name.contains("PUSH") {
            let byte_count_to_push: u8 = operation.name.strip_prefix("PUSH").unwrap().parse()?;

            pushed_bytes = match byte_array
                .get(program_counter + 1..program_counter + 1 + byte_count_to_push as usize)
            {
                Some(bytes) => encode_hex(bytes.to_vec()),
                None => break,
            };
            program_counter += byte_count_to_push as usize;
        }

        output.push_str(
            format!(
                "{} {} {}\n",
                if args.decimal_counter {
                    program_counter.to_string()
                } else {
                    format!("{:06x}", program_counter)
                },
                operation.name,
                pushed_bytes
            )
            .as_str(),
        );
        program_counter += 1;
    }

    logger.info(&format!("disassembled {program_counter} bytes successfully."));
    logger.debug(&format!("disassembly completed in {} ms.", now.elapsed().as_millis()));

    Ok(output)
}
