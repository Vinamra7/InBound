use inscope::domain::edge::EdgeInfo;
use inscope::domain::risk::RiskDecision;
use inscope::domain::risk::gate;

fn edge(ip: &str, country: Option<&str>, asn: Option<u32>) -> EdgeInfo {
    EdgeInfo {
        ip: ip.to_string(),
        country: country.map(|c| c.to_string()),
        asn,
    }
}

#[test]
fn tor_detected_via_country_header() {
    assert!(edge("9.9.9.9", Some("T1"), None).is_tor());
    assert!(!edge("49.36.x.x", Some("IN"), None).is_tor());
    assert!(!edge("49.36.x.x", None, None).is_tor()); // missing header is not Tor
}

#[test]
fn tor_is_blocked_before_anything_else() {
    // Tor + datacenter ASN both present — Block wins
    assert_eq!(
        gate(&edge("9.9.9.9", Some("T1"), Some(16509))),
        RiskDecision::Block
    );
    assert_eq!(
        gate(&edge("9.9.9.9", Some("T1"), None)),
        RiskDecision::Block
    );
}

#[test]
fn datacenter_asn_is_restricted() {
    assert_eq!(
        gate(&edge("3.0.0.1", Some("US"), Some(16509))), // AWS
        RiskDecision::Restricted
    );
    assert_eq!(
        gate(&edge("34.x.x.x", Some("US"), Some(396982))), // Google Cloud
        RiskDecision::Restricted
    );
}

#[test]
fn ordinary_users_are_allowed() {
    assert_eq!(
        gate(&edge("49.36.x.x", Some("IN"), Some(55836))),
        RiskDecision::Allow
    ); // Jio
    assert_eq!(
        gate(&edge("1.2.3.4", Some("DE"), None)),
        RiskDecision::Allow
    ); // asn unseen — not on the list
    assert_eq!(gate(&edge("1.2.3.4", None, None)), RiskDecision::Allow); // nothing known at all
}
