use oxigraph::sparql::QueryResults;
use uht_trace_blockchain::rdf_store::RDFStore;

#[test]
fn test_rdf_insertion_and_query() -> anyhow::Result<()> {
    let mut store = RDFStore::new();
    let turtle_data = r#"
        @prefix ex: <http://example.org/> .
        @prefix prov: <http://www.w3.org/ns/prov#> .

        ex:milkBatch1 a ex:Milk ;
            prov:wasAttributedTo ex:FarmerJohn .
    "#;

    store.add_rdf(turtle_data)?;

    let query = r#"
        PREFIX ex: <http://example.org/>
        SELECT ?batch
        WHERE {
            ?batch a ex:Milk .
        }
    "#;

    if let Ok(QueryResults::Solutions(solutions)) = store.query(query) {
        let results: Vec<_> = solutions.collect();
        assert_eq!(results.len(), 1, "Should find exactly one milk batch");
    } else {
        panic!("SPARQL query failed");
    }

    Ok(())
}

#[test]
fn test_invalid_sparql_query_returns_err() {
    let store = RDFStore::new();
    let invalid_query = "SELECT ?x WHERE { ?x a ex:Milk"; // Missing closing brace

    let result = store.query(invalid_query);
    assert!(result.is_err(), "Invalid SPARQL query should return an Err, not panic or succeed");
}
