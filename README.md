# RES

**RES** stands for **RealEstateScrapper** it is a project that scrappes real estate information from the Portuguese
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

These strings correspond to the possible modes to run the program

### LLM

For now there is yet no MODE that allows you to run the LLM at ease but it will be implemented in the future. For now
you can call the `call_real_estate_llm` in any schema that implements `ToLLMRequestBody` and it will generate the
following response

```
pub struct LLMRealStateResponse {
    url_id: String,
    no_bedrooms: u32,
    no_bathrooms: u32,
    has_garage: bool,
    has_pool: bool,
    has_good_location: bool,
    location: String,
    average_price: u32,
    average_sqr_meters: u32,
    average_price_per_sqr_meters: u32,
    sqr_meters: u32,
    price: u32,
    summary: String,
    score: u32,
}
```

