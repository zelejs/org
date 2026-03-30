use sqlx::PgPool;
use crate::services::org_core;
use console::Style;

pub async fn handle_roots(
    pool: PgPool,
    format: &str,
) -> anyhow::Result<()> {
    let bold = Style::new().bold();
    let cyan = Style::new().cyan();

    let roots = org_core::list_all_root_orgs(&pool).await?;

    match format {
        "table" => {
            if roots.is_empty() {
                println!("No root organizations found.");
                return Ok(());
            }

            // Calculate column widths
            let max_id_len = roots.iter().map(|r| r.org_id.to_string().len()).max().unwrap_or(4);
            let max_appid_len = roots.iter().map(|r| r.appid.as_deref().unwrap_or("NULL").len()).max().unwrap_or(6);
            let max_name_len = roots.iter().map(|r| r.name.len()).max().unwrap_or(4);
            let max_type_len = roots.iter().map(|r| org_type_label(r.org_type).len()).max().unwrap_or(8);
            let max_count_len = roots.iter().map(|r| r.children_count.to_string().len()).max().unwrap_or(13);

            // Header
            println!("{:<id_width$} | {:<appid_width$} | {:<name_width$} | {:<type_width$} | {:<count_width$}",
                bold.apply_to("ID"),
                bold.apply_to("AppID"),
                bold.apply_to("Name"),
                bold.apply_to("Type"),
                bold.apply_to("Children"),
                id_width = max_id_len,
                appid_width = max_appid_len,
                name_width = max_name_len,
                type_width = max_type_len,
                count_width = max_count_len,
            );

            // Separator
            println!("{}", "-".repeat(max_id_len + max_appid_len + max_name_len + max_type_len + max_count_len + 14));

            // Rows
            for root in &roots {
                println!("{:<id_width$} | {:<appid_width$} | {:<name_width$} | {:<type_width$} | {:<count_width$}",
                    root.org_id,
                    root.appid.as_deref().unwrap_or("NULL"),
                    cyan.apply_to(&root.name),
                    org_type_label(root.org_type),
                    root.children_count,
                    id_width = max_id_len,
                    appid_width = max_appid_len,
                    name_width = max_name_len,
                    type_width = max_type_len,
                    count_width = max_count_len,
                );
            }

            println!();
            println!("Total: {} root organization(s)", roots.len());
        }
        "json" | _ => {
            println!("{}", serde_json::to_string_pretty(&roots)?);
        }
    }

    Ok(())
}

fn org_type_label(org_type: Option<i32>) -> String {
    match org_type {
        Some(0) => "Platform",
        Some(1) => "Tenant",
        Some(2) => "Branch",
        Some(3) => "Department",
        Some(4) => "User",
        _ => "Unknown",
    }.to_string()
}
