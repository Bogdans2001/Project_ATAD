use std::process::Command;

#[test]
fn test_report_category_correct() {
    let output = Command::new(env!("CARGO_BIN_EXE_finance_app")).args(["report", "--property", "category"]).output()
        .expect("Command not executed");
    assert!(output.status.success(),"Command not found. stderr: {}", String::from_utf8_lossy(&output.stderr));
}


#[test]
fn test_report_category_wrong() {
    let output = Command::new(env!("CARGO_BIN_EXE_finance_app")).args(["report", "--option", "category"]).output()
        .expect("Command not executed");
    assert!(!output.status.success() , "The command was invalid");
}

#[test]
fn test_report_monthly_correct() {
    let output = Command::new(env!("CARGO_BIN_EXE_finance_app")).args(["report", "--property", "monthly"]).output()
        .expect("Command not executed");
    assert!(output.status.success(),"Command not found. stderr: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn test_report_monthly_wrong() {
    let output = Command::new(env!("CARGO_BIN_EXE_finance_app")).args(["report", "--option", "monthly"]).output()
        .expect("Command not executed");
    assert!(!output.status.success() , "The command was invalid");
}

#[test]
fn test_search_command(){
    let output = Command::new(env!("CARGO_BIN_EXE_finance_app")).args(["search", "--date", "2003-12-12"]).output()
        .expect("Command not executed");
    assert!(output.status.success(),"Command not found. stderr: {}", String::from_utf8_lossy(&output.stderr));
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No transactions found"),"Expected 'category' in output, got: {stdout}");
}

#[test]
fn test_search_command_wrong() {
    let output = Command::new(env!("CARGO_BIN_EXE_finance_app")).args(["search", "--date", "2003-13-13"]).output()
        .expect("Command not executed");
    
    assert!(!output.status.success() , "The date was invalid");
}

#[test]
fn test_add_command_wrong() {
    let output = Command::new(env!("CARGO_BIN_EXE_finance_app")).args(["add", "--kind", "expense", "--amount",
        "18", "--category-id", "0", "--date", "2025-09-33", "--description", "diesel"]).output()
        .expect("Command not executed");
    
    assert!(!output.status.success() , "The date was invalid");
}

#[test]
fn test_budget_command(){
    let output = Command::new(env!("CARGO_BIN_EXE_finance_app")).args(["--category-id", "54", "--month", "2025-09", 
        "--amount", "35"]).output().expect("Command not executed");
    assert!(!output.status.success() , "The category id was invalid");
}