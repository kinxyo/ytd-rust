// use std::collections::HashMap;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    /* TAKE USER INPUT */
    print!("Paste your link: ");

    let mut input = String::new();
    std::io::stdout().flush().expect("failed to flush the buffer");
    std::io::stdin().read_line(&mut input).expect("failed to parse the input");

    println!("\nON IT!");

    /* URL VALIDATION */
    /* ... */

    /* SEND REQUEST */
    let resp = reqwest::get("https://www.youtube.com/watch?v=1yiNOqPOgHE").await?; // `input` will be the argument here, hardcoding the link for now; just for testing.
    let body = resp.text().await?;


    /* PARSE REQUEST */
    println!("{}", body);



    Ok(())
}
