mod gev_rust;
use gev_rust::{on_deploy, on_prove, on_verify};

use clap::Parser;
use gev_core::{GevulotAction, GevulotAlg};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author = "Gevulot Team", version, about, long_about = None)]
pub struct ArgConfiguration {
    /// deploy | prove | verify
    #[clap(value_parser)]
    pub action: GevulotAction,
    /// Prove: algorithm = [ filecoin | groth16 | marlin ]
    #[clap(short, long, value_parser)]
    pub algorithm: Option<GevulotAlg>,
    /// Deploy: for Groth16 & Marlin deployments, this is a .r1cs file.
    #[clap(short, long, value_parser)]
    pub circuit_file: Option<PathBuf>,
    /// Prove: fhen requesting a proof, the id is a UID string
    #[clap(short, long, value_parser)]
    pub id: Option<String>,
    /// Prove: this is the proof file to be written
    /// Verify: this is the proof file to be read in
    #[clap(short, long, value_parser, verbatim_doc_comment)]
    pub proof_file: Option<PathBuf>,
    /// Prove: This is the witness file generated by snarkjs, extension .wtns
    #[clap(short, long, value_parser)]
    pub witness_file: Option<PathBuf>,
    /// Verify: the public inputs in json format; e.g., inputs.json
    #[clap(short, long, value_parser)]
    pub user_inputs_file: Option<PathBuf>,
}

fn main() {
    let arg_conf = ArgConfiguration::parse();

    match arg_conf.action {
        GevulotAction::Deploy => {
            println!("do deploy");
            let program_id = on_deploy(arg_conf.circuit_file.expect("No circuit file passed in"));
            println!("deploy returned: {:?}", program_id);
        }
        GevulotAction::Prove => {
            let program_id = match arg_conf.id {
                Some(program_id) => program_id,
                None => on_deploy(
                    arg_conf
                        .circuit_file
                        .expect("No program id nor circuit file passed in"),
                )
                .expect("circuit deployment failed"),
            };
            println!("do prove");
            _ = on_prove(
                arg_conf
                    .algorithm
                    .expect("No algorithm passed in for the prover."),
                &program_id,
                &arg_conf.witness_file,
                arg_conf
                    .proof_file
                    .expect("No proof file has been passed in for the prover."),
            );
        }
        GevulotAction::Verify => {
            println!("do verify");
            _ = on_verify(
                arg_conf
                    .proof_file
                    .expect("No proof has been passed in for verify."),
                &arg_conf.user_inputs_file,
            );
        }
    }
}