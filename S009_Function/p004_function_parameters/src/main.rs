fn main() {
    println!("begin");

    say_hello(String::from("AliReza"), String::from("boroujerdian"));

    let f_name = String::from("Eren");
    let l_name = String::from("Yeager");
    say_hello(f_name, l_name); //Arguments => f_name and l_name

    say_hello(String::from("Harley"), String::from("Quinn"));

    println!("end");
}

fn say_hello(first_name: String, last_name: String) /*Parameters => first_name and last_name*/
{
    println!("Hello, {first_name} {last_name}!");
}
