use sqlx::PgPool;
use crate::services::org_core;
use console::Style;

pub async fn handle_remove(
    pool: PgPool,
    org_id: i64,
    force: bool,
) -> anyhow::Result<()> {
    let bold = Style::new().bold();
    let green = Style::new().green();
    let red = Style::new().red();
    let yellow = Style::new().yellow();

    println!("{}", bold.apply_to("Removing organization..."));
    println!("  ID: {}", org_id);
    println!("  Force: {}", force);
    println!();

    // First, check if the org exists and show info
    if let Ok(Some(org)) = org_core::get_by_id(&pool, org_id).await {
        println!("  Target: {} ({:?})", org.name, org.org_type);
        println!();
    }

    match org_core::remove_org(&pool, org_id, force, None).await {
        Ok(removed_id) => {
            println!("{}", green.apply_to("✓ Organization removed successfully!"));
            println!("  Removed ID: {}", removed_id);
        }
        Err(e) => {
            println!("{}", red.apply_to(format!("✗ Error: {}", e)));
            if e.to_string().contains("cascade") {
                println!();
                println!("{}", yellow.apply_to("Hint: Use --force to delete with children"));
            }
            return Err(anyhow::anyhow!(e.to_string()));
        }
    }

    Ok(())
}
