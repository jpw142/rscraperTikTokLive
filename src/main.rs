use std::{time::Duration, fs::File};
use thirtyfour::prelude::*;
use tokio::{self, time::sleep};

#[tokio::main]
    async fn main() -> WebDriverResult<()> {
    // STARTUP PROCESS:
    // Open cmd prompt and in type these
    // chrome.exe --remote-debugging-port=9222 --user-data-dir="C:\Users\jackw\OneDrive\Desktop\Data"
    // chromedriver

    // Load google account info
    let file = File::open("sensitiveinfo.json").expect("JSON ERROR JSON ERROR WEEWOOWEEWOO");
    let json: serde_json::Value = serde_json::from_reader(&file).expect("READER ERROR READER ERROR WEEWOOWEEWOO");
    let email = json["email"].as_str().unwrap();
    let password = json["password"].as_str().unwrap();
    let username = json["account"].as_str().unwrap();

    // Get the desired tik tok live chat link
    let url = format!("https://www.tiktok.com/{}/live",username);
    
    // Attach webdriver to existing google account to avoid login shenanigans
    let mut caps = DesiredCapabilities::chrome();
    caps.set_debugger_address("localhost:9222")?;
    let d = WebDriver::new("http://localhost:9515", caps).await?;
    
    // Go to tik tok
    d.goto(url).await?;
    // Checks if the login screen comes up
    let login = d.find(By::Css(".tiktok-aiuhe9-DivModalContent")).await;
    if let Ok(_) = login {
        // If found we should login
        // Wait for sign in screen and click sign in with email
        let sb = d.query(By::Css("div.tiktok-2pt368-DivBoxContainer:nth-child(5)")).first().await?;
        sb.wait_until().displayed().await?;
        sb.click().await?;
        sleep(Duration::from_millis(2500)).await;
        
        // Get list of windows and switch to login window, empiracally I've found the second window is the login, this may be wrong
        let windows = d.windows().await?;
        d.switch_to_window(windows[1].clone()).await?;
        sleep(Duration::from_millis(1500)).await;

        // If we already have our email in then click it else
        let prelog = d.find(By::Css(".tgnCOd")).await;
        if let Ok(element) = prelog {
            element.click().await?;
        }
        else {
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
        }
    }
    Ok(())
}

