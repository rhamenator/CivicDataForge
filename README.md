# CivicDataForge

A generic, organization-neutral field/count data validation and profiling tool
inspired by SUFFXX, UMBRELLA, and COUNTCLS. It accepts a documented CSV contract
(`station_id,region,observed_on,direction,count`) and rejects malformed dates,
directions, and identifiers before producing deterministic regional summaries.

No MDOT names, county tables, station data, or agency-specific rules are copied.

```powershell
cargo test
cargo run -- observations.csv
```

Next slices: schema mapping, type inference, DBF/CSV/Parquet adapters, rejection
reports, count-class rules, geospatial joins, and provenance manifests.
