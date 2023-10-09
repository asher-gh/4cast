# About

A cross platform GUI for visualising and forecasting ICU bed demand. The
app uses fully homomorphic encryption to encrypt all data and forecast
based only on encrypted data.

In this demo application, the ICU bed demand is forecasted during
COVID-19 based on [NHS data].

# Performance

Time Measured for forecasting function with test file with 760 records
without `rustc`'s `release` optimisations.

| Function     | Time(ms) |
| ------------ | -------- |
| `naive_sma`  | 15       |
| `SumTreeSMA` | 1        |
| `enc_sma`    | 65400    |

[NHS data]: https://www.england.nhs.uk/statistics/statistical-work-areas/covid-19-hospital-activity/
