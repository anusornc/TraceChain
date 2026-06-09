use oxigraph::io::GraphFormat;
use oxigraph::model::*;
use oxigraph::sparql::QueryResults;
use oxigraph::store::Store;

pub struct RDFStore {
    pub store: Store,
}

impl Default for RDFStore {
    fn default() -> Self {
        Self::new()
    }
}

impl RDFStore {
    pub fn new() -> Self {
        RDFStore {
            store: Store::new().unwrap(),
        }
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

    pub fn query(&self, sparql: &str) -> Result<QueryResults, oxigraph::sparql::EvaluationError> {
        self.store.query(sparql)
    }
}
