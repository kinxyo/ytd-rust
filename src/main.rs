// use std::collections::HashMap;
use std::io::Write;
use rand::{thread_rng, seq::SliceRandom};

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


    /* CONFIGURING USER-AGENT */ // so as to not get blocked...
    const USER_AGENTS: [&str; 10] = [
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Firefox/89.0",
        "Mozilla/5.0 (Linux; Android 11; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.77 Mobile Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Edge/91.0.864.64",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Opera/77.0.4054.277",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/537.36 (KHTML, like Gecko) Safari/537.36",
        "Mozilla/5.0 (Linux; Android 10; SM-G960U) AppleWebKit/537.36 (KHTML, like Gecko) SamsungBrowser/13.0 Chrome/77.0.3865.92 Mobile Safari/537.36",
        "Mozilla/5.0 (Linux; U; Android 8.1.0; en-US; SM-J737T Build/M1AJQ) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/69.0.3497.100 Mobile Safari/537.36",
        "Mozilla/5.0 (Linux; Android 11; Pixel 4 XL Build/RQ3A.210705.001) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.131 Mobile Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:90.0) Gecko/20100101 Firefox/90.0",
    ];

    let mut rng = thread_rng();
    let user_agent = USER_AGENTS.choose(&mut rng).unwrap_or(&USER_AGENTS[0]); // using `expect` deliberately

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_str(user_agent)?);

    let client = reqwest::Client::new();

    /* FETCHING VIDEO */
    let resp = client.get("https://www.youtube.com/watch?v=1yiNOqPOgHE").headers(headers).send().await?; // `input` will be the argument here, hardcoding the link for now; just for testing.
    let body = resp.text().await?;

    let json = serde_json::from_str(&body)?;
    let formatted_json = serde_json::to_string_pretty(&json)?;

    /* PARSE REQUEST */
    println!("{}", formatted_json);

    Ok(())
}
