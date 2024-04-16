use sqlx::{PgPool, Row};
use colored::*;






pub async fn db_table_check(pool: &PgPool) -> Result<(), sqlx::Error> {

    println!("{}", "Starting DB Table Check...".purple().bold().underline());

    let table_names = vec!["user_data", "user_twitch_credentials"]; // List of tables to check
    let schema_name = "public"; // Schema name

    let query = format!(
        "SELECT tablename
         FROM pg_tables
         WHERE schemaname = $1 AND tablename = ANY($2);",
    );

    let results = sqlx::query(&query)
        .bind(schema_name)
        .bind(&table_names)
        .fetch_all(pool)
        .await?;

    // Creating a set of existing table names returned from the query
    let existing_tables: std::collections::HashSet<&str> = results
        .iter()
        .map(|row| row.get::<&str, _>("tablename"))
        .collect();

    // Checking which tables exist and which do not
    for table in &table_names {
        if existing_tables.contains(table) {
            println!("{} {}", "Table Exists: ".green().underline(), table)
        } else {
            println!("{} {}", "Table DOES NOT Exist: ".red().underline(), table)

        }
    }

    Ok(())
}