use digest::Digest;
use tari_crypto::common::Blake256;
use tari_mmr::MerkleMountainRange;

pub type Hasher = Blake256;


pub fn mmr_push_bytes(data: &[u8]) {
    if let Ok(_s) = std::str::from_utf8(data) {
        let mut mmr = MerkleMountainRange::<Hasher, _>::new(Vec::default());

        let empty_hash = Hasher::digest(b"").to_vec();
        assert_eq!(mmr.get_merkle_root(), Ok(empty_hash));

        for n in 0..1001 {
            let hash = Hasher::digest(&data).to_vec();
            //  println!("Hash is: {:?}", &hash);
            let pushd = mmr.push(hash);
            assert!(pushd.is_ok());
            let _cnt = mmr.get_leaf_count().unwrap();
            assert!(mmr.len().is_ok());

            // println!("Leaf count: {:?}", cnt);
            let _mroot = mmr.get_merkle_root();

            // println!("Merkle root: {:?}", mroot);
            let leaf_hashes = mmr.get_leaf_hashes(n as usize, n as usize).unwrap();
            // println!("Leaf hashes: {:?}", &leaf_hashes);
            assert!(mmr.assign(leaf_hashes).is_ok());

            assert!(mmr.validate().is_ok());
        }

        // let pushd1 = mmr.push(&data.to_vec());
    }
}
