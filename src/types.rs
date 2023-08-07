use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StockData {
    #[serde(rename = "Meta Data")]
    pub meta_data: StockMetaData,

    #[serde(rename = "Time Series (5min)")]
    pub time_series: std::collections::HashMap<String, TimeSeriesData>,
}

#[derive(Deserialize, Debug)]
pub struct StockMetaData {

    #[serde(rename = "3. Last Refreshed")]
    pub last_refreshed: String,

}

#[derive(Deserialize, Debug)]
pub struct TimeSeriesData {

    #[serde(rename = "4. close")]
    pub close: String,
}
