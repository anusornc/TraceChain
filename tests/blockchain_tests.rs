use uht_trace_blockchain::blockchain::Blockchain;

#[test]
fn test_blockchain_add_and_validate() {
    let mut bc = Blockchain::new();
    bc.add_block("test data".into());
    bc.add_block("more test data".into());

    assert!(
        bc.is_valid(),
        "Blockchain should be valid after adding blocks"
    );
}

#[test]
fn test_blockchain_detect_tampering() {
    let mut bc = Blockchain::new();
    bc.add_block("original".into());

    // Tamper with a block
    bc.chain[0].data = "tampered".into();

    assert!(
        !bc.is_valid(),
        "Blockchain should be invalid after tampering"
    );
}

#[test]
fn test_blockchain_detect_intermediate_tampering() {
    let mut bc = Blockchain::new();
    bc.add_block("block 1".into());
    bc.add_block("block 2".into());

    // Tamper with intermediate block (index 1)
    bc.chain[1].data = "tampered block 1".into();

    assert!(!bc.is_valid(), "Blockchain should be invalid after tampering an intermediate block");
}

#[test]
fn test_blockchain_detect_intermediate_tampering_hash_update() {
    let mut bc = Blockchain::new();
    bc.add_block("block 1".into());
    bc.add_block("block 2".into());

    // Tamper with intermediate block (index 1)
    bc.chain[1].data = "tampered block 1".into();
    // Update its hash to try to hide the tampering
    bc.chain[1].hash = bc.chain[1].calculate_hash();

    assert!(!bc.is_valid(), "Blockchain should be invalid even if tampered block's hash is recalculated, because the next block's previous_hash won't match");
}

#[test]
fn test_hash_collision_prevention() {
    use uht_trace_blockchain::blockchain::Block;

    // Suppose a malicious user tries to shift data between fields to get the same hash
    let mut block1 = Block::new(1, "some_data".into(), "prev_hash".into());
    block1.timestamp = "2023-10-27T10:00:00Z_ext".into(); // Timestamp includes the shift

    let mut block2 = Block::new(1, "_extsome_data".into(), "prev_hash".into());
    block2.timestamp = "2023-10-27T10:00:00Z".into();

    let hash1 = block1.calculate_hash();
    let hash2 = block2.calculate_hash();

    // The hashes must be different due to the null byte delimiter
    assert_ne!(hash1, hash2, "Hashes should not collide even if data is shifted between adjacent fields");
}
