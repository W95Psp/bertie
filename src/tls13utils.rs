// Import hacspec and all needed definitions.
#[cfg(feature = "evercrypt")]
use evercrypt_cryptolib::*;
#[cfg(not(feature = "evercrypt"))]
use hacspec_cryptolib::*;
use hacspec_lib::*;

use crate::*;

pub type Bytes = ByteSeq;

pub fn empty() -> ByteSeq {
    ByteSeq::new(0)
}

pub fn zeros(u: usize) -> ByteSeq {
    ByteSeq::new(u)
}

pub fn bytes<T: SeqTrait<U8>>(x: &T) -> ByteSeq {
    Seq::from_seq(x)
}

pub fn merge_opts<T>(o1: Option<T>, o2: Option<T>) -> Result<Option<T>, TLSError> {
    match (o1, o2) {
        (None, Some(o)) => Ok(Some(o)),
        (Some(o), None) => Ok(Some(o)),
        (None, None) => Ok(None),
        _ => Err(parse_failed()),
    }
}

bytes!(Random, 32);

// Local error codes
pub type TLSError = u8;
pub const INCORRECT_STATE: TLSError = 128u8;
pub const ZERO_RTT_DISABLED: TLSError = 129u8;
pub const PAYLOAD_TOO_LONG: TLSError = 130u8;
pub const PSK_MODE_MISMATCH: TLSError = 131u8;
pub const NEGOTIATION_MISMATCH: TLSError = 132u8;
pub const PARSE_FAILED: TLSError = 133u8;
pub const INSUFFICIENT_DATA: TLSError = 134u8;
pub const UNSUPPORTED: TLSError = 135u8;
pub const INVALID_COMPRESSION_LIST: TLSError = 136u8;
pub const PROTOCOL_VERSION_ALERT: TLSError = 137u8;
pub const APPLICATION_DATA_INSTEAD_OF_HANDSHAKE: TLSError = 138u8;
pub const MISSING_KEY_SHARE: TLSError = 139u8;
pub const INVALID_SIGNATURE: TLSError = 140u8;
pub const GOT_HANDSHAKE_FAILURE_ALERT: TLSError = 141u8;

pub fn error_string(c: u8) -> String {
    format!("{}", c)
}
/*
pub fn check_eq_size(s1: TLSError, s2: usize) -> Result<()> {
    if s1 == s2 {Ok(())}
    else {Err(parse_failed())}
}*/

pub fn check(b: bool) -> Result<(), TLSError> {
    if b {
        Ok(())
    } else {
        Err(parse_failed())
    }
}

pub fn eq1(b1: U8, b2: U8) -> bool {
    b1.declassify() == b2.declassify()
}
pub fn check_eq1(b1: U8, b2: U8) -> Result<(), TLSError> {
    if eq1(b1, b2) {
        Ok(())
    } else {
        Err(parse_failed())
    }
}

pub fn eq(b1: &ByteSeq, b2: &ByteSeq) -> bool {
    if b1.len() != b2.len() {
        false
    } else {
        for i in 0..b1.len() {
            if !eq1(b1[i], b2[i]) {
                return false;
            };
        }
        true
    }
}

pub fn check_eq(b1: &ByteSeq, b2: &ByteSeq) -> Result<(), TLSError> {
    if b1.len() != b2.len() {
        Err(parse_failed())
    } else {
        for i in 0..b1.len() {
            check_eq1(b1[i], b2[i])?;
        }
        Ok(())
    }
}

pub fn check_mem(b1: &ByteSeq, b2: &ByteSeq) -> Result<(), TLSError> {
    if b2.len() % b1.len() != 0 {
        Err(parse_failed())
    } else {
        for i in 0..(b2.len() / b1.len()) {
            let snip = b2.slice_range(i * b1.len()..(i + 1) * b1.len());
            if eq(b1, &snip) {
                return Ok(());
            }
        }
        Err(parse_failed())
    }
}

pub fn lbytes1(b: &ByteSeq) -> Result<Bytes, TLSError> {
    let len = b.len();
    if len >= 256 {
        Err(PAYLOAD_TOO_LONG)
    } else {
        let mut lenb = Seq::new(1);
        lenb[0] = U8(len as u8);
        Ok(lenb.concat(b))
    }
}

pub fn lbytes2(b: &ByteSeq) -> Result<Bytes, TLSError> {
    let len = b.len();
    if len >= 65536 {
        Err(PAYLOAD_TOO_LONG)
    } else {
        let len: u16 = len as u16;
        let lenb = Seq::from_seq(&U16_to_be_bytes(U16(len)));
        Ok(lenb.concat(b))
    }
}

pub fn lbytes3(b: &ByteSeq) -> Result<Bytes, TLSError> {
    let len = b.len();
    if len >= 16777216 {
        Err(PAYLOAD_TOO_LONG)
    } else {
        let lenb = U32_to_be_bytes(U32(len as u32));
        Ok(lenb.slice_range(1..4).concat(b))
    }
}

