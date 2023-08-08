use tchatchers_core::report::Report;

use crate::errors::CliError;

/// A struct representing an action related to reporting.
pub struct ReportAction;

impl ReportAction {
    /// Checks the latest reports.
    ///
    /// Retrieves the latest reports from the database and prints them to the console.
    /// Returns an `Ok` result if the operation succeeds, or a `CliError` if an error occurs.
    pub async fn check_latest_reports() -> Result<(), CliError> {
        let pool = tchatchers_core::pool::get_pg_pool().await?;
        let reports = Report::get_all(&pool).await?;
        println!("Reports: {reports:#?}");
        Ok(())
    }
}
