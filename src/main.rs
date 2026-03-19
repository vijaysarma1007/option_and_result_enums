use std::fmt::Result;

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

    let ok = Ok::<i32, &str>(5);
    let disaster = Err::<i32, &str>("something went wrong!");

    println!("{:?}", ok);
    println!("{:?}", disaster);

    let text = "50";
    let text_number = text.parse::<i32>();
    println!("text as number: {:?}", text_number);

    let result = divide(10.0, 2.0);

    println!("{}", result.is_ok());
    println!("{}", result.is_err());

    println!("{}", result.as_ref().unwrap());
    println!(
        "{}",
        result.as_ref().expect("Unabvle to parse calculation.")
    );
    println!("{}", result.unwrap_or(5.0));

    // match result {
    //     Ok(calculation) => println!("Result: {}", calculation),
    //     Err(message) => println!("Error: {}", message),
    // }

    let my_result = operation(true);

    let content = match my_result {
        Ok(message) => message,
        Err(error) => error.to_string(),
    };

    let mut sauces = vec!["Mayonaise", "Ketchup", "Ranch"];

    while let Some(sauce) = sauces.pop() {
        println!("The next sauce is {sauce}");
    }
}

fn operation(great_success: bool) -> std::result::Result<String, &'static str> {
    if great_success {
        Ok("Success".to_string())
    } else {
        Err("error")
    }
}

fn divide(numerator: f64, denominator: f64) -> std::result::Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
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
