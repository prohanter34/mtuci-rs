use my_hash_map::HashM;


fn main() {
    let mut h: HashM<u8, String> = HashM::new();
    h.insert(2, String::from("value"));
    h.insert(2, String::from("fefefef"));
    println!("{}", h.get(2).unwrap());
    println!("{:?}", h);
}
