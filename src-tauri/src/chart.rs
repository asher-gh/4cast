#[derive(Debug, Clone, Default)]
pub struct ChartData {
    pub dates: Vec<String>,
    pub beds_actual: Vec<u32>,
    pub beds_forecast: Vec<u32>,
}