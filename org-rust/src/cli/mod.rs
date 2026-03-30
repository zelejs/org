use clap::{Parser, Subcommand};
use sqlx::PgPool;

#[derive(Parser)]
#[command(name = "org-cli")]
#[command(about = "Organization tree management CLI tool")]
#[command(long_about = "
A CLI tool for managing organization trees using the Nested Set Model.

This tool provides commands for:
  - Initializing and managing organization hierarchies
  - Importing/exporting organization data
  - Validating tree integrity
  - Searching and querying organizations

Environment Variables:
  DATABASE_URL    PostgreSQL connection string (default: postgresql://postgres:postgres@127.0.0.1:5432/enrollment)
  RUST_LOG        Log level (default: org_rust=info)

Examples:
  # Show all root organizations
  org-cli org roots

  # Show organization tree as JSON
  org-cli org show 1 --format json

  # Insert a new organization
  org-cli org insert 1 --name 'Engineering' --org-type 3

  # Search organizations
  org-cli org search 'Engineering' --format table

  # Export organization tree
  org-cli org export --org-id 1 --output org-tree.json

  # Import organization tree
  org-cli org import --file org-tree.json
")]
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
    ///
    /// All organization operations require a valid database connection.
    /// Use --db-url or set DATABASE_URL environment variable.
    #[command(subcommand)]
    Org(OrgCommands),
}

