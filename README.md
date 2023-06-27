# rscraper
This is my tik tok live chat scraper built using a rust port of selenium (thirtyfour). If for some reason you find value in this detritis feel free to reach out, it's certainly my best work.
Requires sensitiveinfo.json formatted as such:
{
    "email": "google account email",
    "password": "google account password",
    "account": "@tiktokusername"
}
Here are a few tips I've come across when it comes to the pain of chrome remote debugging:
--remote-debugging-port= port you want to have the web driver access chrome on (conventionally 9222)
--user-data-dir="directory for a chrome profile, doesn't matter just needs a folder"
in order to launch chrome directly from cmd add the chrome.exe dir to path in enviromental variables in advanced system preferences
put chromedriver in that same.exe fiel in order to launch it from cmd as well
Furthermore you will need an instance of chromedriver running (corresponding to your chome version) and ensure that the port its running on (doesn't have to be the same as chrome port) is replaced in the code, localhost and all.
