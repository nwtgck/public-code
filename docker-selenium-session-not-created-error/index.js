// index.js

const webdriver = require("selenium-webdriver");

(async () => {
  const driver = new webdriver.Builder()
    .forBrowser(webdriver.Browser.FIREFOX)
    .usingServer("http://localhost:4444/wd/hub").build();
  console.log("driver created");

  console.log("Getting...");
  await driver.get("https://example.com");
  const title = await driver.getTitle();
  console.log(`title=${title}`);
  // expected => title=Example Domain
})();
