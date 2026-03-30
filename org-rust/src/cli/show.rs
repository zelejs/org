use sqlx::PgPool;
use crate::services::org_core;
use console::Style;

pub async fn handle_show(
    pool: PgPool,
    org_id: i64,
    format: &str,
    appid: Option<String>,
) -> anyhow::Result<()> {
    match format {
        "tree" => {
            let tree = org_core::get_subtree_with_appid(&pool, org_id, appid.as_deref()).await?;
            print_tree(&tree, 0, true);
        }
        "json" | _ => {
            let tree = org_core::get_subtree_with_appid(&pool, org_id, appid.as_deref()).await?;
            println!("{}", serde_json::to_string_pretty(&tree)?);
        }
    }

    Ok(())
}

fn print_tree(item: &crate::models::org::SysOrgTreeItem, depth: usize, is_last: bool) {
    let cyan = Style::new().cyan();
    let dim = Style::new().dim();

    let prefix = if depth == 0 {
        "".to_string()
    } else {
        let mut p = String::new();
        for _i in 0..depth-1 {
            p.push_str("│   ");
        }
        p.push_str(if is_last { "└── " } else { "├── " });
        p
    };

    let org_type_label = match item.org_type {
        Some(0) => "Platform",
        Some(1) => "Tenant",
        Some(2) => "Branch",
        Some(3) => "Department",
        Some(4) => "User",
        _ => "Unknown",
    };

    println!("{}[{}] {} {} - {}", prefix, item.id, cyan.apply_to(&item.name), dim.apply_to(format!("({})", org_type_label)), dim.apply_to(format!("{:?}", item.appid)));

    let children_count = item.children.len();
    for (i, child) in item.children.iter().enumerate() {
        print_tree(child, depth + 1, i == children_count - 1);
    }
}
