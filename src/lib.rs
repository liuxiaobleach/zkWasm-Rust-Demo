use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn zkmain() -> i64 {
    rlp()
}

pub fn rlp() -> i64 {
    let data: Vec<u8> = vec![200, 131, 97, 98, 99, 131, 100, 101, 102];
    println!("{:?}", data);
    let res: String = rlp::decode(&data).unwrap();
    // assert_eq!(_animal, "cat".to_owned());
    println!("{:?}", res);
    0
}

// pub fn rlp_encode() {
//     let data = ["abc", "d"];
//     let res:Vec<u8> = rlp::encode_list::<E, &str>(&data).to_vec();
//     println!("{:?}", res);
// }

pub fn rlp_list(data: Vec<u8>) -> i64 {
    // let data= vec![248, 88, 179, 84, 104, 101, 32, 108, 101, 110, 103, 116, 104, 32, 111, 102, 32, 116, 104, 105, 115, 32, 115, 101, 110, 116, 101, 110, 99, 101, 32, 105, 115, 32, 109, 111, 114, 101, 32, 116, 104, 97, 110, 32, 53, 53, 32, 98, 121, 116, 101, 115, 44, 32, 163, 73, 32, 107, 110, 111, 119, 32, 105, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 73, 32, 112, 114, 101, 45, 100, 101, 115, 105, 103, 110, 101, 100, 32, 105, 116];
    // let data = vec![248, 94, 131, 97, 98, 99, 248, 88, 179, 84, 104, 101, 32, 108, 101, 110, 103, 116, 104, 32, 111, 102, 32, 116, 104, 105, 115, 32, 115, 101, 110, 116, 101, 110, 99, 101, 32, 105, 115, 32, 109, 111, 114, 101, 32, 116, 104, 97, 110, 32, 53, 53, 32, 98, 121, 116, 101, 115, 44, 32, 163, 73, 32, 107, 110, 111, 119, 32, 105, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 73, 32, 112, 114, 101, 45, 100, 101, 115, 105, 103, 110, 101, 100, 32, 105, 116];
    // let data = vec![0xc3, 0x82,0x53,0x51];
    // let data = vec![0xc5,0x12,0x18,0xc2,0x12,0x18];
    println!("{:?}", data);
    let res: String = rlp::decode(&data).unwrap();
    // assert_eq!(_animal, "cat".to_owned());
    println!("{:?}", res);
    0
}

#[cfg(test)]
mod tests {
    // use crate::_decode;
    use reth_rlp::Decodable;
    // use bytes::Bytes;
    // use reth_rlp::RlpEncodable;
    // use reth_rlp::RlpDecodable;

    use reth_primitives::ReceiptWithBloom;
    // use revm_primitives::{B160};

    #[test]
    fn test_rlp() {
        let addr = "0x000000000022d473030f116ddee9f6b43ac78ba3";
        let expected = vec![249,4,223,1,131,147,131,3,185,1,0,0,1,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,32,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,10,0,0,0,0,0,2,1,0,0,8,0,32,0,0,0,0,0,0,8,0,0,0,0,0,8,0,0,0,8,8,0,0,8,0,0,0,0,0,0,0,0,0,64,0,0,0,0,0,0,0,32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,0,0,0,0,16,0,8,0,0,0,0,0,64,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,32,0,0,0,32,0,0,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,0,32,0,0,0,8,0,0,0,0,0,2,0,0,0,0,0,0,0,8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,32,0,0,0,0,0,0,0,16,0,0,0,0,0,4,0,16,0,0,0,0,0,0,0,0,0,0,0,4,0,249,3,212,248,253,148,0,0,0,0,0,34,212,115,3,15,17,109,222,233,246,180,58,199,139,163,248,132,160,198,163,119,191,196,235,18,0,36,168,172,8,238,242,5,190,22,184,23,2,8,18,199,50,35,232,29,27,219,151,8,236,160,0,0,0,0,0,0,0,0,0,0,0,0,245,233,213,80,195,197,3,100,214,48,237,180,117,59,228,4,205,16,145,33,160,0,0,0,0,0,0,0,0,0,0,0,0,160,184,105,145,198,33,139,54,193,209,157,74,46,158,176,206,54,6,235,72,160,0,0,0,0,0,0,0,0,0,0,0,0,63,201,26,58,253,112,57,92,212,150,198,71,213,166,204,157,75,43,127,173,184,96,0,0,0,0,0,0,0,0,0,0,0,0,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,100,214,65,206,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,248,155,148,192,42,170,57,178,35,254,141,10,14,92,79,39,234,217,8,60,117,108,194,248,99,160,221,242,82,173,27,226,200,155,105,194,176,104,252,55,141,170,149,43,167,241,99,196,161,22,40,245,90,77,245,35,179,239,160,0,0,0,0,0,0,0,0,0,0,0,0,136,230,160,194,221,210,111,238,182,79,3,154,44,65,41,111,203,63,86,64,160,0,0,0,0,0,0,0,0,0,0,0,0,63,201,26,58,253,112,57,92,212,150,198,71,213,166,204,157,75,43,127,173,160,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,84,66,237,153,70,225,209,248,155,148,160,184,105,145,198,33,139,54,193,209,157,74,46,158,176,206,54,6,235,72,248,99,160,221,242,82,173,27,226,200,155,105,194,176,104,252,55,141,170,149,43,167,241,99,196,161,22,40,245,90,77,245,35,179,239,160,0,0,0,0,0,0,0,0,0,0,0,0,245,233,213,80,195,197,3,100,214,48,237,180,117,59,228,4,205,16,145,33,160,0,0,0,0,0,0,0,0,0,0,0,0,136,230,160,194,221,210,111,238,182,79,3,154,44,65,41,111,203,63,86,64,160,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,59,154,202,0,249,1,28,148,136,230,160,194,221,210,111,238,182,79,3,154,44,65,41,111,203,63,86,64,248,99,160,196,32,121,249,74,99,80,215,230,35,95,41,23,73,36,249,40,204,42,200,24,235,100,254,216,0,78,17,95,188,202,103,160,0,0,0,0,0,0,0,0,0,0,0,0,63,201,26,58,253,112,57,92,212,150,198,71,213,166,204,157,75,43,127,173,160,0,0,0,0,0,0,0,0,0,0,0,0,63,201,26,58,253,112,57,92,212,150,198,71,213,166,204,157,75,43,127,173,184,160,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,59,154,202,0,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,248,171,189,18,102,185,30,47,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,89,202,157,112,255,119,47,253,132,4,101,163,133,156,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,202,88,234,188,212,72,118,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,16,159,248,122,148,192,42,170,57,178,35,254,141,10,14,92,79,39,234,217,8,60,117,108,194,248,66,160,127,207,83,44,21,240,166,219,11,214,208,224,56,190,167,29,48,216,8,199,217,140,179,191,114,104,169,91,245,8,27,101,160,0,0,0,0,0,0,0,0,0,0,0,0,63,201,26,58,253,112,57,92,212,150,198,71,213,166,204,157,75,43,127,173,160,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,84,66,237,153,70,225,209];
        let decoded = ReceiptWithBloom::decode( &mut &*expected).expect("decode failure");
        let decoded_addr = decoded.receipt.logs[0].address;
        // assert_eq!(decoded_addr.as_bytes(), addr.as_bytes());
        println!("decoded_addr {:?}", decoded_addr);
        println!("addr {:?}", addr);
        println!("decoded {:?}", decoded);
    }
}