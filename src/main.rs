use std::time::Duration;
use std::process::Command;

use thirtyfour::prelude::*;
use tokio::{self, time::sleep};

#[tokio::main]
    async fn main() -> WebDriverResult<()> {
    // STARTUP PROCESS:
    // Open 2 cmd prompts and in both type these
    // chromedriver
    // chrome.exe --remote-debugging-port=9222 --user-data-dir="C:\Users\jackw\OneDrive\Desktop\Data"

    // Get the desired tik tok live username chat
    let mut line = String::new();
    println!("Enter TikTok username:");
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    println!("Connecting to {}", line);
    line = String::from("@tv_asahi_news");
    
    // let mut password = String::new();
    // println!("Enter Password:");
    // let _ = std::io::stdin().read_line(&mut password).unwrap();

    // let email = String::from("henrynoname66@gmail.com");


    // let url = format!("https://www.tiktok.com/{}/live",line);


    // Goto the website
    let mut caps = DesiredCapabilities::chrome();
    caps.set_debugger_address("localhost:9222")?;
    let d = WebDriver::new("http://localhost:9515", caps).await?;
    
    d.goto("https://wikipedia.org").await?;
    // Wait for sign in screen and click sign in with email
    // let sb = c.query(By::Css("div.tiktok-2pt368-DivBoxContainer:nth-child(5)")).first().await?;
    // sb.wait_until().displayed().await?;
    // sb.click().await?;

    // Get list of windows, empiracally I've found the second window is the login, this may be wrong
    // let windows = c.windows().await?;
    // c.switch_to_window(windows[1].clone()).await?;
    
    // // Just let it load a bit, life doesn't have to be so fast
    // sleep(Duration::from_millis(1500)).await;

    // let ep = c.query(By::Css("#identifierId")).first().await?;
    // ep.send_keys(email).await?;

    // let next = c.query(By::Css(".VfPpkd-LgbsSe-OWXEXe-k8QpJ > div:nth-child(3)")).first().await?;
    // next.click().await?;

    // sleep(Duration::from_millis(10000)).await;
    
    Ok(())
}

