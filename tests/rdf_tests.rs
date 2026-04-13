use oxigraph::sparql::QueryResults;
use uht_trace_blockchain::rdf_store::RDFStore;

#[test]
fn test_rdf_insertion_and_query() {
    let mut store = RDFStore::new();
    let turtle_data = r#"
        @prefix ex: <http://example.org/> .
        @prefix prov: <http://www.w3.org/ns/prov#> .

        ex:milkBatch1 a ex:Milk ;
            prov:wasAttributedTo ex:FarmerJohn .
    "#;

    store.add_rdf(turtle_data);

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
}

#[test]
fn test_query_invalid_sparql() {
    let store = RDFStore::new();
    let query = "INVALID SPARQL QUERY";
    let result = store.query(query);
    assert!(
        result.is_err(),
        "Invalid SPARQL query should return an error instead of panicking"
    );
}
