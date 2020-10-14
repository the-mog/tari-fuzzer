use crate::engines::Fuzzer;
use crate::types;
use anyhow::{Context, Result};
use fuzz_targets::common::SeedGen;
use log::{info, trace};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

/// Create corpus dir for fuzz target
pub fn create_corpus_dir(base: impl AsRef<Path>, target: &str) -> Result<PathBuf> {
    trace!("Creating corpus directory to store our seeds");
    let base = base.as_ref();
    let corpus_dir = base.join(&format!("corpus-{}", target));
    fs::create_dir_all(&corpus_dir).context(format!(
        "unable to create corpus dir for {}{}",
        base.display(),
        target
    ))?;
    info!("Corpus directory created: {:?}", &corpus_dir);
    Ok(corpus_dir)
}

pub fn gen_corpus(target: &str, fuzzer: Fuzzer, num_items: usize) -> Result<PathBuf> {
    trace!("Generating corpus");
    let corpus_dir = create_corpus_dir(fuzzer.directory(), target)?;
    let mut gen = SeedGen::new();
    let mut idx: usize = 0;
    match target {
        //MMR
        "mmr_push_bytes" => {
            //let seedfn = corpus_name!("fuzz_mmr_push_bytes_seeds");
            //let gen = seedfn(&mut gen);
            while idx < num_items {
                let seeds = types::gen_vec_u8(&mut gen);
                let name = hex::encode(&seeds);
                let filename = corpus_dir.join(name);
                let mut f = fs::File::create(&filename)
                    .with_context(|| format!("Failed to create file: {:?}", filename))?;
                f.write_all(&seeds)
                    .with_context(|| format!("Failed to write to file: {:?}", filename))?;
                idx += 1;
            }
            Ok(corpus_dir)
        }
        //TO_HEX
        "util_to_hex" => {
            while idx < num_items {
                let seeds = types::gen_str(&mut gen);
                let name = hex::encode(&seeds);
                let filename = corpus_dir.join(name);
                let mut f = fs::File::create(&filename)
                    .with_context(|| format!("Failed to create file: {:?}", filename))?;
                f.write_all(&seeds.as_bytes())
                    .with_context(|| format!("Failed to write to file: {:?}", filename))?;
                idx += 1;
            }
            Ok(corpus_dir)
        }
        "util_bytes_to_bits" => {
            while idx < num_items {
                let seeds = types::gen_u8(&mut gen);
                let seeds = seeds.to_string();
                let name = hex::encode(&seeds);
                let filename = corpus_dir.join(name);
                let mut f = fs::File::create(&filename)
                    .with_context(|| format!("Failed to create file: {:?}", filename))?;
                f.write_all(&seeds.as_bytes())
                    .with_context(|| format!("Failed to write to file: {:?}", filename))?;
                idx += 1;
            }
            Ok(corpus_dir)
        }
        "util_from_hex" => {
            while idx < num_items {
                let seeds = types::gen_str(&mut gen);
                let name = hex::encode(&seeds);
                let filename = corpus_dir.join(name);
                let mut f = fs::File::create(&filename)
                    .with_context(|| format!("Failed to create file: {:?}", filename))?;
                f.write_all(&seeds.as_bytes())
                    .with_context(|| format!("Failed to write to file: {:?}", filename))?;
                idx += 1;
            }
            Ok(corpus_dir)
        }
        "util_to_hex_multiple" => {
            info!("Not generating seeds for this target!");
            Ok(corpus_dir)
        }
        "crypto_opcodes_to_hash" => {
            while idx < num_items {
                let seeds = types::gen_u8(&mut gen);
                let seeds_payload = &[seeds];
                let seeds_str = seeds.to_string();
                let name = hex::encode(&seeds_str);
                let filename = corpus_dir.join(name);
                let mut f = fs::File::create(&filename)
                    .with_context(|| format!("Failed to create file: {:?}", filename))?;
                f.write_all(seeds_payload)
                    .with_context(|| format!("Failed to write to file: {:?}", filename))?;
                idx += 1;
            }
            Ok(corpus_dir)
        }
        "crypto_opcodes_read_nexth" => {
            while idx < num_items {
                let seeds = types::gen_u8(&mut gen);
                let seeds_payload = &[seeds];
                let seeds_str = seeds.to_string();
                let name = hex::encode(&seeds_str);
                let filename = corpus_dir.join(name);
                let mut f = fs::File::create(&filename)
                    .with_context(|| format!("Failed to create file: {:?}", filename))?;
                f.write_all(seeds_payload)
                    .with_context(|| format!("Failed to write to file: {:?}", filename))?;
                idx += 1;
            }
            Ok(corpus_dir)
        },
        "core_trx_create_coinbase" => {
            info!("Not generating seeds for this target!");
            Ok(corpus_dir)
        },
        "core_trx_with_maturity" => {
            info!("Not generating seeds for this target!");
            Ok(corpus_dir)
        },

        
        _ => panic!("Unable to generate fuzzing seeds for {:?}", &target),
    }
}
