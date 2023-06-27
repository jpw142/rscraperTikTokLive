# rscraper TikTokLive
This is my tik tok live chat scraper built using a rust port of selenium (thirtyfour). If for some reason you find value in this detritis feel free to reach out, it's certainly my best work and I would love to help with selenium nonsense if I can. I was inspired by all the money-grubbing unofficial tiktok apis and thought, I can at least do better than that... I hope. If you do end up using this in any way I apologize for my atrocious code and hope it serves as a push of point for much better ventures.

Requires sensitiveinfo.json formatted as such:  
{  
    "email": "google account email",  
    "password": "google account password",  
    "account": "@tiktokusername"  
}  

I use chrome remote debugging in order to log into tik tok without the bot warning for logging in  
This works because of frankly a lot of nonsense but the jist of it is instead of creating a dirty automation tagged browser for your script, it starts a 'normal' clean and lovely browser and then takes control of it like a sacculina to a crab  

Here are a few tips I've come across when it comes to the pain of chrome remote debugging:  
    --remote-debugging-port= port you want to have the web driver access chrome on (conventionally 9222)  
    --user-data-dir="directory for a new chrome profile, doesn't matter just needs a folder"  
    in order to launch chrome directly from cmd add the chrome.exe dir to system path in enviromental variables in advanced system  
    preferences  
    put chromedriver in that same file in order to launch it from cmd as well  
    worst comes to worse just restart your computer and try try try again
    
Furthermore you will need an instance of chromedriver running (corresponding to your chome version) and ensure that the port its running on (will be spit out by chromedriver in terminal) is replaced in the code, localhost and all.  

Here is the feature plan and that will be all thank you for reading:  
\[-\] Chat Message Tracking  
\[-\] Donation Tracking  
\[ \] Following Tracking  
\[ \] Sharing Tracking  
\[ \] Joining Tracking  
\[ \] Better Interfaces  
\[ \] Database of coin values for each gift  
\[ \] Database of # of chat messages and donations and such for each person  
\[ \] Key Word Detection  
\[ \] Perhaps some sort of message bot \(if possible\)  
\[ \] Cleaner code \(impossible\)  

