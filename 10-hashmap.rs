use std::collections::HashMap;

fn main(){
    let input_vec= vec![(String::from("log1c"),21), (String::from("billu"),22)];
    let hm = get_key_value(input_vec);
    println!("{:?}",hm);

}

fn get_key_value(v: Vec<(String, i32)>)-> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in v {
        hm.insert(key, value);
    }

    hm
}
