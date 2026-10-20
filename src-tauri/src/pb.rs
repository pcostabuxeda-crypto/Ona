//! Protobuf mínim per a `collection2v2` (biblioteca), que librespot no compila.
//! PageRequest/WriteRequest/PageResponse: només camps string, varint i missatges.

fn varint(out: &mut Vec<u8>, mut v: u64) {
    while v >= 0x80 {
        out.push(v as u8 | 0x80);
        v >>= 7;
    }
    out.push(v as u8);
}

fn bytes(out: &mut Vec<u8>, field: u32, b: &[u8]) {
    varint(out, (field << 3 | 2) as u64);
    varint(out, b.len() as u64);
    out.extend_from_slice(b);
}

fn int(out: &mut Vec<u8>, field: u32, v: u64) {
    varint(out, (field << 3) as u64);
    varint(out, v);
}

/// PageRequest { username=1, set=2, pagination_token=3, limit=4 }
pub fn page_request(user: &str, set: &str, token: &str, limit: u32) -> Vec<u8> {
    let mut o = Vec::new();
    bytes(&mut o, 1, user.as_bytes());
    bytes(&mut o, 2, set.as_bytes());
    if !token.is_empty() {
        bytes(&mut o, 3, token.as_bytes());
    }
    int(&mut o, 4, limit as u64);
    o
}

/// WriteRequest { username=1, set=2, items=3 [CollectionItem { uri=1, added_at=2, is_removed=3 }] }
pub fn write_request(user: &str, set: &str, uris: &[String], remove: bool, now: u64) -> Vec<u8> {
    let mut o = Vec::new();
    bytes(&mut o, 1, user.as_bytes());
    bytes(&mut o, 2, set.as_bytes());
    for uri in uris {
        let mut item = Vec::new();
        bytes(&mut item, 1, uri.as_bytes());
        int(&mut item, 2, now);
        if remove {
            int(&mut item, 3, 1);
        }
        bytes(&mut o, 3, &item);
    }
    o
}

pub struct Item {
    pub uri: String,
    pub added_at: u64,
    pub removed: bool,
}

/// Recorre els camps d'un missatge: (camp, varint o bytes).
fn fields(mut b: &[u8]) -> Option<Vec<(u32, u64, &[u8])>> {
    let mut out = Vec::new();
    let read = |b: &mut &[u8]| -> Option<u64> {
        let mut v = 0u64;
        for shift in (0..64).step_by(7) {
            let (&byte, rest) = b.split_first()?;
            *b = rest;
            v |= ((byte & 0x7f) as u64) << shift;
            if byte < 0x80 {
                return Some(v);
            }
        }
        None
    };
    while !b.is_empty() {
        let key = read(&mut b)?;
        let (field, wire) = ((key >> 3) as u32, key & 7);
        match wire {
            0 => out.push((field, read(&mut b)?, &[][..])),
            2 => {
                let len = read(&mut b)? as usize;
                if len > b.len() {
                    return None;
                }
                let (v, rest) = b.split_at(len);
                out.push((field, 0, v));
                b = rest;
            }
            1 => b = b.get(8..)?,
            5 => b = b.get(4..)?,
            _ => return None,
        }
    }
    Some(out)
}

/// PageResponse { items=1, next_page_token=2, sync_token=3 } → (ítems, següent pàgina)
pub fn page_response(b: &[u8]) -> Option<(Vec<Item>, String)> {
    let mut items = Vec::new();
    let mut next = String::new();
    for (f, _, v) in fields(b)? {
        match f {
            1 => {
                let mut it = Item { uri: String::new(), added_at: 0, removed: false };
                for (g, n, w) in fields(v)? {
                    match g {
                        1 => it.uri = String::from_utf8_lossy(w).into(),
                        2 => it.added_at = n,
                        3 => it.removed = n != 0,
                        _ => {}
                    }
                }
                items.push(it);
            }
            2 => next = String::from_utf8_lossy(v).into(),
            _ => {}
        }
    }
    Some((items, next))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        // Una WriteRequest té la mateixa forma d'ítems que una PageResponse (camp 3 vs 1).
        let w = write_request("u", "collection", &["spotify:track:x".into()], true, 300);
        let f = fields(&w).unwrap();
        assert_eq!(f[0].2, b"u");
        assert_eq!(f[1].2, b"collection");
        let mut resp = Vec::new();
        bytes(&mut resp, 1, f[2].2);
        bytes(&mut resp, 2, b"next");
        let (items, next) = page_response(&resp).unwrap();
        assert_eq!(next, "next");
        assert_eq!(items[0].uri, "spotify:track:x");
        assert_eq!(items[0].added_at, 300);
        assert!(items[0].removed);
        assert_eq!(page_request("u", "s", "", 300), [10, 1, b'u', 18, 1, b's', 32, 172, 2]);
    }
}
