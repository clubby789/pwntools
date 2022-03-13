//! A `tracing_subscriber` log formatter
//! [`init_logger`] **must** be called in order to use this!

use colored::Colorize;
pub use tracing::{debug, error, info, warn};
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;

struct PwnFormatter;
impl<S, N> FormatEvent<S, N> for PwnFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let meta = event.metadata();
        let log_char = match *meta.level() {
            Level::DEBUG => "|".purple(),
            Level::INFO => "*".blue(),
            Level::WARN => "!".yellow(),
            _ => "X".red(),
        };
        write!(&mut writer, "[{}] {}: ", log_char, meta.target().green())?;
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

/// Initialise the logging subscriber
pub fn init_logger() {
    tracing_subscriber::fmt()
        .event_format(PwnFormatter {})
        .init();
}
