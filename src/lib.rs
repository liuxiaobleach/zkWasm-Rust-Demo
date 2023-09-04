use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn zkmain() -> i64 {
    rlp()
}

// pub fn rlp() -> i64 {
//     let data = [200, 131, 97, 98, 99, 131, 100, 101, 102];
//     let animal: String = rlp::decode(&data).expect("could not decode");
//     println!("{:?}", animal);
//     assert_eq!(animal, "cat".to_owned());
//     0
// }

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

pub enum DataType {
    STRING,
    LIST
}

#[cfg(test)]
mod tests {
    //use crate::_decode;
    use reth_rlp::Decodable;
    use bytes::Bytes;
    use reth_rlp::RlpEncodable;
    use reth_rlp::RlpDecodable;

    #[derive(Debug, PartialEq, RlpEncodable, RlpDecodable)]
    struct Item {
        a: Bytes,
    }

    #[test]
    fn test_rlp() {
        let item = Item { a: b"dog".to_vec().into() };

        let expected = vec![0xc4, 0x83, b'd', b'o', b'g'];
        let decoded = Decodable::decode(&mut &*expected).expect("decode failure");
        assert_eq!(item, decoded);
        println!("{:?}", decoded);
        println!("{:?}", item);
    }
}