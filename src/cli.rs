use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the daemon to listen for Niri events
    Daemon,
    /// Start the Gutter Bar view (GUI)
    View {
        /// Force a specific view mode (default, active, divider, sidebar)
        #[arg(long)]
        mode: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_daemon() {
        let cli = Cli::parse_from(&["gutter-bar", "daemon"]);
        match cli.command {
            Commands::Daemon => {}, // Success
            _ => panic!("Expected Daemon command"),
        }
    }

    #[test]
    fn test_cli_view() {
        let cli = Cli::parse_from(&["gutter-bar", "view"]);
        match cli.command {
            Commands::View { mode } => assert_eq!(mode, None),
            _ => panic!("Expected View command"),
        }
    }

    #[test]
    fn test_cli_view_mode() {
        let cli = Cli::parse_from(&["gutter-bar", "view", "--mode", "active"]);
        match cli.command {
            Commands::View { mode } => assert_eq!(mode.as_deref(), Some("active")),
            _ => panic!("Expected View command"),
        }
    }
}
