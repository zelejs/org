use sqlx::PgPool;
use crate::services::org_core;
use console::Style;

pub async fn handle_init(
    pool: PgPool,
    appid: Option<String>,
    name: Option<String>,
) -> anyhow::Result<()> {
    let name = name.unwrap_or_else(|| "根组织".to_string());
    let bold = Style::new().bold();
    let green = Style::new().green();
    let yellow = Style::new().yellow();

    println!("{}", bold.apply_to("Initializing root organization..."));
    println!("  AppID: {:?}", appid);
    println!("  Name: {}", name);
    println!();

    match org_core::init_root_org(&pool, appid.clone(), name).await? {
        org_core::RootOrgStatus::Created(org) => {
            println!("{}", green.apply_to("✓ Root organization created successfully!"));
            println!("  ID: {}", org.id);
            println!("  Name: {}", org.name);
            println!("  AppID: {:?}", org.appid);
            println!("  Left: {:?}", org.left_num);
            println!("  Right: {:?}", org.right_num);
        }
        org_core::RootOrgStatus::AlreadyExists(org, count) => {
            println!("{}", yellow.apply_to("⚠ Root organization already exists"));
            println!("  ID: {}", org.id);
            println!("  Name: {}", org.name);
            println!("  AppID: {:?}", org.appid);
            println!("  Children count: {}", count);
        }
    }

    Ok(())
}
