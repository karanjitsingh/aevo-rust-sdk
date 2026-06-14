> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# Prompts

Once connected and authenticated, you can use natural language to interact with AEVO. Below are example prompts across common trading workflows.

### Market Data

> What's the current price of ETH?
>
> Show me the ETH-PERP orderbook
>
> What are the available expiry dates for ETH options?
>
> List all active BTC markets
>
> What's the current funding rate on ETH-PERP?

### Account

> Check my account balance
>
> Show my open positions
>
> What's my portfolio PnL?
>
> Show my recent trade history

### Perpetual Trading

> Buy 0.5 ETH-PERP at $3,800
>
> Set my ETH-PERP leverage to 5x, then open a 1 ETH long at market
>
> Cancel all my open orders
>
> Show my open orders and cancel the ETH ones

### Options — Single Leg

> Buy 1 ETH call option expiring March 28 at strike 4000
>
> Sell 1 BTC put option expiring April 25 at strike 80000

### Options — Butterfly Spread

> Execute an ETH butterfly spread:
>
> * Buy 1 ETH-28MAR25-3500-C
> * Sell 2 ETH-28MAR25-4000-C
> * Buy 1 ETH-28MAR25-4500-C
>
> Show me the risk/reward before executing.

### Options — Iron Condor

> I want to set up an iron condor on ETH expiring March 28:
>
> * Buy 3000 put
> * Sell 3500 put
> * Sell 4500 call
> * Buy 5000 call
>
> What's the max profit and max loss?

### Options — Bull Call Spread

> Set up a bull call spread on ETH for March 28 expiry.
> Lower strike 3800, upper strike 4200.
> Show me the cost and breakeven before placing.

### Options — Bear Put Spread

> I'm bearish on BTC. Set up a bear put spread expiring April 25,
> upper strike 85000, lower strike 75000.

### Options — Straddle

> I think ETH is about to make a big move but I'm not sure which direction.
> Set up a straddle at the ATM strike for the nearest expiry.

### Options — Strangle

> Set up an ETH strangle for March 28:
>
> * Put strike at 3200
> * Call strike at 4800
>   How much will it cost me?

### Strategy Selection

> I'm neutral on ETH but expect high volatility this week.
> What options strategy should I use?
>
> I'm moderately bullish on BTC. What's a good defined-risk trade?

### Multi-Step Workflows

> Analyze the ETH-PERP market — check price, funding rate, orderbook depth,
> and recent trades. Then tell me if it's a good time to go long.
>
> Walk me through the full onboarding process — I'm a new user.
>
> Build me an ETH-PERP order for 0.1 contracts at $3,500 but don't submit it yet.
> Show me the signed payload first.