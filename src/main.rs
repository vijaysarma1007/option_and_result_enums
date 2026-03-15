fn main() {
    let muscial_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass: Option<&String> = muscial_instruments.get(2);
    println!("{:?}", bass);
    //let valid_instrument = bass.unwrap();
    play(bass);
    println!("{:?}", bass);
    let valid_instrument = bass.expect("Unable to retrive the element");
    println!("valid instrument: {}", valid_instrument);

    let invalid_instrument = muscial_instruments.get(100);
    play(invalid_instrument);

    //  println!("{:?}", invalid_instrument);
    //  println!("{}", invalid_instrument.expect("unable to extract"));

    let a = Option::Some::<i32>(5);
    let b = Option::Some("hello");
    let c = Option::Some(true);
    let d: Option<&str> = Option::None;

    let avaliability = is_item_in_stock(false, true);
    println!("{avaliability:?}");

    match avaliability {
        Option::Some(value) => println!("Item is avaliable: {value}"),
        Option::None => println!("your item doesn't exist in our system"),
    }

    let present_value = Some(13);
    let missing_value: Option<i32> = None;

    println!("unwarap or : {}", present_value.unwrap_or(0));
    println!("unwarap or : {}", missing_value.unwrap_or(0));
}

fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Option::Some(true)
    } else if item_is_in_system {
        Option::Some(false)
    } else {
        Option::None
    }
}

fn play(instrument_option: Option<&String>) {
    match instrument_option {
        Option::Some(instrument) => {
            println!("playing the {instrument}");
        }
        Option::None => println!("Singing with my voice"),
    }
}
