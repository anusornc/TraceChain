mod blockchain;
mod demo;
mod rdf_store;

use crate::blockchain::Blockchain;
use crate::rdf_store::RDFStore;
use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser)]
#[command(name = "TraceChain")]
#[command(about = "Blockchain + RDF Traceability CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a Turtle RDF file as a new block
    AddFile { path: String },

    /// Run a SPARQL query file
    Query { path: String },

    /// Run the built-in UHT manufacturing demo
    Demo,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::AddFile { path } => {
            let mut bc = Blockchain::new();
            let mut store = RDFStore::new();
            let rdf_data = fs::read_to_string(&path)
                .map_err(|e| anyhow::anyhow!("Cannot read RDF file '{}': {}", path, e))?;
            bc.add_block(rdf_data.clone());
            store.add_rdf(&rdf_data);
            println!("Added RDF as block. Blockchain valid? {}", bc.is_valid());
        }
        Commands::Query { path } => {
            let store = RDFStore::new(); // In a real case, load previous blocks' RDF
            let query = fs::read_to_string(&path)
                .map_err(|e| anyhow::anyhow!("Cannot read query file '{}': {}", path, e))?;
            if let oxigraph::sparql::QueryResults::Solutions(solutions) = store.query(&query) {
                for solution in solutions {
                    match solution {
                        Ok(s) => println!("{:?}", s),
                        Err(e) => eprintln!("Error resolving solution: {}", e),
                    }
                }
            }
        }
        Commands::Demo => {
            demo::run_demo()?;
        }
    }

    Ok(())
}
