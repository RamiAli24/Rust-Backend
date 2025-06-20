#![allow(missing_docs)]
use forge_api_web::{init_tracing, run};
use std::process::ExitCode;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tokio::main]
async fn main() -> ExitCode {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    init_tracing();

    match run().await {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            tracing::error!(
                error.msg = %e,
                error.error_chain = ?e,
                "Shutting down due to error"
            );
            ExitCode::FAILURE
        }
    }
}
