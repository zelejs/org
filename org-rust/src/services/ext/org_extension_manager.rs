use std::sync::Arc;
use std::collections::HashMap;
use crate::services::ext::org_extension::{OrgExtension, OrgLevel};
use crate::models::org::{SysOrg, CreateOrgRequest};
use crate::services::RequestContext;
use crate::models::error::AppResult;

/// 组织扩展管理器
/// 负责注册和管理扩展处理器，并根据组织层级自动分发调用
pub struct OrgExtensionManager {
    // 按 level 映射到扩展实现
    extensions: HashMap<String, Arc<dyn OrgExtension>>,
}

impl OrgExtensionManager {
    /// 创建新的扩展管理器
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
        }
    }

    /// 注册扩展处理器
    pub fn register(&mut self, extension: Arc<dyn OrgExtension>) {
        for level in extension.supported_levels() {
            let key = match level {
                OrgLevel::NodeLevel(n) => format!("node:{}", n),
                OrgLevel::BusinessLevel(s) => format!("biz:{}", s),
            };
            self.extensions.insert(key, extension.clone());
        }
    }

    /// 获取匹配的扩展处理器
    fn get_extension(&self, org: &SysOrg) -> Option<&Arc<dyn OrgExtension>> {
        // 优先按业务层级匹配
        if let Some(ref level) = org.level {
            let key = format!("biz:{}", level);
            if let Some(ext) = self.extensions.get(&key) {
                return Some(ext);
            }
        }

        // 按物理层级匹配
        if let Some(node_level) = org.node_level {
            let key = format!("node:{}", node_level);
            if let Some(ext) = self.extensions.get(&key) {
                return Some(ext);
            }
        }

        None
    }

    /// 触发组织创建事件
    pub async fn on_org_created(
        &self,
        org: &SysOrg,
        req: &CreateOrgRequest,
        ctx: &RequestContext,
    ) -> AppResult<Option<i64>> {
        if let Some(extension) = self.get_extension(org) {
            extension.on_org_created(org, req, ctx).await
        } else {
            Ok(None)
        }
    }

    /// 触发组织删除事件
    pub async fn on_org_delete(
        &self,
        org: &SysOrg,
        ctx: &RequestContext,
    ) -> AppResult<()> {
        if let Some(extension) = self.get_extension(org) {
            extension.on_org_delete(org, ctx).await
        } else {
            Ok(())
        }
    }
}

impl Default for OrgExtensionManager {
    fn default() -> Self {
        Self::new()
    }
}
