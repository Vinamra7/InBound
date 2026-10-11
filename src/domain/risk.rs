use crate::domain::edge::EdgeInfo;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RiskDecision {
    Block,      // never serve (Tor)
    Restricted, // serve with the reduced quota
    Allow,
}

pub const DATA_CENTER_ASNS: &[u32] = &[
    16509,  // Amazon AWS
    396982, // Google Cloud
    14061,  // DigitalOcean
    24940,  // Hetzner
    16276,  // OVH
];

pub fn gate(edge: &EdgeInfo) -> RiskDecision {
    if edge.is_tor() {
        return RiskDecision::Block;
    }
    if let Some(asn) = edge.asn {
        if DATA_CENTER_ASNS.contains(&asn) {
            return RiskDecision::Restricted;
        }
    }
    RiskDecision::Allow
}
