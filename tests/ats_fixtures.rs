// SPDX-License-Identifier: Unlicense
// Unlicense — public domain — cochranblock.org
//! Integration tests for ats_fixtures (consumer-facing surface).
//!
//! The inline `#[cfg(test)] mod tests` in `src/ats_fixtures.rs` exercises
//! private invariants. These tests exercise the crate boundary: every
//! contract a downstream user of `exopack::ats_fixtures::{render,
//! expected_keys, AtsVendor, FixtureOpts}` is allowed to rely on.

#![cfg(feature = "ats_fixtures")]

use exopack::ats_fixtures::{expected_keys, render, AtsVendor, FixtureOpts};

const ALL_VENDORS: &[AtsVendor] = &[
    AtsVendor::Greenhouse,
    AtsVendor::Lever,
    AtsVendor::Workday,
    AtsVendor::Icims,
    AtsVendor::Ashby,
];

#[test]
fn vendor_label_is_stable_and_unique() {
    let labels: Vec<&'static str> = ALL_VENDORS.iter().map(|v| v.label()).collect();
    let mut sorted = labels.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), labels.len(), "vendor labels must be unique");
    // Lowercase + no whitespace — downstream code embeds these in filenames.
    for l in &labels {
        assert_eq!(*l, l.to_lowercase(), "label {l} must be lowercase");
        assert!(!l.contains(char::is_whitespace), "label {l} has whitespace");
    }
}

#[test]
fn expected_keys_returns_canonical_classifier_keys() {
    // Across all vendors, every (id, key) pair the consumer is told to
    // assert on must use a key from the canonical classifier vocab.
    // If a renderer drifts to a new key, downstream classifiers break
    // silently — pin the vocab here.
    // Canonical key set lifted from SHARED_FIELDS + per-vendor overrides
    // in expected_keys (src/ats_fixtures.rs). Lever/Ashby collapse first
    // name into full_name; Ashby remaps `street1` → legacy `address`
    // for its single-line "Where are you based?" field. Empty string is
    // the deliberate out-of-vocab decoy ("salary") — classifiers must
    // produce no match.
    let canonical: &[&str] = &[
        "first_name",
        "last_name",
        "full_name",
        "email",
        "phone",
        "street1",
        "address",
        "city",
        "postal_code",
        "linkedin",
        "github",
        "work_authorization",
        "freetext",
        "",
    ];
    for v in ALL_VENDORS {
        for (id, key) in expected_keys(*v, &FixtureOpts::default()) {
            assert!(
                canonical.contains(&key),
                "{v:?}: key {key} (id={id}) not in canonical vocab"
            );
        }
    }
}

#[test]
fn every_expected_id_appears_in_rendered_html() {
    // The consumer contract: id from expected_keys must be locatable in
    // the rendered DOM. Otherwise the test bench is asking consumers to
    // hunt for fields that don't exist.
    for v in ALL_VENDORS {
        let opts = FixtureOpts::default();
        let html = render(*v, &opts);
        for (id, key) in expected_keys(*v, &opts) {
            assert!(
                html.contains(&id),
                "{v:?}: expected id {id} (key={key}) not found in rendered HTML"
            );
        }
    }
}

#[test]
fn dynamic_ids_changes_html_but_not_key_count() {
    // FixtureOpts::dynamic_ids must change the rendered IDs (Workday-
    // style noisy IDs), but the number of expected fields stays the
    // same — opts changes form, not schema.
    for v in ALL_VENDORS {
        let stable = render(*v, &FixtureOpts::default());
        let noisy = render(
            *v,
            &FixtureOpts {
                dynamic_ids: true,
                ..FixtureOpts::default()
            },
        );
        let key_count_stable = expected_keys(*v, &FixtureOpts::default()).len();
        let key_count_noisy = expected_keys(
            *v,
            &FixtureOpts {
                dynamic_ids: true,
                ..FixtureOpts::default()
            },
        )
        .len();
        assert_eq!(
            key_count_stable, key_count_noisy,
            "{v:?}: dynamic_ids must not change field count"
        );
        // For vendors that actually use IDs in markup the two strings
        // should differ; Lever uses aria-label-only so its DOM may not
        // change with dynamic_ids — that's fine, only assert difference
        // where IDs appear at all.
        if stable.contains("id=\"") {
            assert_ne!(stable, noisy, "{v:?}: dynamic_ids did not alter markup");
        }
    }
}

#[test]
fn late_hydration_emits_a_script_tag() {
    // Workday-style late hydration is implemented by deferring DOM
    // construction to a <script> block. With the option on, the
    // rendered HTML must contain a <script> tag. With it off, it
    // doesn't necessarily — but the option being on must change
    // something observable.
    for v in ALL_VENDORS {
        let off = render(*v, &FixtureOpts::default());
        let on = render(
            *v,
            &FixtureOpts {
                late_hydration_ms: Some(120),
                ..FixtureOpts::default()
            },
        );
        assert!(
            on.contains("<script"),
            "{v:?}: late_hydration must emit a <script>"
        );
        assert_ne!(off, on, "{v:?}: late_hydration changed nothing");
    }
}

#[test]
fn rebuild_on_focus_is_observable_in_markup() {
    // The anti-bot rebuild-on-focus shim must be visible in the
    // rendered HTML (it ships as inline JS that listens to focus and
    // mutates the DOM). Without the option, no such listener is wired.
    for v in ALL_VENDORS {
        let off = render(*v, &FixtureOpts::default());
        let on = render(
            *v,
            &FixtureOpts {
                rebuild_on_focus: true,
                ..FixtureOpts::default()
            },
        );
        assert_ne!(off, on, "{v:?}: rebuild_on_focus changed nothing");
        assert!(
            on.contains("focus"),
            "{v:?}: rebuild_on_focus must wire a focus handler"
        );
    }
}

#[test]
fn renders_are_deterministic() {
    // Same vendor + same opts → same bytes. Required because consumers
    // snapshot the HTML for regression. Any randomness would break that.
    for v in ALL_VENDORS {
        let a = render(*v, &FixtureOpts::default());
        let b = render(*v, &FixtureOpts::default());
        assert_eq!(a, b, "{v:?}: render is non-deterministic");
    }
}

#[test]
fn lever_and_ashby_collapse_name_to_full_name() {
    // Per module docs: Lever and Ashby render slot "first" as a
    // combined Name field that classifies as "full_name", not
    // "first_name". This is a deliberate vendor quirk — pin it so a
    // refactor doesn't silently regress.
    let opts = FixtureOpts::default();
    for v in [AtsVendor::Lever, AtsVendor::Ashby] {
        let keys: Vec<&'static str> = expected_keys(v, &opts).iter().map(|(_, k)| *k).collect();
        assert!(
            keys.contains(&"full_name"),
            "{v:?}: must expose full_name"
        );
        assert!(
            !keys.contains(&"first_name"),
            "{v:?}: must NOT expose first_name (collapses to full_name)"
        );
    }
    // Inverse: Greenhouse / Workday / iCIMS must keep the split.
    for v in [AtsVendor::Greenhouse, AtsVendor::Workday, AtsVendor::Icims] {
        let keys: Vec<&'static str> = expected_keys(v, &opts).iter().map(|(_, k)| *k).collect();
        assert!(
            keys.contains(&"first_name") && keys.contains(&"last_name"),
            "{v:?}: must keep first/last split"
        );
    }
}
