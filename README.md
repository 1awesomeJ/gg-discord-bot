# Discord bot on flows.network

This flow function acts as a Discord bot.

## Connecting with the bot:
Please join my server "My WasmEdge" using this link https://discord.gg/k43UKcK9
in order to interact with the bot.
Find the bot on our server, and send it a direct message.

## Usage:
This bot is named gg_bot. It helps to fetch the current market price of stocks from most of the world's exchanges.
It expects a message that includes the pattern ```!stock <ticker>``` e.g ```!stock GOOGL```

Here are a few things to note:
1. There could be more texts before and after the ```!stock <ticker>``` pattern. e.g:
User: 
```
Hi gg_bot, could you help me fetch the last closing price of !stock META, perhaps I can buy?
```
gg_bot:
```
The current market price for META is 312.5700. Do not take this as a financial advice
```
2. It handles tickers with hyphens:
User: 
```
Hi gg_bot, could you help me fetch the last closing price of !stock BRK-A perhaps I can buy?
```
gg_bot:
```
The current market price for BRK-A is 553101.0000. Do not take this as a financial advice
```
3. It handles white spaces between ```!stock``` and ```<ticker>```
User: 
```
!stock MSFT
```
gg_bot:
```
The current market price for MSFT is 325.6600. Do not take this as a financial advice
```
User: 
```
!stock        IBM
```
gg_bot:
```
The current market price for IBM is 146.0000. Do not take this as a financial advice
```
4. The regex pattern being used is ```r"!stock\s*([\w-]+)"``` so if the pattern isn't matched, by making sure 
to have ```!stock <ticker> ``` the bot won't be able to fetch the ticker. Examples:
```
stock TSLA
```
```
I couldn't fetch the ticker from your message, please make sure to include the pattern !stock <ticker>, e.g !stock MSFT in your message.
```
```
stock! TSLA
```
```
I couldn't fetch the ticker from your message, please make sure to include the pattern !stock <ticker>, e.g !stock MSFT in your message.
```
```
stock!-TSLA
```
```
I couldn't fetch the ticker from your message, please make sure to include the pattern !stock <ticker>, e.g !stock MSFT in your message.
```

5. The ticker may not be recognized:
```
!stock DANGCEM
```
```
I couldn't fetch the stock price for DANGCEM. you could be entering a wrong ticker, the stock may be outside of our coverage markets, or we could be tracking it with a somewhat different ticker if it's a stock listed outside the U.S markets
```