#[derive(Subcommand)]
pub enum OrgCommands {
    /// Initialize root organization
    ///
    /// Creates a new root organization node. This is typically used when
    /// setting up a new application tenant.
    ///
    /// Examples:
    ///   org-cli org init --appid 'app-001' --name 'Acme Corp'
    ///   org-cli org init  # Creates platform root with default name
    Init {
        /// Application ID (default: null for platform root)
        #[arg(long)]
        appid: Option<String>,
        /// Organization name (default: "根组织")
        #[arg(long)]
        name: Option<String>,
    },
    /// Show organization subtree
    ///
    /// Display the organization tree starting from the specified node.
    ///
    /// Examples:
    ///   org-cli org show 1 --format json
    ///   org-cli org show 1 --format tree
    ///   org-cli org show 1 --appid 'app-001' --format json
    Show {
        /// Organization ID
        org_id: i64,
        /// Output format: json or tree
        #[arg(long, default_value = "json")]
        format: String,
        /// Application ID to filter organizations
        #[arg(long)]
        appid: Option<String>,
    },
    /// List all root organizations
    ///
    /// Display all root-level organizations (organizations without parents).
    ///
    /// Examples:
    ///   org-cli org roots --format table
    ///   org-cli org roots --format json
    Roots {
        /// Output format: json or table
        #[arg(long, default_value = "json")]
        format: String,
    },
    /// Insert child organization
    ///
    /// Create a new organization as a child of the specified parent.
    ///
    /// Organization types:
    ///   0 = Platform (system-level)
    ///   1 = Tenant (application-level root)
    ///   2 = Branch (company branch/office)
    ///   3 = Department (functional department)
    ///   4 = User (individual user)
    ///
    /// Examples:
    ///   org-cli org insert 1 --name 'Engineering' --full-name 'Engineering Dept' --org-type 3
    ///   org-cli org insert 1 --name 'R&D' --org-type 3 --note 'Research and Development'
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
    ///
    /// Delete an organization node. Without --force, only leaf nodes can be deleted.
    /// With --force, all descendants will be cascade deleted.
    ///
    /// WARNING: Using --force will permanently delete the entire subtree!
    ///
    /// Examples:
    ///   org-cli org remove 123          # Remove leaf only
    ///   org-cli org remove 123 --force  # Remove subtree
    Remove {
        /// Organization ID to remove
        org_id: i64,
        /// Force delete (cascade delete children)
        #[arg(long)]
        force: bool,
    },
    /// Export organization tree to JSON
    ///
    /// Export the organization tree structure to a JSON file.
    ///
    /// Examples:
    ///   org-cli org export --org-id 1 --output org-tree.json
    ///   org-cli org export --output all-orgs.json  # Export all
    Export {
        /// Root organization ID (default: export all)
        #[arg(long)]
        org_id: Option<i64>,
        /// Output file path
        #[arg(long)]
        output: Option<String>,
    },
    /// Import organization tree from JSON
    ///
    /// Import organization data from a JSON file (format matching export output).
    /// Use --dry-run to validate without making changes.
    ///
    /// Examples:
    ///   org-cli org import --file org-tree.json
    ///   org-cli org import --file org-tree.json --dry-run
    ///   cat org-tree.json | org-cli org import
    Import {
        /// JSON file path (default: stdin)
        #[arg(long)]
        file: Option<String>,
        /// Dry run (validate only)
        #[arg(long)]
        dry_run: bool,
    },
    /// Update organization properties
    ///
    /// Update fields of an existing organization. Only specified fields will be updated.
    ///
    /// Examples:
    ///   org-cli org update 123 --name 'New Name'
    ///   org-cli org update 123 --full-name 'New Full Name' --note 'Updated note'
    Update {
        /// Organization ID to update
        org_id: i64,
        /// Organization name
        #[arg(long)]
        name: Option<String>,
        /// Organization full name
        #[arg(long)]
        full_name: Option<String>,
        /// Organization code
        #[arg(long)]
        org_code: Option<String>,
        /// Note
        #[arg(long)]
        note: Option<String>,
        /// Organization type: 0=platform, 1=tenant, 2=branch, 3=department, 4=user
        #[arg(long)]
        org_type: Option<i32>,
    },
    /// Search organizations
    ///
    /// Search for organizations by name or organization code.
    ///
    /// Examples:
    ///   org-cli org search 'Engineering'
    ///   org-cli org search 'ENG' --format table --limit 10
    Search {
        /// Search term (name or org_code)
        term: String,
        /// Output format: json or table
        #[arg(long, default_value = "table")]
        format: String,
        /// Limit results
        #[arg(long, default_value = "20")]
        limit: i64,
    },
    /// Validate tree integrity
    ///
    /// Check the Nested Set Model integrity of the organization tree.
    /// Reports inconsistencies in left_num/right_num values.
    ///
    /// Examples:
    ///   org-cli org validate           # Validate all trees
    ///   org-cli org validate --org-id 1  # Validate specific tree
    Validate {
        /// Organization ID to validate (default: validate all)
        org_id: Option<i64>,
    },
    /// Move subtree to new parent
    ///
    /// Move an organization subtree to a new parent. This updates the entire
    /// subtree's left_num/right_num values to maintain tree integrity.
    ///
    /// WARNING: You cannot move a subtree into its own descendants!
    ///
    /// Examples:
    ///   org-cli org move 123 456  # Move org 123 under org 456
    Move {
        /// Organization ID to move
        org_id: i64,
        /// New parent organization ID
        new_parent_id: i64,
    },
}

mod init;
mod show;
mod roots;
mod insert;
mod remove;
mod export;
mod import;
mod update;
mod search;
mod validate;
mod r#move;

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
        Show { org_id, format, appid } => show::handle_show(pool, org_id, &format, appid).await,
        Roots { format } => roots::handle_roots(pool, &format).await,
        Insert { parent_id, name, full_name, org_code, note, org_type } => {
            insert::handle_insert(pool, parent_id, name, full_name, org_code, note, org_type).await
        }
        Remove { org_id, force } => remove::handle_remove(pool, org_id, force).await,
        Export { org_id, output } => export::handle_export(pool, org_id, output).await,
        Import { file, dry_run } => import::handle_import(pool, file, dry_run).await,
        Update { org_id, name, full_name, org_code, note, org_type } => {
            update::handle_update(pool, org_id, name, full_name, org_code, note, org_type).await
        }
        Search { term, format, limit } => search::handle_search(pool, &term, &format, limit).await,
        Validate { org_id } => validate::handle_validate(pool, org_id).await,
        Move { org_id, new_parent_id } => r#move::handle_move(pool, org_id, new_parent_id).await,
    }
}
