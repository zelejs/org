use sqlx::PgPool;
use crate::services::org_core;
use console::Style;

pub async fn handle_validate(
    pool: PgPool,
    org_id: Option<i64>,
) -> anyhow::Result<()> {
    let bold = Style::new().bold();
    let green = Style::new().green();
    let yellow = Style::new().yellow();
    let red = Style::new().red();

    println!("{}", bold.apply_to("Validating organization tree..."));
    println!();

    let mut has_errors = false;
    let mut errors = Vec::new();

    let orgs_to_check = if let Some(id) = org_id {
        // Check single subtree
        let tree = org_core::get_subtree(&pool, id).await?;
        collect_org_ids(&tree)
    } else {
        // Check all organizations
        let all_orgs = sqlx::query_as::<_, crate::models::org::SysOrg>(
            "SELECT id FROM t_sys_org WHERE delete_flag = 0"
        )
        .fetch_all(&pool)
        .await?;
        all_orgs.into_iter().map(|o| o.id).collect::<Vec<_>>()
    };

    // Check 1: Validate left_num < right_num
    println!("{}", bold.apply_to("Check 1: Validating left_num < right_num"));
    for &id in &orgs_to_check {
        if let Ok(Some(org)) = org_core::get_by_id(&pool, id).await {
            let left = org.left_num.unwrap_or(0);
            let right = org.right_num.unwrap_or(0);
            if left >= right {
                errors.push(format!("Org #{}: left_num ({}) >= right_num ({})", id, left, right));
                has_errors = true;
            }
        }
    }
    if !has_errors {
        println!("  {}", green.apply_to("✓ All organizations have valid left_num < right_num"));
    } else {
        println!("  {}", red.apply_to(format!("✗ Found {} errors", errors.len())));
    }
    println!();

    // Check 2: Validate parent-child relationships
    println!("{}", bold.apply_to("Check 2: Validating parent-child relationships"));
    let mut parent_errors = 0;
    for &id in &orgs_to_check {
        if let Ok(Some(org)) = org_core::get_by_id(&pool, id).await {
            if let Some(pid) = org.pid {
                if let Ok(Some(parent)) = org_core::get_by_id(&pool, pid).await {
                    let parent_left = parent.left_num.unwrap_or(0);
                    let parent_right = parent.right_num.unwrap_or(0);
                    let child_left = org.left_num.unwrap_or(0);
                    let child_right = org.right_num.unwrap_or(0);

                    // Child should be within parent bounds
                    if !(child_left > parent_left && child_right < parent_right) {
                        errors.push(format!(
                            "Org #{} (child of #{}) has bounds [{},{}] outside parent [{},{}]",
                            id, pid, child_left, child_right, parent_left, parent_right
                        ));
                        parent_errors += 1;
                    }
                }
            }
        }
    }
    if parent_errors == 0 {
        println!("  {}", green.apply_to("✓ All parent-child relationships are valid"));
    } else {
        println!("  {}", red.apply_to(format!("✗ Found {} invalid parent-child relationships", parent_errors)));
    }
    println!();

    // Check 3: Check for gaps or overlaps
    println!("{}", bold.apply_to("Check 3: Checking for gaps and overlaps"));
    let mut gap_errors = 0;
    let all_orgs_sorted = sqlx::query_as::<_, crate::models::org::SysOrg>(
        "SELECT * FROM t_sys_org WHERE delete_flag = 0 ORDER BY left_num ASC"
    )
    .fetch_all(&pool)
    .await?;

    for i in 1..all_orgs_sorted.len() {
        let prev = &all_orgs_sorted[i - 1];
        let curr = &all_orgs_sorted[i];

        // Check if current left_num overlaps with previous right_num
        let prev_right = prev.right_num.unwrap_or(0);
        let curr_left = curr.left_num.unwrap_or(0);

        if curr_left <= prev_right {
            // Check if they are actually parent-child (which is valid)
            let is_valid_parent_child = curr.pid == Some(prev.id)
                || (curr.left_num.unwrap_or(0) > prev.left_num.unwrap_or(0)
                    && curr.right_num.unwrap_or(0) < prev.right_num.unwrap_or(0));

            if !is_valid_parent_child {
                errors.push(format!(
                    "Potential overlap: Org #{} [{},{}] and Org #{} [{},{}]",
                    prev.id, prev.left_num.unwrap_or(0), prev_right,
                    curr.id, curr_left, curr.right_num.unwrap_or(0)
                ));
                gap_errors += 1;
            }
        }
    }
    if gap_errors == 0 {
        println!("  {}", green.apply_to("✓ No gaps or overlaps detected"));
    } else {
        println!("  {}", yellow.apply_to(format!("⚠ Found {} potential issues", gap_errors)));
    }
    println!();

    // Summary
    println!("{}", bold.apply_to("Summary"));
    println!("  Organizations checked: {}", orgs_to_check.len());
    println!("  Total errors found: {}", errors.len());
    println!();

    if !errors.is_empty() {
        println!("{}", bold.apply_to("Error details:"));
        for error in &errors {
            println!("  {}", red.apply_to(error));
        }
    }

    if has_errors || parent_errors > 0 || gap_errors > 0 {
        println!();
        println!("{}", red.apply_to("✗ Validation failed!"));
        std::process::exit(1);
    } else {
        println!("{}", green.apply_to("✓ All validations passed!"));
    }

    Ok(())
}

fn collect_org_ids(tree: &crate::models::org::SysOrgTreeItem) -> Vec<i64> {
    let mut ids = vec![tree.id];
    for child in &tree.children {
        ids.extend(collect_org_ids(child));
    }
    ids
}
