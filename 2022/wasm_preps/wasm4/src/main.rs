fn main()
{
    let mut message = String::from("Hello");
    let message_2: &mut String = &mut message;

    
    unpredictable_mutate(message_2);
    let message_3 = &message;
    println!("{}", message);
}

fn unpredictable_mutate(val: &mut String)
{
    val.push_str("_unpredictable");
}


