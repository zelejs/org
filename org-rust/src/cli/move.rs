use sqlx::PgPool;
use console::Style;

pub async fn handle_move(
    pool: PgPool,
    org_id: i64,
    new_parent_id: i64,
) -> anyhow::Result<()> {
    let bold = Style::new().bold();
    let green = Style::new().green();
    let red = Style::new().red();

    println!("{}", bold.apply_to("Moving organization subtree..."));
    println!("  Org ID: {}", org_id);
    println!("  New Parent ID: {}", new_parent_id);
    println!();

    // Validate inputs
    if org_id == new_parent_id {
        println!("{}", red.apply_to("✗ Error: Cannot move organization to itself!"));
        return Err(anyhow::anyhow!("Cannot move organization to itself"));
    }

    // Get the org to move
    let org_to_move = crate::services::org_core::get_by_id(&pool, org_id).await?
        .ok_or_else(|| anyhow::anyhow!("Organization {} not found", org_id))?;

    // Get the new parent
    let new_parent = crate::services::org_core::get_by_id(&pool, new_parent_id).await?
        .ok_or_else(|| anyhow::anyhow!("New parent organization {} not found", new_parent_id))?;

    // Check if new parent is a descendant of org_to_move (would create a cycle)
    let org_left = org_to_move.left_num.ok_or_else(|| anyhow::anyhow!("Org left_num is null"))?;
    let org_right = org_to_move.right_num.ok_or_else(|| anyhow::anyhow!("Org right_num is null"))?;
    let parent_left = new_parent.left_num.ok_or_else(|| anyhow::anyhow!("Parent left_num is null"))?;
    let parent_right = new_parent.right_num.ok_or_else(|| anyhow::anyhow!("Parent right_num is null"))?;

    // If parent is within org's bounds, it's a descendant - cycle!
    if parent_left > org_left && parent_right < org_right {
        println!("{}", red.apply_to("✗ Error: Cannot move organization to its own descendant!"));
        return Err(anyhow::anyhow!("Cannot move organization to its own descendant"));
    }

    // Show current info
    println!("  Current state:");
    println!("    Org: {} (level: {}, bounds: [{},{}])",
        org_to_move.name,
        org_to_move.node_level.unwrap_or(0),
        org_left,
        org_right
    );
    println!("    New Parent: {} (level: {}, bounds: [{},{}])",
        new_parent.name,
        new_parent.node_level.unwrap_or(0),
        parent_left,
        parent_right
    );
    println!();

    // Perform the move operation
    match perform_move(&pool, org_id, new_parent_id).await {
        Ok(_) => {
            println!("{}", green.apply_to("✓ Organization moved successfully!"));

            // Show updated info
            let updated_org = crate::services::org_core::get_by_id(&pool, org_id).await?;
            if let Some(updated) = updated_org {
                println!();
                println!("  Updated state:");
                println!("    Org: {} (level: {}, bounds: [{},{}])",
                    updated.name,
                    updated.node_level.unwrap_or(0),
                    updated.left_num.unwrap_or(0),
                    updated.right_num.unwrap_or(0)
                );
            }
        }
        Err(e) => {
            println!("{}", red.apply_to(format!("✗ Error: {}", e)));
            return Err(e);
        }
    }

    Ok(())
}

