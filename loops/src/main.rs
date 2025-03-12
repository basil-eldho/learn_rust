fn main() {
   let mut s = String::from("hello");

    change(&mut s);
}

fn change(str: &mut String){
    str.push_str(", world");
}