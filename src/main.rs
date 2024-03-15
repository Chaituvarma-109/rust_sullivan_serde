use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, to_string_pretty};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Dog {
    name: String,
    year_born: i32,
    owner: DogOwner,
}

#[derive(Serialize, Deserialize, Debug)]
struct DogOwner {
    first_name: String,
    last_name: String,
}

fn main() {
    // test_serialize();
    test_deserialize();
}

fn test_serialize() {
    let p1 = DogOwner{first_name: "Trevor".to_string(), last_name: "Sullivan".to_string()};
    let d1 = Dog{name: "Cheyenne".to_string(), year_born: 2021, owner: p1};

    let d1_ser = to_string_pretty(&d1);
    if d1_ser.is_ok() {
        println!("{}", d1_ser.ok().unwrap());
    } else {
        println!("{:?}", d1_ser.err());
    }
}

fn test_deserialize() {
    let json_string = r#"
    {
        "name": "Cheyenne",
        "year_born": 2021,
        "owner": {
          "first_name": "Trevor",
          "last_name": "Sullivan"
        }
    }
    "#;

    let dog_deser = from_str::<Dog>(&json_string);

    if dog_deser.is_ok() {
        println!("{:#?}", dog_deser.ok().unwrap());
    } else {
        println!("{:#?}", dog_deser.err());
    }
}
