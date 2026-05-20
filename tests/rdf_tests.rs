use uht_trace_blockchain::rdf_store::RDFStore;
use oxigraph::sparql::QueryResults;

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

    if let QueryResults::Solutions(solutions) = store.query(query) {
        let results: Vec<_> = solutions.collect();
        assert_eq!(results.len(), 1, "Should find exactly one milk batch");
    } else {
        panic!("SPARQL query failed");
    }
}

#[test]
#[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
fn test_add_rdf_malformed_syntax() {
    let mut store = RDFStore::new();
    // This is deliberately malformed Turtle syntax
    let malformed_data = "THIS IS NOT VALID TURTLE SYNTAX";

    // add_rdf uses unwrap(), so this should panic
    store.add_rdf(malformed_data);
}