/// Move a subtree to a new parent using Nested Set Model
/// This is a complex operation that requires:
/// 1. Calculate the size of the subtree to move
/// 2. Remove the subtree from its current position
/// 3. Shift nodes to close the gap
/// 4. Make space at the new position
/// 5. Insert the subtree at the new position
async fn perform_move(
    pool: &PgPool,
    org_id: i64,
    new_parent_id: i64,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    // Get the org to move and new parent
    let org_to_move = sqlx::query_as::<_, crate::models::org::SysOrg>(
        "SELECT * FROM t_sys_org WHERE id = $1 AND delete_flag = 0"
    )
    .bind(org_id)
    .fetch_one(&mut *tx)
    .await?;

    let new_parent = sqlx::query_as::<_, crate::models::org::SysOrg>(
        "SELECT * FROM t_sys_org WHERE id = $1 AND delete_flag = 0"
    )
    .bind(new_parent_id)
    .fetch_one(&mut *tx)
    .await?;

    let org_left = org_to_move.left_num.ok_or_else(|| anyhow::anyhow!("Org left_num is null"))?;
    let org_right = org_to_move.right_num.ok_or_else(|| anyhow::anyhow!("Org right_num is null"))?;
    let subtree_size = org_right - org_left + 1;

    let parent_right = new_parent.right_num.ok_or_else(|| anyhow::anyhow!("Parent right_num is null"))?;
    let parent_level = new_parent.node_level.ok_or_else(|| anyhow::anyhow!("Parent node_level is null"))?;
    let org_level = org_to_move.node_level.ok_or_else(|| anyhow::anyhow!("Org node_level is null"))?;
    let level_delta = parent_level + 1 - org_level;

    // Determine if we're moving left or right
    let moving_right = org_left > parent_right;

    if moving_right {
        // Moving subtree to the LEFT (earlier in the tree)
        // 1. Shift the subtree left by (org_left - parent_right)
        let shift_amount = org_left - parent_right;

        sqlx::query("UPDATE t_sys_org SET left_num = left_num - $1 WHERE left_num >= $2 AND left_num <= $3 AND delete_flag = 0")
            .bind(shift_amount)
            .bind(org_left)
            .bind(org_right)
            .execute(&mut *tx)
            .await?;

        sqlx::query("UPDATE t_sys_org SET right_num = right_num - $1 WHERE right_num >= $2 AND right_num <= $3 AND delete_flag = 0")
            .bind(shift_amount)
            .bind(org_left)
            .bind(org_right)
            .execute(&mut *tx)
            .await?;

        // 2. Shift nodes between old and new position right by subtree_size
        sqlx::query("UPDATE t_sys_org SET left_num = left_num + $1 WHERE left_num >= $2 AND left_num < $3 AND delete_flag = 0")
            .bind(subtree_size)
            .bind(parent_right)
            .bind(org_left - shift_amount)
            .execute(&mut *tx)
            .await?;

        sqlx::query("UPDATE t_sys_org SET right_num = right_num + $1 WHERE right_num >= $2 AND right_num < $3 AND delete_flag = 0")
            .bind(subtree_size)
            .bind(parent_right)
            .bind(org_left - shift_amount)
            .execute(&mut *tx)
            .await?;
    } else {
        // Moving subtree to the RIGHT (later in the tree)
        // 1. Make space at new position by shifting nodes right
        sqlx::query("UPDATE t_sys_org SET left_num = left_num + $1 WHERE left_num > $2 AND delete_flag = 0")
            .bind(subtree_size)
            .bind(parent_right)
            .execute(&mut *tx)
            .await?;

        sqlx::query("UPDATE t_sys_org SET right_num = right_num + $1 WHERE right_num > $2 AND delete_flag = 0")
            .bind(subtree_size)
            .bind(parent_right)
            .execute(&mut *tx)
            .await?;

        // 2. Move the subtree to the new position
        // The new position is at parent_right, but we need to account for the shift we just made
        // The subtree needs to move from (org_left, org_right) to (parent_right + 1, parent_right + subtree_size)
        // But since we just shifted everything after parent_right by subtree_size, the target is now (parent_right + subtree_size + 1, ...)

        // Actually, let me recalculate:
        // After the shift, the target position starts at parent_right + subtree_size
        let new_left = if org_left > parent_right {
            parent_right + 1
        } else {
            parent_right + subtree_size - (org_right - org_left)
        };

        let shift_amount = new_left - org_left;

        sqlx::query("UPDATE t_sys_org SET left_num = left_num + $1 WHERE left_num >= $2 AND left_num <= $3 AND delete_flag = 0")
            .bind(shift_amount)
            .bind(org_left)
            .bind(org_right)
            .execute(&mut *tx)
            .await?;

        sqlx::query("UPDATE t_sys_org SET right_num = right_num + $1 WHERE right_num >= $2 AND right_num <= $3 AND delete_flag = 0")
            .bind(shift_amount)
            .bind(org_left)
            .bind(org_right)
            .execute(&mut *tx)
            .await?;

        // 3. Close the gap at the old position
        sqlx::query("UPDATE t_sys_org SET left_num = left_num - $1 WHERE left_num > $2 AND delete_flag = 0")
            .bind(subtree_size)
            .bind(org_right)
            .execute(&mut *tx)
            .await?;

        sqlx::query("UPDATE t_sys_org SET right_num = right_num - $1 WHERE right_num > $2 AND delete_flag = 0")
            .bind(subtree_size)
            .bind(org_right)
            .execute(&mut *tx)
            .await?;
    }

    // Update parent_id and node_level for the moved subtree
    sqlx::query("UPDATE t_sys_org SET pid = $1, node_level = node_level + $2 WHERE left_num >= $3 AND right_num <= $4 AND delete_flag = 0")
        .bind(new_parent_id)
        .bind(level_delta)
        .bind(org_to_move.left_num.unwrap_or(0))
        .bind(org_to_move.right_num.unwrap_or(0))
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}
