use std::time::Duration;
use thirtyfour::prelude::*;
use tokio::{self, time::sleep};

#[tokio::main]
    async fn main() -> WebDriverResult<()> {
    // STARTUP PROCESS:
    // Open 2 cmd prompts and in both type these
    // chromedriver
    // chrome.exe --remote-debugging-port=9222 --user-data-dir="C:\Users\jackw\OneDrive\Desktop\Data"

    // Get the desired tik tok live chat link
    let mut username = String::new();
    // println!("Enter TikTok Username:");
    // let _ = std::io::stdin().read_line(&mut line).unwrap();
    username = String::from("@tv_asahi_news");
    let url = format!("https://www.tiktok.com/{}/live",username);

    // Google account that will be used to login to tiktok
    let mut email = String::new();
    println!("Enter Email:");
    let _ = std::io::stdin().read_line(&mut email).unwrap();
    let mut password = String::new();
    println!("Enter Password:");
    let _ = std::io::stdin().read_line(&mut password).unwrap();
    
    // Attach webdriver to existing google account to avoid login shenanigans
    let mut caps = DesiredCapabilities::chrome();
    caps.set_debugger_address("localhost:9222")?;
    let d = WebDriver::new("http://localhost:9515", caps).await?;
    
    // Go to tik tok and login
    d.goto(url).await?;
    // Wait for sign in screen and click sign in with email
    let sb = d.query(By::Css("div.tiktok-2pt368-DivBoxContainer:nth-child(5)")).first().await?;
    sb.wait_until().displayed().await?;
    sb.click().await?;
    // Get list of windows, empiracally I've found the second window is the login, this may be wrong
    let windows = d.windows().await?;
    d.switch_to_window(windows[1].clone()).await?;
    sleep(Duration::from_millis(1500)).await;
    // Enter email into email box
    let ee = d.query(By::Css("#identifierId")).first().await?;
    ee.send_keys(email).await?;
    // Go to next page
    let next = d.query(By::Css(".VfPpkd-LgbsSe-OWXEXe-k8QpJ > span:nth-child(4)")).first().await?;
    next.click().await?;
    sleep(Duration::from_millis(2000)).await;
    // Enter password into password box
    let ep = d.query(By::Css("#password > div.aCsJod.oJeWuf > div > div.Xb9hP > input")).first().await?;
    ep.send_keys(password).await?;
    // Go to next page
    let next2 = d.query(By::Css(".VfPpkd-LgbsSe-OWXEXe-k8QpJ > span:nth-child(4)")).first().await?;
    next2.click().await?;
    sleep(Duration::from_millis(5000)).await;
    // By this point it's loading and logged in successfully
    
    Ok(())
}

