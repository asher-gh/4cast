# About

A cross platform GUI for visualising and forecasting ICU bed demand. The
app uses fully homomorphic encryption to encrypt all data and forecast
based only on encrypted data.

The ICU bed data is taken from the publicly available COVID-19 [NHS data].

![app_gif]

# Performance

Time taken by forecasting function for 761 records.

| Function         | Time(ms) |
| ---------------- | -------- |
| `naive_sma`      | 14.6     |
| `SumTreeSMA`     | 0.8      |
| `enc_sma`        | 26550    |
| `enc_sma_cached` | 25300    |

[NHS data]: https://www.england.nhs.uk/statistics/statistical-work-areas/covid-19-hospital-activity/
[app_gif]: https://github.com/asher-gh/4cast/assets/74317567/497ed0cf-dcd3-4bb9-9211-e9594d7dd9cf
