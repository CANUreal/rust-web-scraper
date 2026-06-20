use reqwest::Url;
use scraper::{Html, Selector};
use tokio::io::{self, AsyncBufReadExt, BufReader, AsyncWriteExt};


async fn gettin_input() -> String {
    let mut stdout = io::stdout();
    let _ = stdout.write_all(b"can you spare some url siir?  ").await;
    let _ = stdout.flush().await;
    
    let mut id = String::new();
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);

    reader
        .read_line(&mut id)
        .await
        .expect("Can not read the INPUT BRAHHH");

    let mut input = id.trim().to_string();
    if !input.starts_with("http://") && !input.starts_with("https://") && !input.is_empty() {
        input = format!("https://{}", input);
    }
    input
}

async fn get_data_from_address(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("scraping the url, : {}", url);

    let parsedd_url = Url::parse(url)?;

    let response = reqwest::get(parsedd_url).await?.text().await?;
    let document = Html::parse_document(&response);
    let h1_slctr = Selector::parse("p").map_err(|e| format!("Inavlid selector {:?}", e))?;

    for element in document.select(&h1_slctr) {
        let text = element.text().collect::<Vec<_>>().join( " ");
        println!("> {}", text.trim());
    }
    Ok(())
}
#[tokio::main]
async fn main() -> io::Result<()> {
    let url = gettin_input().await;
    
    if url.is_empty() {
        println!("invalid input");
        return Ok(())
    }

    if let Err(e) = get_data_from_address(&url).await {
        eprintln!("An error occured {}", e);
    }

    let mut stdout = io::stdout();

    stdout.write_all(b"This was all.\n").await?;

    stdout.flush().await?;
    
    stdout.write_all(b"Press RET to end it").await?;
    let _ = stdout.flush().await?;

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut buf = String::new(); 

    reader.read_line(&mut buf).await?;
    
    Ok(())
}