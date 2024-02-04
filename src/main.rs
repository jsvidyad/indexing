mod info;

use info::Info;

fn main() {
    let mut user = Info::new("Jyothish Vidyadharan".to_string(), 
                        vec!["jyothishv@gmail.com".to_string(), "jsvidyad@syr.edu".to_string()]);
    println!("{:?}", user);
    user[1] = "jyothish_jobs@yahoo.com".to_string();
    println!("{:?}", user);
}
