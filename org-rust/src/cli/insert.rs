use sqlx::PgPool;
use crate::models::org::CreateOrgRequest;
use crate::services::{RequestContext, org_core};
use console::Style;

pub async fn handle_insert(
    pool: PgPool,
    parent_id: i64,
    name: String,
    full_name: Option<String>,
    org_code: Option<String>,
    note: Option<String>,
    org_type: Option<i32>,
) -> anyhow::Result<()> {
    let bold = Style::new().bold();
    let green = Style::new().green();
    let red = Style::new().red();

    println!("{}", bold.apply_to("Inserting child organization..."));
    println!("  Parent ID: {}", parent_id);
    println!("  Name: {}", name);
    println!("  Full Name: {:?}", full_name);
    println!("  Org Code: {:?}", org_code);
    println!("  Note: {:?}", note);
    println!("  Type: {:?}", org_type);
    println!();

    let ctx = RequestContext {
        org_id: Some(parent_id),
        tenant_org_id: None,
        appid: None,
    };

    let req = CreateOrgRequest {
        name,
        full_name,
        org_code,
        note,
        org_type,
        icon: None,
        level: None,
    };

    match org_core::insert_child_org(&pool, parent_id, req, &ctx).await {
        Ok(new_id) => {
            println!("{}", green.apply_to("✓ Child organization created successfully!"));
            println!("  New ID: {}", new_id);
        }
        Err(e) => {
            println!("{}", red.apply_to(format!("✗ Error: {}", e)));
            return Err(anyhow::anyhow!(e.to_string()));
        }
    }

    Ok(())
}
