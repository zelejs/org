use sqlx::PgPool;
use crate::models::org::UpdateOrgRequest;
use crate::services::OrgService;
use console::Style;

pub async fn handle_update(
    pool: PgPool,
    org_id: i64,
    name: Option<String>,
    full_name: Option<String>,
    org_code: Option<String>,
    note: Option<String>,
    org_type: Option<i32>,
) -> anyhow::Result<()> {
    let bold = Style::new().bold();
    let green = Style::new().green();
    let red = Style::new().red();

    println!("{}", bold.apply_to("Updating organization..."));
    println!("  ID: {}", org_id);
    println!();

    // Show current values
    let service = OrgService::new(pool.clone());
    let current = service.get_org(org_id).await?;

    println!("  Current:");
    println!("    Name: {}", current.name);
    println!("    Full Name: {:?}", current.full_name);
    println!("    Org Code: {:?}", current.org_code);
    println!("    Note: {:?}", current.note);
    println!("    Type: {:?}", current.org_type);
    println!();

    let req = UpdateOrgRequest {
        pid: None,
        name,
        full_name,
        org_code,
        node_level: None,
        left_num: None,
        right_num: None,
        note,
        org_type,
        icon: None,
        level: None,
    };

    match service.update_node(org_id, req).await {
        Ok(updated_id) => {
            println!("{}", green.apply_to("✓ Organization updated successfully!"));
            println!("  Updated ID: {}", updated_id);

            // Show updated values
            let updated = service.get_org(updated_id).await?;
            println!();
            println!("  Updated:");
            println!("    Name: {}", updated.name);
            println!("    Full Name: {:?}", updated.full_name);
            println!("    Org Code: {:?}", updated.org_code);
            println!("    Note: {:?}", updated.note);
            println!("    Type: {:?}", updated.org_type);
        }
        Err(e) => {
            println!("{}", red.apply_to(format!("✗ Error: {}", e)));
            return Err(anyhow::anyhow!(e.to_string()));
        }
    }

    Ok(())
}
