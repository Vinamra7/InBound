use crate::domain::edge::EdgeInfo;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RiskDecision {
    Block,      // never serve (Tor)
    Restricted, // serve with the reduced quota
    Allow,
}

pub const DATA_CENTER_ASNS: &[u32] = &[
    16509,  // Amazon AWS
    14618,  // Amazon AWS (us-east)
    396982, // Google Cloud
    14061,  // DigitalOcean
    24940,  // Hetzner
    16276,  // OVH
    8075,   // Microsoft Azure
    63949,  // Linode / Akamai
    20473,  // Vultr
    31898,  // Oracle Cloud
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
