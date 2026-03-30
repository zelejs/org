use sqlx::PgPool;
use console::Style;

pub async fn handle_search(
    pool: PgPool,
    term: &str,
    format: &str,
    limit: i64,
) -> anyhow::Result<()> {
    let bold = Style::new().bold();
    let cyan = Style::new().cyan();
    let dim = Style::new().dim();

    println!("{}", bold.apply_to("Searching organizations..."));
    println!("  Term: {}", term);
    println!("  Limit: {}", limit);
    println!();

    // Search by name or org_code
    let results = sqlx::query_as::<_, crate::models::org::SysOrg>(
        r#"
        SELECT * FROM t_sys_org
        WHERE delete_flag = 0
        AND (name ILIKE $1 OR org_code ILIKE $1)
        ORDER BY node_level ASC
        LIMIT $2
        "#
    )
    .bind(format!("%{}%", term))
    .bind(limit)
    .fetch_all(&pool)
    .await?;

    if results.is_empty() {
        println!("No organizations found matching '{}'", term);
        return Ok(());
    }

    match format {
        "table" => {
            // Calculate column widths
            let max_id_len = results.iter().map(|r| r.id.to_string().len()).max().unwrap_or(4);
            let max_name_len = results.iter().map(|r| r.name.len()).max().unwrap_or(4);
            let max_code_len = results.iter().map(|r| r.org_code.as_deref().unwrap_or("NULL").len()).max().unwrap_or(6);
            let max_appid_len = results.iter().map(|r| r.appid.as_deref().unwrap_or("NULL").len()).max().unwrap_or(6);
            let max_level_len = 5; // "Level"

            // Header
            println!("{:<id_width$} | {:<name_width$} | {:<code_width$} | {:<level_width$} | {:<appid_width$}",
                bold.apply_to("ID"),
                bold.apply_to("Name"),
                bold.apply_to("Code"),
                bold.apply_to("Level"),
                bold.apply_to("AppID"),
                id_width = max_id_len,
                name_width = max_name_len,
                code_width = max_code_len,
                level_width = max_level_len,
                appid_width = max_appid_len,
            );

            // Separator
            println!("{}", "-".repeat(max_id_len + max_name_len + max_code_len + max_level_len + max_appid_len + 14));

            // Rows
            for org in &results {
                println!("{:<id_width$} | {:<name_width$} | {:<code_width$} | {:<level_width$} | {:<appid_width$}",
                    org.id,
                    cyan.apply_to(&org.name),
                    org.org_code.as_deref().unwrap_or("NULL"),
                    org.node_level.unwrap_or(0),
                    dim.apply_to(org.appid.as_deref().unwrap_or("NULL")),
                    id_width = max_id_len,
                    name_width = max_name_len,
                    code_width = max_code_len,
                    level_width = max_level_len,
                    appid_width = max_appid_len,
                );
            }

            println!();
            println!("Total: {} result(s)", results.len());
        }
        "json" | _ => {
            println!("{}", serde_json::to_string_pretty(&results)?);
        }
    }

    Ok(())
}
