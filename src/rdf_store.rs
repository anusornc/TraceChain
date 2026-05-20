use oxigraph::store::Store;
use oxigraph::model::*;
use oxigraph::sparql::{QueryResults, EvaluationError};
use oxigraph::io::GraphFormat;

pub struct RDFStore {
    pub store: Store,
}

impl RDFStore {
    pub fn new() -> Self {
        RDFStore {
            store: Store::new().unwrap(),
        }
    }

    pub fn add_rdf(&mut self, rdf_data: &str) {
        self.store
            .load_graph(
                rdf_data.as_bytes(),
                GraphFormat::Turtle,
                &GraphName::DefaultGraph,
                None,
            )
            .unwrap();
    }

    pub fn query(&self, sparql: &str) -> Result<QueryResults, EvaluationError> {
        self.store.query(sparql)
    }
}
