use oxigraph::io::GraphFormat;
use oxigraph::model::*;
use oxigraph::sparql::QueryResults;
use oxigraph::store::Store;

pub struct RDFStore {
    pub store: Store,
}

impl RDFStore {
    pub fn new() -> anyhow::Result<Self> {
        Ok(RDFStore {
            store: Store::new()?,
        })
    }

    pub fn add_rdf(&mut self, rdf_data: &str) -> anyhow::Result<()> {
        self.store.load_graph(
            rdf_data.as_bytes(),
            GraphFormat::Turtle,
            &GraphName::DefaultGraph,
            None,
        )?;
        Ok(())
    }

    pub fn query(&self, sparql: &str) -> QueryResults {
        self.store.query(sparql).unwrap()
    }
}
