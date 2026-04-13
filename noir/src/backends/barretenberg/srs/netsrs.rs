use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, RANGE};
use std::ops::Deref;
use std::time::Duration;

use super::{Srs, G2};

pub struct NetSrs(pub Srs);

impl Deref for NetSrs {
    type Target = Srs;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NetSrs {
    pub fn new(num_points: u32) -> Self {
        NetSrs(Srs {
            num_points,
            g1_data: Self::download_g1_data(num_points),
            g2_data: G2.to_vec(),
        })
    }

    pub fn to_srs(self) -> Srs {
        self.0
    }

    /// Build a blocking HTTP client with a generous timeout.
    ///
    /// The default reqwest blocking client has no overall timeout but uses TCP
    /// keepalive defaults that can trip on CI runners with slow/flaky network
    /// paths to crs.aztec.network. A 5 minute timeout covers legitimate large
    /// SRS downloads (tens of MB) while still failing fast on genuine outages.
    fn http_client() -> Client {
        Client::builder()
            .timeout(Duration::from_secs(300))
            .build()
            .expect("failed to build reqwest client")
    }

    fn download_g1_data(num_points: u32) -> Vec<u8> {
        let g1_end: u32 = num_points * 64 - 1;

        let mut headers = HeaderMap::new();
        headers.insert(RANGE, format!("bytes={}-{}", 0, g1_end).parse().unwrap());

        let response = Self::http_client()
            .get("https://crs.aztec.network/g1.dat")
            .headers(headers)
            .send()
            .expect("failed to request g1 SRS data");

        response.bytes().expect("failed to read g1 SRS body").to_vec()
    }

    #[allow(dead_code)]
    fn download_g2_data() -> Vec<u8> {
        let response = Self::http_client()
            .get("https://crs.aztec.network/g2.dat")
            .send()
            .expect("failed to request g2 SRS data");

        response.bytes().expect("failed to read g2 SRS body").to_vec()
    }
}
