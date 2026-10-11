#[derive(Debug, Clone, PartialEq)]
pub struct EdgeInfo {
    pub ip: String, // CF-Connecting-IP — set by Cloudflare's edge from the TCP connection, cannot be spoofed by headers
    pub country: Option<String>, // CF-IPCountry — "IN"; "T1" = Tor exit node
    pub asn: Option<u32>, // autonomous system number of the origin network (request.cf)
    pub tz: Option<String>, // timezone of the edge (request.cf)
    pub lang: Option<String>, // language of the edge (request.cf)
}

impl EdgeInfo {
    pub fn is_tor(&self) -> bool {
        self.country.as_deref() == Some("T1")
    }
}
