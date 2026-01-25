use anyhow::{Context,anyhow};
use std::fs::read_to_string;
use quick_xml::de::from_str;
use std::path::Path;
use crate::importers::csv_parser::validate_row;
use serde::Deserialize;
use chrono::NaiveDate;


#[derive(Debug, Deserialize)]
pub struct Ofx {
    #[serde(rename = "BANKMSGSRSV1")]
    pub bank: BankMsgs,
}

#[derive(Debug, Deserialize)]
pub struct BankMsgs {
    #[serde(rename = "STMTTRNRS")]
    pub stmt: StmtTrnRs,
}

#[derive(Debug, Deserialize)]
pub struct StmtTrnRs {
    #[serde(rename = "STMTRS")]
    pub stmtrs: StmtRs,
}

#[derive(Debug, Deserialize)]
pub struct StmtRs {
    #[serde(rename = "BANKTRANLIST")]
    pub banktranlist: BankTranList,
}

#[derive(Debug, Deserialize)]
pub struct BankTranList {
    #[serde(rename = "STMTTRN", default)]
    pub transactions: Vec<StmtTrn>,
}

#[derive(Debug, Deserialize)]
pub struct StmtTrn {
    #[serde(rename = "DTPOSTED")]
    pub dtposted: String,

    #[serde(rename = "TRNAMT")]
    pub trnamt: String,

    #[serde(rename = "NAME")]
    pub name: String,
}


pub fn read_ofx_as_arrays(ofx_path: &str) -> anyhow::Result<Vec<Vec<String>>> {
    if !Path::new(ofx_path).exists() {
        return Err(anyhow!("Wrong path: {}", ofx_path));
    }
    let file_content = read_to_string(ofx_path).with_context(|| format!("File cannot be read {}", ofx_path))?;
    let start_xml_information = file_content.find("<OFX>").context("Please provide an OFX file")?;
    let xml_data = &file_content[start_xml_information..];
    let  ofx_data:Ofx = from_str(xml_data).context("The XML component could not be parsed")?;

    let mut result = Vec::new();

    for transaction in ofx_data.bank.stmt.stmtrs.banktranlist.transactions {
         let mut result_field:Vec<String> = Vec::new();

        let date = NaiveDate::parse_from_str(&transaction.dtposted[..8], "%Y%m%d").map_err(|_| anyhow!(format!("Invalid date")))?;
        result_field.push(date.to_string());

        let mut amount: String = transaction.trnamt;
        let amount_number:f64 = amount.parse()?;
        if amount_number < 0.0 {
            let amount_positive = -(amount_number);
            amount = amount_positive.to_string();
        }
        result_field.push(amount);
        
        let mut kind = "expense";
        if amount_number >= 0.0 {
            kind = "income";
        }
        result_field.push(kind.to_string());

        result_field.push("0".to_string());

        result_field.push(transaction.name);

         match validate_row(&result_field){
            Ok(()) => {
                result.push(result_field);
            }
            Err(e) =>{
                return Err(e);
            }
        }

    }
    Ok(result)
}