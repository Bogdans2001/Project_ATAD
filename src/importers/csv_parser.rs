use csv::Reader;
use chrono::NaiveDate;
use anyhow::anyhow;
use std::path::Path;

fn validate_row(row: &Vec<String>) -> anyhow::Result<()> {
    if row.len() != 5 {
        return Err(anyhow!("Invalid number of columns".to_string()));
    }
    if !NaiveDate::parse_from_str(&row[0], "%Y-%m-%d").is_ok() {
        return Err(anyhow!(format!("Invalid date: {}", row[0])));
    }
    if !row[1].parse::<f64>().is_ok() {
        return Err(anyhow!(format!("Invalid amount: {}", row[2])));
    }
    if row[2]!="income" && row[2]!="expense" {
        return Err(anyhow!(format!("Invalid kind: {}",row[2])));
    }
    if !row[3].parse::<i64>().is_ok() {
        return Err(anyhow!(format!("Invalid category: {}", row[3])));
    }
    if row[4] == "" {
        return Err(anyhow!("Empty description".to_string()));
    }
    Ok(())
}


pub fn read_csv_as_arrays(csv_path: &str) -> anyhow::Result<Vec<Vec<String>>> {
    if !Path::new(csv_path).exists() {
        return Err(anyhow!("Wrong path: {}", csv_path));
    }
    let mut rdr = Reader::from_path(csv_path)?;
    let mut rows = Vec::new();

    for rec in rdr.records() {
        let mut row:Vec<String> = Vec::new();
        let record = rec?;
        for field in record.iter() {
            row.push(field.to_string());
        }
        match validate_row(&row){
            Ok(()) => {
                rows.push(row);
            }
            Err(e) =>{
                return Err(e);
            }
        }
    }

    Ok(rows)
}