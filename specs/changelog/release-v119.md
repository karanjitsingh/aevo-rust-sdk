> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# Release v1.1.9

Introduces weighted rate limits on REST endpoints. For now weights have 2 tiers (2 and 5) and the weights for each endpoint are described below.

## Rate Limits

1. `/index-history`: 5
2. `/index-histories`: 5
3. `/mark-history`: 2
4. `/settlement-history`: 2
5. `/statistics`: 2
6. `/coingecko-statistics`: 2
7. `/funding-history` : 2
8. `/instrument/:instrument_name` : 2
9. `/instrument/:instrument_name/trade-history`: 2
10. `/leaderboard`: 2
11. `/options-history`: 2
12. `/markets-summary` : 2
13. `/last-traded-tradingview/history` : 2
14. `/account/accumulated-fundings`: 2
15. `/portfolio`: 2
16. `/orders/:order-id` : 2
17. `/block-history`: 2
18. `/quote-history`: 2
19. `/block-trade-history`: 2
20. `/order-history`: 5
21. `/order-history/stops`: 5
22. `/trade-history`: 5
23. `/referral-rewards-history`: 2
24. `/referral-history`: 2
25. `/referral-statistics`: 2
26. `/transaction-history`: 2
27. `/pintu/quest-complete`: 2
28. `/pintu/leaderboard`: 2
29. `/balance-history`: 2
30. `/notifications`: 2