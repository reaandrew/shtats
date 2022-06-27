const fs = require('fs');
const path = require('path');
const tmp = require("tmp");
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

    const tmpobj = tmp.fileSync({postfix: '.png'});
    await page.screenshot({path: tmpobj.name, fullPage: true});
    await browser.close();
    fs.renameSync(tmpobj.name, "./docs/images/shtats_thumbnail.png")

})();
