# RES

**RES** stands for **RealEstateScrapper** it is a project that scrapes real estate information from the Portuguese
market.
It also allows for an LLM assistant to better analyse if a particular house/apartment is a good deal.

## Requirements

To be able to run the scrappers you need a [web driver](https://www.selenium.dev/documentation/webdriver/) you can
install a webdriver like [Chrome´s driver](https://developer.chrome.com/docs/chromedriver/get-started) to download it
check [here](https://googlechromelabs.github.io/chrome-for-testing/#stable). If you have a Mac you should already have
it under /usr/bin/safaridriver
Note: Safari doesn't support concurrent runs.

To be able to use the LLM you need an API key for [OpenRouter](https://openrouter.ai). The model currenly in use
is [meta-llama/llama-3.2-3b-instruct:free](https://openrouter.ai/meta-llama/llama-3.2-3b-instruct:free), it's completly
free so all you need to do is create an account and you should be good to go.

## How to use it

### Scrappers

To run a particular Scrapper you need to specify in your `.env` file a `MODE` to select what scrapper you want to run
and the `DRIVER_PATH` to point to your web driver.
The following scrappers are implemented:

- remax
- era
- supercasas
- imovirtual
- idealista

These strings correspond to the possible modes to run the program

### LLM

To run the LLM you need to specify in your `.env` file a `MODE` that should have the value of llm, a
`OPEN_ROUTER_API_KEY`,
a `INPUT_PATH` and a `OUTPUT_PATH`.
It will output a Json with the response of the model to the target `OUTPUT_PATH`, it will use each JSON inside
`INPUT_PATH`
as the body.
See below the Json schema.

```
pub struct LLMRealStateResponse {
    url_id: String,
    no_bedrooms: u32,
    no_bathrooms: u32,
    has_garage: bool,
    has_pool: bool,
    has_good_location: bool,
    location: String,
    average_price: f32,
    average_sqr_meters: f32,
    average_price_per_sqr_meters: f32,
    sqr_meters: f32,
    price: Option<f32>,
    summary: Option<String>,
    score: f32,
}
```

#### Known limitations

Open router has a limit on the free models, it is limited to 20 requests per minute and 200 per day. For more
information see [limits](https://openrouter.ai/docs/limits) 
