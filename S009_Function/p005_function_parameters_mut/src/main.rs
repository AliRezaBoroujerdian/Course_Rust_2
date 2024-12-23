fn main() {
    println!("begin");

    say_hello(String::from("AliReza"), String::from("Boroujerdian"));

    let mut f_name = String::from("Eren");
    let l_name = String::from("Yeager");
    say_hello(f_name, l_name); // Arguments => f_name, l_name

    say_hello(String::from("Harley"), String::from("Quinn"));

    println!("end");
}

fn say_hello(mut first_name: String, mut last_name: String)
/* Parameters => first_name, last_name */
{
    first_name = first_name.to_uppercase();
    last_name = last_name.to_uppercase();
    println!("Hi {} {}!", first_name, last_name);
}
