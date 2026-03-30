use sqlx::PgPool;
use crate::models::org::SysOrgTreeItem;
use console::Style;
use std::fs;
use std::io::{self, Read};

pub async fn handle_import(
    _pool: PgPool,
    file: Option<String>,
    dry_run: bool,
) -> anyhow::Result<()> {
    let bold = Style::new().bold();
    let green = Style::new().green();
    let yellow = Style::new().yellow();

    println!("{}", bold.apply_to("Importing organization tree..."));

    let json = if let Some(path) = file {
        println!("  Reading from: {}", path);
        fs::read_to_string(&path)?
    } else {
        println!("  Reading from stdin...");
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        input
    };

    if dry_run {
        println!();
        println!("{}", yellow.apply_to("⚠ DRY RUN MODE - No changes will be made"));
        println!();
    }

    // Parse JSON
    let tree: SysOrgTreeItem = serde_json::from_str(&json)?;

    println!("  Parsed organization: {}", tree.name);
    println!("  Tree depth: {}", max_depth(&tree));
    println!("  Total nodes: {}", count_nodes(&tree));
    println!();

    if dry_run {
        println!("{}", green.apply_to("✓ Validation passed!"));
        println!("  Run without --dry-run to execute import.");
        println!();
        println!("Note: Import functionality is not fully implemented yet.");
        println!("The data structure has been validated successfully.");
    } else {
        println!("{}", yellow.apply_to("Note: Import functionality is not fully implemented yet."));
        println!("Please use the --dry-run flag to validate the JSON structure.");
    }

    Ok(())
}

fn max_depth(item: &SysOrgTreeItem) -> usize {
    if item.children.is_empty() {
        1
    } else {
        1 + item.children.iter().map(max_depth).max().unwrap_or(0)
    }
}

fn count_nodes(item: &SysOrgTreeItem) -> usize {
    1 + item.children.iter().map(count_nodes).sum::<usize>()
}
