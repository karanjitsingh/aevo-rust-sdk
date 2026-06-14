> ## Documentation Index
> Fetch the complete documentation index at: https://api-docs.aevo.xyz/llms.txt
> Use this file to discover all available pages before exploring further.

# SUBSCRIBE Ticker Throttled (NEW)

<div class="ws-method-div"><span class="APIMethod APIMethod_fixedWidth ws-sub-tag">SUB</span> <span class="ws-small-font">wss://ws.aevo.xyz</span></div><span class="ws-description">Returns instrument ticker information when its top of the book changes and returns information such as: instrument data, funding rates, bid, asks and mark information.</span>

<span class="ws-small-font">**UPDATE TIME**</span>

<span class="ws-data-type ws-small-font">**500ms**</span>

<span class="ws-small-font">**REQUEST**</span>

<Table align={["left"]}>
  <thead>
    <tr>
      <th />
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>
        * \*op\*\* <span class="ws-data-type ws-small-font">string</span>  <span class="ws-required-tag ws-small-font">required</span>
          Operation code allowed values: `subscribe` `unsubscribe`
      </td>
    </tr>

    <tr>
      <td>
        * \*data\*\* <span class="ws-data-type ws-small-font">array of strings</span>  <span class="ws-required-tag ws-small-font">required</span>\
          Channel name in the format `ticker-500ms:ASSET:INSTRUMENT_TYPE` or `ticker-500ms:INSTRUMENT_NAME`.\
          Eg. `ticker-500ms:ETH:OPTION` or `ticker-500ms:ETH-31MAR23-1350-C`.
      </td>
    </tr>
  </tbody>
</Table>

* `ASSET` allowed values: `ETH`
* `INSTRUMENT_TYPE` allowed values : `OPTION` `PERPETUAL`

<details />

<summary>Example</summary>

```json
{
	"op":"subscribe", 
	"data": ["ticker-500ms:ETH:OPTION"]
}
```

```json
{
	"op":"subscribe", 
	"data": ["ticker-500ms:ETH-31MAR23-1350-C"]
}
```

<br />

<span class="ws-small-font">**RESPONSE**</span>

<Table align={["left"]}>
  <thead>
    <tr>
      <th />
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>
        * \*channel\*\* <span class="ws-data-type ws-small-font">string</span>  <span class="ws-required-tag ws-small-font">required</span>
          Channel name in the format `ticker:ASSET:INSTRUMENT_TYPE` or `ticker:INSTRUMENT_NAME`.
          Eg. `ticker:ETH:OPTION` or `ticker:ETH-31MAR23-1350-C`.
      </td>
    </tr>

    <tr>
      <td>
        <span class="ws-small-font">data.</span>**timestamp** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>\
        Update timestamp in UNIX timestamp in nanoseconds. Eg. `1680249600000000000`
      </td>
    </tr>

    <tr>
      <td>
        <span class="ws-small-font">data.</span>**tickers** <span class="ws-data-type ws-small-font">array of objects</span> <span class="ws-required-tag ws-small-font">required</span>\
        Ticker information.

        <div style={{ height: "10px" }} /><table><tr><td class="child-header">**TICKER OBJECT**</td></tr><tr><td>**instrument\_id** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>
        Instrument ID number. Eg. `12`</td></tr><tr><td>**instrument\_name** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>
        Instrument name. Eg.` ETH-24DEC22-1250-C`</td></tr><tr><td>**instrument\_type** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>
        Type of instrument. Allowed values: `OPTION` `PERPETUAL`</td></tr><tr><td>**funding\_rate** <span class="ws-data-type ws-small-font">string</span> <br />Funding rate of the last epoch</td></tr><tr><td>**next\_funding\_rate** <span class="ws-data-type ws-small-font">string</span><br />Estimated funding rate of the next epoch</td></tr><tr><td>**mark** <span class="ws-data-type ws-small-font">object</span> <span class="ws-required-tag ws-small-font">required</span>
        Mark price details in `price` object format.
        <div style={{ height: "10px" }} /><table><tr><td class="child-header">**PRICE LEVEL OBJECT**</td></tr><tr><td>**price** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>
        Price in USD. </td></tr><tr><td>**delta** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>
        Option's Delta. </td></tr><tr><td>**theta** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>
        Option's Theta. </td></tr><tr><td>**gamma** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>
        Option's Rho. </td></tr><tr><td>**rho** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>
        Option's Rho. </td></tr><tr><td>**vega** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>
        Option's Vega. </td></tr><tr><td>**iv** <span class="ws-data-type ws-small-font">string</span> <span class="ws-required-tag ws-small-font">required</span>
        Implied volatility. </td></tr><tr><td>**amount** <span class="ws-data-type ws-small-font">string</span>
        Amount of contracts. </td></tr></table></td></tr><tr><td>**bid** <span class="ws-data-type ws-small-font">object</span> <span class="ws-required-tag ws-small-font">required</span>
        Top bid details in `ticker` object format.
        <div style={{ height: "10px" }} /><table><tr><td class="child-header">**PRICE LEVEL OBJECT**</td></tr></table></td></tr><tr><td>**ask** <span class="ws-data-type ws-small-font">object</span> <span class="ws-required-tag ws-small-font">required</span>
        Top ask details in `ticker` object format.
        <div style={{ height: "10px" }} /><table><tr><td class="child-header">**PRICE LEVEL OBJECT**</td></tr></table></td></tr></table>
      </td>
    </tr>
  </tbody>
</Table>

<details />

<summary>Example</summary>

```json
{
    "channel": "ticker-500ms:ETH:OPTION",
    "data": {
        "timestamp": "1673436965238291661",
        "tickers": [
            {
                "instrument_id": 165,
                "instrument_name": "ETH-31MAR23-1350-C",
                "instrument_type": "OPTION",
              	"funding_rate": "0.000026",
                "next_funding_rate": "0.00002",
                "mark": {
                    "price": "140.007862",
                    "delta": "0.540531375408708",
                    "theta": "-0.9224266733203602",
                    "gamma": "0.0010828257944813621",
                    "rho": "1.2586305929021393",
                    "vega": "2.466304065911212",
                    "iv": "0.5898161402416873"
                },
                "bid": {
                    "price": "2",
                    "delta": "0.2159147503564693",
                    "theta": "-0.03033364841496897",
                    "gamma": "0.017935537671398397",
                    "rho": "0.6193026765188775",
                    "vega": "2.466304065911212",
                    "iv": "0.026280592178461275",
                    "amount": "10"
                },
                "ask": {
                    "price": "10",
                    "delta": "0.3756922766741976",
                    "theta": "-0.09391922572782523",
                    "gamma": "0.009719490995099413",
                    "rho": "1.063503444637495",
                    "vega": "2.466304065911212",
                    "iv": "0.06281820373274899",
                    "amount": "1"
                }
            }
        ]
    }
}
```