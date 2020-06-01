mod lib;
use std::io;
fn main() {
    let json_val = match lib::read_input(io::stdin()) {
        Err(why) => panic!("{}", why.to_string()),
        Ok(json_val) => json_val,
    };
    if json_val == "ping" {
        // your code here
        let response = serde_json::json!({ "msg": "pong" });
        match lib::write_output(io::stdout(), &response) {
            Err(why) => panic!("{}", why.to_string()),
            Ok(_) => (),
        };
    }
}