pub fn check_lbytes1(b: &ByteSeq) -> Result<usize, TLSError> {
    if b.len() < 1 {
        Err(parse_failed())
    } else {
        let l = (b[0] as U8).declassify() as usize;
        if b.len() - 1 < l {
            Err(parse_failed())
        } else {
            Ok(l)
        }
    }
}

pub fn check_lbytes2(b: &ByteSeq) -> Result<usize, TLSError> {
    if b.len() < 2 {
        Err(parse_failed())
    } else {
        let l0 = (b[0] as U8).declassify() as usize;
        let l1 = (b[1] as U8).declassify() as usize;
        let l = l0 * 256 + l1;
        if b.len() - 2 < l as usize {
            Err(parse_failed())
        } else {
            Ok(l)
        }
    }
}

pub fn check_lbytes3(b: &ByteSeq) -> Result<usize, TLSError> {
    if b.len() < 3 {
        Err(parse_failed())
    } else {
        let l0 = (b[0] as U8).declassify() as usize;
        let l1 = (b[1] as U8).declassify() as usize;
        let l2 = (b[2] as U8).declassify() as usize;
        let l = l0 * 65536 + l1 * 256 + l2;
        if b.len() - 3 < l {
            Err(parse_failed())
        } else {
            Ok(l)
        }
    }
}

pub fn check_lbytes1_full(b: &ByteSeq) -> Result<(), TLSError> {
    if check_lbytes1(b)? + 1 != b.len() {
        Err(parse_failed())
    } else {
        Ok(())
    }
}

pub fn check_lbytes2_full(b: &ByteSeq) -> Result<(), TLSError> {
    if check_lbytes2(b)? + 2 != b.len() {
        Err(parse_failed())
    } else {
        Ok(())
    }
}

pub fn check_lbytes3_full(b: &ByteSeq) -> Result<(), TLSError> {
    if check_lbytes3(b)? + 3 != b.len() {
        Err(parse_failed())
    } else {
        Ok(())
    }
}

// Algorithmns(ha, ae, sa, gn, psk_mode, zero_rtt)
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Algorithms(
    pub HashAlgorithm,
    pub AeadAlgorithm,
    pub SignatureScheme,
    pub KemScheme,
    pub bool,
    pub bool,
);

pub fn hash_alg(algs: &Algorithms) -> HashAlgorithm {
    algs.0
}
pub fn aead_alg(algs: &Algorithms) -> AeadAlgorithm {
    algs.1
}
pub fn sig_alg(algs: &Algorithms) -> SignatureScheme {
    algs.2
}
pub fn kem_alg(algs: &Algorithms) -> KemScheme {
    algs.3
}
pub fn psk_mode(algs: &Algorithms) -> bool {
    algs.4
}
pub fn zero_rtt(algs: &Algorithms) -> bool {
    algs.5
}

// Handshake Data
pub struct HandshakeData(pub Bytes);

pub fn handshake_data(b: Bytes) -> HandshakeData {
    HandshakeData(b)
}
pub fn handshake_data_bytes(hd: &HandshakeData) -> Bytes {
    hd.0.clone()
}

pub fn handshake_data_len(p: &HandshakeData) -> usize {
    p.0.len()
}

pub fn handshake_concat(msg1: HandshakeData, msg2: &HandshakeData) -> HandshakeData {
    let HandshakeData(m1) = msg1;
    let HandshakeData(m2) = msg2;
    HandshakeData(m1.concat(m2))
}

// Application Data
#[derive(PartialEq)]
pub struct AppData(Bytes);

pub fn app_data(b: Bytes) -> AppData {
    AppData(b)
}
pub fn app_data_bytes(a: AppData) -> Bytes {
    a.0
}

pub struct ServerDB(
    pub Bytes,
    pub Bytes,
    pub SignatureKey,
    pub Option<(Bytes, PSK)>,
);

pub fn lookup_db(
    algs: Algorithms,
    db: &ServerDB,
    sni: &Bytes,
    tkt: &Option<Bytes>,
) -> Result<(Bytes, SignatureKey, Option<PSK>), TLSError> {
    let ServerDB(server_name, cert, sk, psk_opt) = db;
    if eq(sni, &empty()) || eq(sni, server_name) {
        match (psk_mode(&algs), tkt, psk_opt) {
            (true, Some(ctkt), Some((stkt, psk))) => {
                check_eq(ctkt, stkt)?;
                Ok((cert.clone(), sk.clone(), Some(psk.clone())))
            }
            (false, _, _) => Ok((cert.clone(), sk.clone(), None)),
            _ => Err(PSK_MODE_MISMATCH),
        }
    } else {
        Err(parse_failed())
    }
}
