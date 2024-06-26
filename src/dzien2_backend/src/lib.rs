#[ic_cdk::query]
fn greet(name: String, surname: u8) -> String {
    format!("Hello, {} {}!", name, surname)
}
