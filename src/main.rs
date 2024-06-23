extern crate odbc;
// Use this crate and set environmet variable RUST_LOG=odbc to see ODBC warnings
extern crate env_logger;
use odbc::*;
use std::io;
use odbc_safe::AutocommitOn;

fn main() {

    env_logger::init();

    match connect() {
        Ok(()) => println!("Success"),
        Err(diag) => println!("Error: {}", diag),
    }
}

fn connect() -> std::result::Result<(), DiagnosticRecord> {

    let env = create_environment_v3().map_err(|e| e.unwrap())?;

    let mut buffer = String::new();
    //println!("Please enter connection string: ");
    //io::stdin().read_line(&mut buffer).unwrap();
    buffer = "Driver={ODBC DRIVER 18 for SQL SERVER}; TrustServerCertificate=yes; Server=163.123.183.80,18501; Database=HEFESTO; UID=eliseu441; PWD=Trembolona550;".parse().unwrap();

    let conn :Connection<AutocommitOn> = env.connect_with_connection_string(&buffer)?;
    execute_statement(&conn)
}
//linha alterada para funcionar build corretamente
fn execute_statement<T: odbc::odbc_safe::AutocommitMode>(conn: &Connection<T>) -> Result<()> {
    let stmt = Statement::with_parent(conn)?;

    let mut sql_text = String::new();
    println!("Please enter SQL statement string: ");
    io::stdin().read_line(&mut sql_text).unwrap();

    match stmt.exec_direct(&sql_text)? {
        Data(mut stmt) => {
            let cols = stmt.num_result_cols()?;
            while let Some(mut cursor) = stmt.fetch()? {
                for i in 1..(cols + 1) {
                    match cursor.get_data::<&str>(i as u16)? {
                        Some(val) => print!(" {}", val),
                        None => print!(" NULL"),
                    }
                }
                println!("");
            }
        }
        NoData(_) => println!("Query executed, no data returned"),
    }

    Ok(())
}