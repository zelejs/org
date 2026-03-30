use sqlx::PgPool;
use crate::models::org::{SysOrgTreeItem, CreateOrgRequest};
use crate::services::{RequestContext, org_core};
use console::Style;
use std::fs;
use std::io::{self, Read};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;

pub async fn handle_import(
    pool: PgPool,
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
        return Ok(());
    }

    // Check if root already exists
    let existing_root = if let Some(appid) = &tree.appid {
        org_core::get_root_org_by_appid(&pool, Some(appid)).await?
    } else {
        org_core::get_root_org_by_appid(&pool, None).await?
    };

    let root_id = if let Some(existing) = existing_root {
        println!("{}", yellow.apply_to("⚠ Root organization already exists"));
        println!("  Existing ID: {}", existing.id);
        println!("  Existing Name: {}", existing.name);
        println!("  Importing as children under existing root...");
        existing.id
    } else {
        // Create new root organization
        let name = tree.name.clone();
        let appid = tree.appid.clone();

        match org_core::init_root_org(&pool, appid, name).await? {
            org_core::RootOrgStatus::Created(org) => {
                println!("{}", green.apply_to("✓ Root organization created"));
                println!("  ID: {}", org.id);
                org.id
            }
            org_core::RootOrgStatus::AlreadyExists(org, _) => {
                println!("{}", yellow.apply_to("⚠ Root organization already exists"));
                println!("  Using existing ID: {}", org.id);
                org.id
            }
        }
    };

    println!();

    // Flatten the tree into a list of (parent_id, child_node) pairs
    let mut import_queue: Vec<(i64, SysOrgTreeItem)> = Vec::new();
    flatten_tree(root_id, &tree, &mut import_queue);

    let total_nodes = import_queue.len();
    if total_nodes > 0 {
        let pb = ProgressBar::new(total_nodes as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"));

        // Track ID mappings for children that have their own children
        let mut id_map: HashMap<i64, i64> = HashMap::new();

        for (parent_id, child) in import_queue {
            let ctx = RequestContext {
                org_id: Some(parent_id),
                tenant_org_id: None,
                appid: tree.appid.clone(),
            };

            let req = CreateOrgRequest {
                name: child.name.clone(),
                full_name: child.full_name.clone(),
                org_code: None, // Will be auto-generated
                note: child.note.clone(),
                org_type: child.org_type,
            };

            let new_id = org_core::insert_child_org(&pool, parent_id, req, &ctx).await?;
            id_map.insert(child.id, new_id);
            pb.inc(1);
        }

        pb.finish_with_message("Import completed");
    }

    println!();
    println!("{}", green.apply_to("✓ Import completed successfully!"));

    // Show the imported tree
    println!();
    println!("{}", bold.apply_to("Imported tree:"));
    let result_tree = org_core::get_subtree(&pool, root_id).await?;
    print_tree_item(&result_tree, 0);

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

fn flatten_tree(parent_id: i64, node: &SysOrgTreeItem, queue: &mut Vec<(i64, SysOrgTreeItem)>) {
    for child in &node.children {
        queue.push((parent_id, child.clone()));
        flatten_tree(child.id, child, queue);
    }
}

fn print_tree_item(item: &SysOrgTreeItem, depth: usize) {
    let indent = "  ".repeat(depth);
    let org_type_label = match item.org_type {
        Some(0) => "Platform",
        Some(1) => "Tenant",
        Some(2) => "Branch",
        Some(3) => "Department",
        Some(4) => "User",
        _ => "Unknown",
    };

    println!("{}[{}] {} - {}", indent, item.id, item.name, org_type_label);

    for child in &item.children {
        print_tree_item(child, depth + 1);
    }
}
