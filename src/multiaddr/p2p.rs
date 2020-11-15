use crate::{
    multibase::Multibase,
    multicodec::{self, Multicodec},
    multihash::Multihash,
    Error, Result,
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct P2p {
    peer_id: PeerId,
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum PeerId {
    Text(String),
    Binr(Vec<u8>),
}

impl PeerId {
    fn to_text(&self) -> Result<Self> {
        let val = match self {
            val @ PeerId::Text(_) => val.clone(),
            PeerId::Binr(buf) => {
                let (mh, _) = Multihash::decode(buf)?;
                PeerId::Text(bs58::encode(mh.encode()?).into_string())
            }
        };

        Ok(val)
    }

    fn to_bytes(&self) -> Result<Self> {
        let val = match self {
            val @ PeerId::Binr(_) => val.clone(),
            PeerId::Text(txt) => PeerId::Binr(text_to_bytes(txt)?),
        };

        Ok(val)
    }
}

impl P2p {
    pub(crate) fn from_text<'a, 'b>(parts: &'a [&'b str]) -> Result<(Self, &'a [&'b str])> {
        let val = match parts {
            [peer_id, tail @ ..] => {
                let peer_id = PeerId::Text(peer_id.to_string());
                (P2p { peer_id }, tail)
            }
            _ => err_at!(BadAddr, msg: "p2p {:?}", parts)?,
        };

        Ok(val)
    }

    pub(crate) fn to_text(&self) -> Result<String> {
        match self.peer_id.to_text()? {
            PeerId::Text(txt) => Ok("/p2p".to_string() + &txt),
            _ => unreachable!(),
        }
    }

    pub(crate) fn decode(data: &[u8]) -> Result<(Self, &[u8])> {
        use unsigned_varint::decode::u128 as uv_decode;

        let val = {
            let (addr, data) = {
                let (n, data) = err_at!(DecodeError, uv_decode(data))?;
                read_slice!(data, (n as usize), "p2p")?
            };
            let val = P2p {
                peer_id: PeerId::Binr(addr.to_vec()),
            };
            (val, data)
        };

        Ok(val)
    }

    pub(crate) fn encode(&self) -> Result<Vec<u8>> {
        use unsigned_varint::encode::u128 as uv_encode;

        let mut buf = [0_u8; 19];

        let addr = match self.peer_id.to_bytes()? {
            PeerId::Binr(buf) => buf,
            _ => unreachable!(),
        };

        let mut data = Multicodec::from_code(multicodec::P2P)?.encode()?;
        data.extend_from_slice(uv_encode(addr.len() as u128, &mut buf));
        data.extend_from_slice(&addr);
        Ok(data)
    }

    pub fn to_peer_id(&self) -> Result<String> {
        match self.peer_id.to_text()? {
            PeerId::Text(txt) => Ok(txt),
            _ => unreachable!(),
        }
    }
}

fn text_to_bytes(text: &str) -> Result<Vec<u8>> {
    let mut chars = text.chars();
    let mh = match (chars.next(), chars.next()) {
        (Some('Q'), Some('m')) | (Some('1'), Some(_)) => {
            // legacy format base58btc.
            let bytes = err_at!(BadInput, bs58::decode(text.as_bytes()).into_vec())?;
            let (mh, _) = Multihash::decode(&bytes)?;
            mh
        }
        _ => {
            let bytes = {
                let mb = Multibase::from_text(text)?;
                match mb.to_bytes() {
                    Some(bytes) => bytes,
                    None => err_at!(BadInput, msg: "{}", text)?,
                }
            };
            // <multicodec-cidv1><libp2p-key-codec><multihash>
            let (codec, bytes) = Multicodec::decode(&bytes)?;
            match codec.to_code() {
                multicodec::CID_V1 => (),
                _ => err_at!(BadInput, msg: "CID {}", codec)?,
            }

            let (codec, bytes) = Multicodec::decode(bytes)?;
            match codec.to_code() {
                multicodec::LIBP2P_KEY => (),
                _ => err_at!(BadInput, msg: "codec {}", codec)?,
            }
            let (mh, _) = Multihash::decode(bytes)?;
            mh
        }
    };

    mh.encode()
}
