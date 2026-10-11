# Choice accessibility correction

The first complete native gallery passed the four control fixtures but its independent AT-SPI journey found the combobox named only `Second`, instead of its field label. GTK’s nested trigger can replace the author-supplied name with the selected item. Choice now sets a literal `field label: current option` name after mapping and selection changes, removing the stale LabelledBy relation first. The closure owns bounded strings only, so it cannot retain its own control. No selected value or event dispatch changes.

The corrected full gallery passes. Independent LTR and RTL clients find `Validation choice: Second`, explicitly change the native option through a local gallery action, then find `Validation choice: First` with the exact related error preserved. They clear feedback and verify state, relation, sensitivity and unchanged toggle value. The original bounded public-only tree/trace is retained; no private host driver trace is published.
