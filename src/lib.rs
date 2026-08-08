use serde::Deserialize;
use std::{collections::BTreeMap, error::Error, path::Path};

#[derive(Debug, Clone, Deserialize)]
pub struct Observation {
    pub station_id: String,
    pub region: String,
    pub observed_on: String,
    pub direction: String,
    pub count: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegionSummary {
    pub observations: usize,
    pub total_count: u64,
    pub peak_count: u64,
    pub average_count: f64,
}

pub fn load(path: impl AsRef<Path>) -> Result<Vec<Observation>, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(path)?;
    let observations: Vec<Observation> = reader.deserialize().collect::<Result<_, _>>()?;
    validate(&observations)?;
    Ok(observations)
}

pub fn validate(observations: &[Observation]) -> Result<(), String> {
    for (index, item) in observations.iter().enumerate() {
        if item.station_id.trim().is_empty() || item.region.trim().is_empty() {
            return Err(format!("row {} requires station_id and region", index + 2));
        }
        if !matches!(
            item.direction.trim().to_ascii_uppercase().as_str(),
            "N" | "NE" | "E" | "SE" | "S" | "SW" | "W" | "NW" | "BOTH"
        ) {
            return Err(format!("row {} has invalid direction", index + 2));
        }
        if !is_iso_date(&item.observed_on) {
            return Err(format!("row {} requires YYYY-MM-DD observed_on", index + 2));
        }
    }
    Ok(())
}

pub fn summarize(observations: &[Observation]) -> BTreeMap<String, RegionSummary> {
    let mut grouped: BTreeMap<String, Vec<u64>> = BTreeMap::new();
    for item in observations {
        grouped
            .entry(item.region.clone())
            .or_default()
            .push(item.count);
    }
    grouped
        .into_iter()
        .map(|(region, counts)| {
            let total_count = counts.iter().sum();
            let observations = counts.len();
            (
                region,
                RegionSummary {
                    observations,
                    total_count,
                    peak_count: counts.into_iter().max().unwrap_or_default(),
                    average_count: total_count as f64 / observations as f64,
                },
            )
        })
        .collect()
}

fn is_iso_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| index == 4 || index == 7 || byte.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_profile_is_deterministic() {
        let rows = vec![
            Observation {
                station_id: "A".into(),
                region: "North".into(),
                observed_on: "2026-01-01".into(),
                direction: "N".into(),
                count: 10,
            },
            Observation {
                station_id: "B".into(),
                region: "North".into(),
                observed_on: "2026-01-02".into(),
                direction: "S".into(),
                count: 30,
            },
        ];
        validate(&rows).unwrap();
        let summary = summarize(&rows);
        assert_eq!(summary["North"].peak_count, 30);
        assert_eq!(summary["North"].average_count, 20.0);
    }
}
