use cargo_metadata::MetadataCommand;
use lazy_static::lazy_static;
use std::path::PathBuf;
use structopt::clap::arg_enum;

lazy_static! {
    static ref WORKSPACE_ROOT: PathBuf = MetadataCommand::new().no_deps().exec().unwrap().workspace_root;
}

arg_enum! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Fuzzer {
        Afl,
        Honggfuzz,
        Libfuzzer,
    }
}

impl Fuzzer {
    /// Get Cargo package name of corresponding fuzzers.
    pub fn package_name(self) -> &'static str {
        match self {
            Fuzzer::Afl => "fuzzer-afl",
            Fuzzer::Honggfuzz => "fuzzer-honggfuzz",
            Fuzzer::Libfuzzer => "fuzzer-libfuzzer",
        }
    }

    /// Get Cargo directory of corresponding fuzzers.
    pub fn directory(self) -> PathBuf {
        WORKSPACE_ROOT.join(self.package_name())
    }
}
