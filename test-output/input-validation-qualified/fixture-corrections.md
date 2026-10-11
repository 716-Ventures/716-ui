# Fixture corrections

The initial fixture selected field, document and error text together; their shared PRIMARY ownership cleared the document selection. The corrected fixture checks each selected editor in turn. A later `has_focus` assertion failed in headless Weston despite exact root focus remaining the TextView (`window active=false visible=true, root focus=Some("textview")`). The final fixture checks the exact GTK logical focus target, matching the retained-button test; physical input remains unqualified by this gallery. No production feedback code changed during those corrections.
