use bowtruckle::markdown::render_json_markdown;
use serde_json::json;

#[test]
fn renders_full_transaction_json_as_editable_markdown() {
    let tx = json!({
        "transaction_hash": "abc123",
        "transaction": {
            "body": {
                "inputs": [
                    {
                        "index": 5,
                        "transaction_id": "inputtx"
                    }
                ],
                "outputs": [
                    {
                        "address": "addr_test1...",
                        "amount": {
                            "coin": "4000000",
                            "multiasset": {
                                "policy1": {
                                    "535452494b45": "500000000",
                                    "44444f53203230323430383935": "1"
                                }
                            }
                        },
                        "plutus_data": null,
                        "script_ref": null
                    }
                ],
                "fee": "359717",
                "required_signers": ["signer1"]
            },
            "witness_set": {
                "plutus_data": null
            },
            "is_valid": true,
            "auxiliary_data": {
                "metadata": {
                    "674": "{\"map\":[]}"
                }
            }
        }
    });
    let markdown = render_json_markdown(&tx);

    assert!(markdown.starts_with("# Cardano Transaction\n"));
    assert!(markdown.contains("**Transaction Hash:** `abc123`"));
    assert!(markdown.contains("## Transaction"));
    assert!(markdown.contains("### Body"));
    assert!(markdown.contains("#### Inputs"));
    assert!(markdown.contains("- **Transaction Id:** `inputtx`"));
    assert!(markdown.contains("#### Outputs"));
    assert!(markdown.contains("- **Address:** `addr_test1...`"));
    assert!(markdown.contains("- asset `535452494b45` (`STRIKE`) qty `500000000`"));
    assert!(markdown.contains("- asset `44444f53203230323430383935` (`DDOS 20240895`) qty `1`"));
    assert!(markdown.contains("- **Fee:** `359717`"));
    assert!(markdown.contains("- **Plutus Data:** `null`"));
    assert!(markdown.contains("- **Script Ref:** `null`"));
    assert!(markdown.contains("### Witness Set"));
    assert!(markdown.contains("### Auxiliary Data"));
}
