
const playwright = require("playwright-aws-lambda")
export default async function generatePdf(req, res) {
    try {
    const browser = await playwright.launchChromium({ headless: true });
  
      const context = await browser.newContext();
  
      const page = await context.newPage();
  
      await page.goto(`https://plants.usda.gov/home/plantProfile?symbol=${"ABBA"}`);
      await page.waitForSelector("img[_ngcontent-c0]")
      let data = await page.evaluate(()=>{
        let data = {}
        data.img =  document.querySelector("img[_ngcontent-c0]").src
        return JSON.stringify(data)
      })
      await page.locator("a[aria-controls=CharacteristicsTab-panel]").click()
      let stats = await page.evaluate(()=>{
        let data = {}
        let fields = ["Active Growth Period", "Lifespan", "Bloom Period", "Precipitation, Minimum"]
        Array.from(document.getElementsByTagName('tr')).forEach(e=> {
            fields.forEach(field=> {
                if(e.children[0].textContent === field)
                data[field] = e.children[1].textContent
            })
        })
        return JSON.stringify(data)
      })
      let response = {...JSON.parse(data), ...JSON.parse(stats)}
      await browser.close();
  
      return res.status(200).json([response]);
    } catch (error) {
      return res.status(error.statusCode || 500).json({ error: error.message });
    }
  }