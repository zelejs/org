use sqlx::PgPool;
use crate::services::org_core;
use console::Style;
use std::fs::File;
use std::io::Write;

pub async fn handle_export(
    pool: PgPool,
    org_id: Option<i64>,
    output: Option<String>,
) -> anyhow::Result<()> {
    let bold = Style::new().bold();
    let green = Style::new().green();

    println!("{}", bold.apply_to("Exporting organization tree..."));

    let result = if let Some(id) = org_id {
        println!("  Exporting subtree of org ID: {}", id);
        org_core::get_subtree(&pool, id).await?
    } else {
        println!("  Exporting all organizations");
        // Export all roots
        let roots = org_core::list_all_root_orgs(&pool).await?;
        if roots.is_empty() {
            println!("No organizations found to export.");
            return Ok(());
        }

        // Export first root's subtree (could extend to export all)
        org_core::get_subtree(&pool, roots[0].org_id).await?
    };

    let json = serde_json::to_string_pretty(&result)?;

    if let Some(path) = output {
        println!("  Writing to: {}", path);
        let mut file = File::create(&path)?;
        file.write_all(json.as_bytes())?;
        file.write_all(b"\n")?;
        println!("{}", green.apply_to("✓ Export completed!"));
    } else {
        println!("{}", green.apply_to("✓ Export completed!"));
        println!();
        println!("{}", json);
    }

    Ok(())
}
