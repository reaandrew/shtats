const path = require('path');
const puppeteer = require('puppeteer');


(async () => {
    const reportPath = path.resolve("./", "report.html");
    const browser = await puppeteer.launch({defaultViewport: null, args: ['--no-sandbox', '--disable-setuid-sandbox']});
    const page = await browser.newPage();
    await page.setViewport({
        width: 1200,
        height: 1000,
    });
    await page.goto('file:///'+reportPath);
    await page.screenshot({path: '/tmp/report-screenshot.png', fullPage: true});

    await browser.close();
})();
