use std::{error::Error, future::pending};
use tokio::time::timeout;
use tokio::time::Sleep;
use tokio::time::Timeout;
use zbus::{connection, interface};
use waybar_line_show::dict::Dict;
use std::env;
use tokio::time::Duration;
use tokio::time::sleep;

struct Greeter {
    dict: Dict,
}

#[interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
    async fn send_word(&mut self, word: &str)->i32{
        println!("{}", self.dict.search_translation(word));
        return 1;
    }
}

// Although we use `tokio` here, you can use any async runtime of choice.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let endic_path=env::var("HOME").unwrap() + "/.config/waybar/ecdict.csv";
    let greeter = Greeter { dict:Dict::new(&endic_path)?};
    let _conn = connection::Builder::session()?
        .name("org.zbus.MyGreeter")?
        .serve_at("/org/zbus/MyGreeter", greeter)?
        .build()
        .await?;

    // Do other things or go to wait forever
    pending::<()>().await;
    Ok(())
}
