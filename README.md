# About

A cross platform GUI for visualising and forecasting ICU bed demand. The
app uses fully homomorphic encryption to encrypt all data and forecast
based only on encrypted data.

In this demo application, the ICU bed demand is forecasted during
COVID-19 based on [NHS data].

![4cast]

# Performance

Time Measured for forecasting function with test file with 760 records
without `rustc`'s `release` optimisations.

| Function     | Time(ms) |
| ------------ | -------- |
| `naive_sma`  | 14.6     |
| `SumTreeSMA` | 0.8      |
| `enc_sma`    | 531.8    |

[NHS data]: https://www.england.nhs.uk/statistics/statistical-work-areas/covid-19-hospital-activity/
[4cast]: https://github.com/asher-gh/4cast/assets/74317567/497ed0cf-dcd3-4bb9-9211-e9594d7dd9cf
