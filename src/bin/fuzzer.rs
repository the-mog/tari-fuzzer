use clap::AppSettings;
use fuzz_targets::check_target;
use log::{info, trace, warn, LevelFilter};
use once_cell::sync::Lazy;
use std::process::exit;
use structopt::StructOpt;
use tarifuzzer::{corpus, runner, Fuzzer, Result, TariError};

// Constants for our defaults
const ENGINE_DEFAULT: &str = "Libfuzzer";
const CORPUS_ITEMS: usize = 25;
static CORPUS_ITEMS_STR: Lazy<String> = Lazy::new(|| CORPUS_ITEMS.to_string());

#[derive(StructOpt, Debug)]
#[structopt(name = "tari-fuzzer", global_settings = &
[AppSettings::DisableHelpSubcommand, AppSettings::VersionlessSubcommands])]

struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "fuzz", about = "Fuzz specified target")]
    Fuzz {
        #[structopt(name = "TARGET", help = "A string fuzz target", required = true, short = "t")]
        target: String,
        /// Engine to use (use `engines` to list them)
        #[structopt(name="ENGINE", help = "A fuzzing engine to use",
        case_insensitive=true, short = "e", default_value=&ENGINE_DEFAULT)]
        engine: Fuzzer,
        #[structopt(
        name="SeedsCount",
        help = "Number of seeds to be generated for your corpus.",
        short = "n",
        default_value = &CORPUS_ITEMS_STR,
        )]
        seeds: usize,
    },
    #[structopt(name = "list-targets", about = "Get the list of available targets")]
    ListTargets {},
    #[structopt(name = "list-engines", about = "Get the list of available engines")]
    ListEngines {},
    #[structopt(name = "gen-corpus", about = "Generate corpus using proptest crate")]
    GenCorpus {
        #[structopt(name = "TARGET", help = "A string fuzz target", required = true, short = "t")]
        target: String,
        /// Number of items to generate in the corpus
        #[structopt(
        name="SeedsCount",
        help = "Number of seeds to be generated for your corpus.",
        short = "n",
        default_value = &CORPUS_ITEMS_STR,
        )]
        seeds: usize,
        /// Engine to use (use `engines` to list them)
        #[structopt(name="ENGINE", help = "A fuzzing engine to use", short = "e", default_value=&ENGINE_DEFAULT)]
        engine: Fuzzer,
    },
}

fn main() {
    env_logger::builder().filter_level(LevelFilter::Trace).init();
    let opt = Opt::from_args();
    if let Err(TariError::UnexpectedError(_0)) = run(opt) {
        warn!("Exiting...");
        exit(1);
    }
}

fn run(opt: Opt) -> Result<()> {
    info!("Tari-fuzzer {}", env!("CARGO_PKG_VERSION"));
    // info!("Got it Yat ?\u{1F982}\u{1F596}\u{270C}\u{FE0F}\u{2764}\u{FE0F}\u{2604}\u{FE0F}\u{1F37A}\u{1F37A}");
    info!("https://www.tari.com");
    match opt.command {
        Command::Fuzz { target, engine, seeds } => {
            info!(
                "Fuzzing job started for target: {:?} using engine {:?}",
                &target, &engine
            );
            check_target(&target);
            match engine {
                Fuzzer::Afl => runner::run_afl(&target, engine, seeds).is_err(),
                Fuzzer::Honggfuzz => runner::run_honggfuzz(&target, engine, seeds).is_err(),
                _ => runner::run_libfuzzer(&target, engine, seeds).is_err(),
            };
        },
        Command::GenCorpus { target, engine, seeds } => {
            trace!(
                "Generating {:?} seeds for target: {:?} using engine {:?}",
                &seeds,
                &target,
                &engine
            );
            if &seeds == &0 {
                info!("Seed count is set to 0, no seeds will be created!")
            };
            check_target(&target);
            let _corpus_dir = corpus::gen_corpus(&target, engine, seeds);
            //info!("{:?}", &corpus_dir);
            info!("Corpus generation completed!");
            //if corpus_dir.is_err() {
              //  info!("Corpus directory exists, NOT generating any seeds");
                //info!("Corpus generation completed!")
            //} else {
              //  info!("Corpus generation completed!");
           // }
        },
        Command::ListEngines {} => {
            info!(
                "Current supported engines are:\n\
                \u{1F004}{:?} => https://github.com/rust-fuzz/afl.rs \n\
                \u{1F0CF}{:?} => https://github.com/rust-fuzz/libfuzzer\n\
                \u{1F335}{:?} => https://honggfuzz.dev/ \n",
                Fuzzer::Afl,
                Fuzzer::Libfuzzer,
                Fuzzer::Honggfuzz
            );
        },
        Command::ListTargets {} => {
            fuzz_targets::list_targets();
        },
    };
    Ok(())
}
