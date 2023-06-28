#![allow(unreachable_code)]
#![allow(dead_code)]
use std::{time::Duration, fs::File};
use thirtyfour::prelude::*;
use tokio::{self, time::sleep, task::{self},};

/*
Note to future jack
please fix your code

Note from future jack
no :)
*/
enum EventType {
    Donation(String, u16),
    Message(String),
    Follow,
    Shared,
    Join,
}

struct Event {
    user: String,
    payload: EventType,
}

/* STARTUP PROCESS:
Open cmd prompt and in type these
chrome.exe --remote-debugging-port=9222 --user-data-dir="C:\Users\jackw\OneDrive\Desktop\Data"
chromedriver
*/

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> WebDriverResult<()> {
    // Load google account info
    let file = File::open("sensitiveinfo.json").expect("JSON ERROR JSON ERROR WEEWOOWEEWOO");
    let json: serde_json::Value = serde_json::from_reader(&file).expect("READER ERROR READER ERROR WEEWOOWEEWOO");
    let email = json["email"].as_str().unwrap();
    let password = json["password"].as_str().unwrap();
    let username = json["account"].as_str().unwrap();

    // Get the desired tik tok live chat link
    let url = format!("https://www.tiktok.com/{}/live",username);
    
    // Attach webdriver to existing google chrome instance to avoid login shenanigans
    let mut caps = DesiredCapabilities::chrome();
    // This localhost is the localhost you run chrome remote debugger on with the command
    caps.set_debugger_address("localhost:9222")?;
    // This localhost is the port chromedriver spits out
    let d = WebDriver::new("http://localhost:9515", caps).await?;
    
    // Go to tik tok live page
    d.goto(url).await?;
    
    // Login
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

        // If we already have our email saved in then click it
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
            // Life goes fast, just take a break and let things load for a second
            sleep(Duration::from_millis(5000)).await;
        }
    }
    // By this point the page is loading and has been logged in successfully
    
    // Attatch to all elements that would like to be tracked
    let chat = d.query(By::Css(".tiktok-d3d5tr-DivChatMessageList")).first().await?;
    println!("[-] Chat Attached");
    let donationbar = d.query(By::ClassName("tiktok-wqpdxo-StyledGiftTray")).with_tag("div").first().await?;
    println!("[-] DonationBar Attached");
    // let stickybar = d.query(By::ClassName("tiktok-1pwimsz-DivBottomStickyMessageContainer")).first().await?;
    println!("[-] StickyBar Attached");
    let chatbox = d.query(By::ClassName("tiktok-1k2t5bj-DivCommentContainer")).first().await?;
    let messagebox = chatbox.query(By::ClassName("tiktok-ahx06z-DivEditor")).first().await?;
    println!("[-] Chatbox Attached");

    // Chatter
    println!("[-] Initializing Chatter");
    let mut last_message = chat.clone();
    let chatter = task::spawn(async move {
        loop{
            let mut chatmessages = chat.clone().find_all(By::ClassName("tiktok-1orcc4m-DivChatMessage")).await?;
        
            // Delete all old info
            'outer: for (i, message) in chatmessages.clone().iter_mut().enumerate() {
                // If we find that a message in new scan is equal to old scan, delete all elements before that because they're old
                if message == &last_message.clone() {
                    for _ in 0..=i {
                        chatmessages.remove(0);
                    }
                    break 'outer;
                }
            }

            // If there is no new chat messages then why would we print or do anything silly
            if chatmessages.len() == 0 {
                continue;
            }

            last_message = chatmessages[chatmessages.len() -1].clone();
            for message in chatmessages {
                // If you are getting nonsense with messages not showing its' definitly because not handling these errors
                let userinfo = message.find(By::ClassName("tiktok-1ymr58b-SpanEllipsisName")).await?;
                let comment = message.find(By::ClassName("tiktok-1kue6t3-DivComment")).await?;
                println!("{}: {}", userinfo.inner_html().await?, comment.inner_html().await?);

            }
        }
        Ok::<(), WebDriverError>(())
    });
    println!("[-] Chatter Initialized");

    // Donater
    println!("[-] Initializing Donater");
    // Used to store the data from the element as it gets increased
    let mut multvec: Vec<String> = vec![];
    let mut uservec: Vec<String> = vec![];
    let mut donovec: Vec<String> = vec![];
    // When the element dissapears and errors, adding it to this list will signify it's removal from the prior three lists
    let mut indexes_to_remove: Vec<usize> = vec![];
    let donater = task::spawn(async move {
        loop{
            let donations = donationbar.clone().find_all(By::Css(".tiktok-w5o66o-DivSendGift")).await.expect("piip");

            // If there is no donations then why would we print or do anything silly
            if donations.len() == 0 {
                continue;
            }
            // Avoid any index errors, if somehow the lists are messed up just add 0's
            for _ in 0..donations.len() {
                if multvec.len() != donations.len() {
                    multvec.push(0.to_string());
                }
                if uservec.len() != donations.len() {
                    uservec.push(0.to_string());
                }
                if donovec.len() != donations.len() {
                    donovec.push(0.to_string());
                }
            }

            // I feel so stupid for using this but it works, there must be a better way
            let mut okay;
            for (index, donation) in donations.iter().enumerate() {
                okay = true;
                // Finds the elements
                let multiplier = donation.find(By::ClassName("tiktok-arje3t-SpanBullet")).await;
                let userinfo = donation.find(By::ClassName("tiktok-1e2buzz-DivTitleGift")).await;
                let donation = donation.find(By::ClassName("tiktok-nom0kn-DivDescriptionGift")).await;
                // By this point the element may have dissapeared so we have to check that it exists
                match multiplier {
                    Ok(_) => (),
                    Err(_) => {okay = false;}
                }
                match userinfo {
                    Ok(_) => (),
                    Err(_) => {okay = false;}
                }
                match donation {
                    Ok(_) => (),
                    Err(_) => {okay = false;}
                }
                if okay {
                    // Get the inner vlaues from them
                    let multiplier = multiplier?.inner_html().await;
                    let userinfo = userinfo?.inner_html().await;
                    let donation = donation?.inner_html().await;
                    // By this point the element may have dissapeared so we have to check that it exists
                    match multiplier {
                        Ok(_) => (),
                        Err(_) => {okay = false;}
                    }
                    match userinfo {
                        Ok(_) => (),
                        Err(_) => {okay = false;}
                    }
                    match donation {
                        Ok(_) => (),
                        Err(_) => {okay = false;}
                    }
                    if okay {
                        multvec[index] = multiplier?;
                        uservec[index] = userinfo?;
                        donovec[index] = donation?;
                    }
                    
                }
                // If it has finally errored that means it's dissapeared and reached its maximum true value, now we can print
                if !okay {
                    println!("\n{}: {} {}\n", uservec[index], donovec[index], multvec[index]);
                    indexes_to_remove.push(index.clone());
                }                
            }
            // Gotta remove the detritis from our veins
            for _ in 0..indexes_to_remove.len() {
                if let Some(index) = indexes_to_remove.pop() {
                    multvec.remove(index);
                    uservec.remove(index);
                    donovec.remove(index);
                }
            }
        }
        Ok::<(), WebDriverError>(())
    });
    println!("[-] Donater Initialized");
    
    println!("[-] Joining Handles");
    println!("{:?}", chatter.await);
    println!("{:?}", donater.await);
    Ok(())
}

async fn sendmessage(message: &str, chatbox: WebElement, messagebox: WebElement) -> WebDriverResult<()> {
    // Enter text
    messagebox.send_keys(message).await?;
    // Click send
    chatbox.query(By::ClassName("tiktok-1dgtn4b-DivPostButton")).first().await?.click().await?;
    Ok(())
}