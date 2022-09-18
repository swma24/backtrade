fn main() {
    let client = reqwest::blocking::Client::new();
    // let res = client.post("http://httpbin.org/post")
    //     .body("the exact body that is sent")
    //     .send();
    // println!("res = {:?}", res);
    let res = client.get("https://www.rust-lang.org")
        .send();
    println!("res = {:?}", res);
}
