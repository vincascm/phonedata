
fn main() {
    let phone_data_file = std::env::args().nth(1).expect("missing data file");
    let phone_data = phonedata::PhoneData::new(&phone_data_file).unwrap();
    let phone = std::env::args().nth(2).expect("missing phone number");
    println!("find: {:?}", phone_data.find(&phone).unwrap());
}

