//! KernelClaw Zero-Dep - SHA256 implementation
//! Pure Rust SHA256 - replaces sha2 crate
//! Based on RFC 6234 and FIPS 180-4

/// SHA256 output size in bytes
pub const SHA256_SIZE: usize = 32;

/// SHA256 context
#[derive(Clone)]
pub struct Sha256 {
    state: [u32; 8],
    buf: [u8; 64],
    len: usize,
    total: u64,
}

impl Sha256 {
    /// Create new hasher
    pub fn new() -> Self {
        Sha256 {
            state: [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
                   0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19],
            buf: [0; 64],
            len: 0,
            total: 0,
        }
    }
    
    /// Update with input bytes
    pub fn update(&mut self, data: &[u8]) {
        for &b in data {
            self.buf[self.len] = b;
            self.len += 1;
            self.total += 1;
            if self.len == 64 {
                self.compress();
                self.len = 0;
            }
        }
    }
    
    /// Get final hash
    pub fn finalize(mut self) -> [u8; SHA256_SIZE] {
        // Pad
        let bitlen = self.total * 8;
        self.buf[self.len] = 0x80;
        self.len += 1;
        
        if self.len > 56 {
            while self.len < 64 {
                self.buf[self.len] = 0;
                self.len += 1;
            }
            self.compress();
            self.len = 0;
        }
        
        while self.len < 56 {
            self.buf[self.len] = 0;
            self.len += 1;
        }
        
        // Bit length as big-endian 64-bit
        let bit_le = bitlen as u64;
        for i in 0..8 {
            self.buf[55 - i] = (bit_le >> (i * 8)) as u8;
        }
        
        self.compress();
        
        // Output as big-endian
        let mut out = [0u8; SHA256_SIZE];
        for i in 0..8 {
            out[i * 4] = (self.state[i] >> 24) as u8;
            out[i * 4 + 1] = (self.state[i] >> 16) as u8;
            out[i * 4 + 2] = (self.state[i] >> 8) as u8;
            out[i * 4 + 3] = self.state[i] as u8;
        }
        out
    }
    
    /// Compress block
    fn compress(&mut self) {
        let mut w = [0u32; 64];
        
        // Prepare message schedule
        for i in 0..16 {
            w[i] = ((self.buf[i * 4] as u32) << 24)
                | ((self.buf[i * 4 + 1] as u32) << 16)
                | ((self.buf[i * 4 + 2] as u32) << 8)
                | (self.buf[i * 4 + 3] as u32);
        }
        
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        
        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];
        
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let k = K[i];
            let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(k).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        
        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
    }
}

impl Default for Sha256 {
    fn default() -> Self {
        Self::new()
    }
}

/// SHA256 constants K
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
    0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
    0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
    0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
    0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
    0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f, 0xc67178f2,
];

/// Compute SHA256 hash
pub fn hash(data: &[u8]) -> [u8; SHA256_SIZE] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize()
}

/// Compute SHA256 and return as hex string
pub fn hex_hash(data: &[u8]) -> String {
    let h = hash(data);
    h.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Compute SHA256 for receipt (convenience)
pub fn receipt_hash(id: &str, timestamp: i64, content: &str) -> String {
    let data = format!("{}:{}:{}", id, timestamp, content);
    hex_hash(data.as_bytes())
}