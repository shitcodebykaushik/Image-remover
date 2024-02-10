use error_chain::error_chain;
use select::document::Document;

use select::predicate::Name;

 error_chain!{
    foreign_links{
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }

 }
   #[tokio::main]
   async fn main() -> Result<()> {
    let res = reqwest::get("https://experience.dropbox.com/resources/what-does-sync-mean")
    .await?
    .text()
    .await?;


    Document::from(res.as_str())
    .find(Name("a"))
    .filter_map(|n|n.attr("href"))
    .for_each(|x|println!("{}",x));

    Ok(())
   }
