use clap::{Parser, Subcommand};
use sqlx::PgPool;

#[derive(Parser)]
#[command(name = "org-cli")]
#[command(about = "Organization tree management CLI tool", long_about = None)]
pub struct Cli {
    /// Database URL (default: DATABASE_URL env var)
    #[arg(long, global = true)]
    pub db_url: Option<String>,

    /// Application ID for multi-tenant isolation
    #[arg(long, global = true)]
    pub appid: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Organization management commands
    #[command(subcommand)]
    Org(OrgCommands),
}

#[derive(Subcommand)]
pub enum OrgCommands {
    /// Initialize root organization
    Init {
        /// Application ID (default: null for platform root)
        #[arg(long)]
        appid: Option<String>,
        /// Organization name (default: "根组织")
        #[arg(long)]
        name: Option<String>,
    },
    /// Show organization subtree
    Show {
        /// Organization ID
        org_id: i64,
        /// Output format: json or tree
        #[arg(long, default_value = "json")]
        format: String,
    },
    /// List all root organizations
    Roots {
        /// Output format: json or table
        #[arg(long, default_value = "json")]
        format: String,
    },
    /// Insert child organization
    Insert {
        /// Parent organization ID
        parent_id: i64,
        /// Organization name
        #[arg(long)]
        name: String,
        /// Organization full name
        #[arg(long)]
        full_name: Option<String>,
        /// Organization code (default: random 8 chars)
        #[arg(long)]
        org_code: Option<String>,
        /// Note
        #[arg(long)]
        note: Option<String>,
        /// Organization type: 0=platform, 1=tenant, 2=branch, 3=department, 4=user
        #[arg(long)]
        org_type: Option<i32>,
    },
    /// Remove organization
    Remove {
        /// Organization ID to remove
        org_id: i64,
        /// Force delete (cascade delete children)
        #[arg(long)]
        force: bool,
    },
    /// Export organization tree to JSON
    Export {
        /// Root organization ID (default: export all)
        #[arg(long)]
        org_id: Option<i64>,
        /// Output file path
        #[arg(long)]
        output: Option<String>,
    },
    /// Import organization tree from JSON
    Import {
        /// JSON file path (default: stdin)
        #[arg(long)]
        file: Option<String>,
        /// Dry run (validate only)
        #[arg(long)]
        dry_run: bool,
    },
}

mod init;
mod show;
mod roots;
mod insert;
mod remove;
mod export;
mod import;

pub async fn run(cli: Cli) -> anyhow::Result<()> {
    let database_url = cli.db_url.unwrap_or_else(|| {
        std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:postgres@127.0.0.1:5432/enrollment".to_string())
    });

    let pool = PgPool::connect(&database_url).await?;

    match cli.command {
        Commands::Org(org_cmd) => {
            handle_org_command(org_cmd, pool).await?;
        }
    }

    Ok(())
}

use OrgCommands::*;

async fn handle_org_command(cmd: OrgCommands, pool: PgPool) -> anyhow::Result<()> {
    match cmd {
        Init { appid, name } => init::handle_init(pool, appid, name).await,
        Show { org_id, format } => show::handle_show(pool, org_id, &format).await,
        Roots { format } => roots::handle_roots(pool, &format).await,
        Insert { parent_id, name, full_name, org_code, note, org_type } => {
            insert::handle_insert(pool, parent_id, name, full_name, org_code, note, org_type).await
        }
        Remove { org_id, force } => remove::handle_remove(pool, org_id, force).await,
        Export { org_id, output } => export::handle_export(pool, org_id, output).await,
        Import { file, dry_run } => import::handle_import(pool, file, dry_run).await,
    }
}
