extern crate reqwest;

fn main() {
    match reqwest::get("http://home.localhost/hello") {
        Ok(mut response) => {
            //Check if 200 Ok
            if response.status() == reqwest::StatusCode::Ok {
                match response.text() {
                    Ok(text) => println!("Response Text: {}", text),
                    Err(_) => println!("Could not read response text")
                }
            } else {
                println!("Response was not 200 OK.");
            }
        },
        Err(_) => println!("Could not make the request.")
    }
}
