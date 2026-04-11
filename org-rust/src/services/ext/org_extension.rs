use async_trait::async_trait;
use crate::models::org::{SysOrg, CreateOrgRequest};
use crate::services::RequestContext;
use crate::models::error::AppResult;

/// 第三方组织扩展接口
/// 每个层级可以有不同的实现
#[async_trait]
pub trait OrgExtension: Send + Sync {
    /// 组织创建后的扩展处理
    /// 返回 ext_org_id 用于填充 t_sys_org_ext 表
    async fn on_org_created(
        &self,
        org: &SysOrg,
        req: &CreateOrgRequest,
        ctx: &RequestContext,
    ) -> AppResult<Option<i64>>;

    /// 组织删除前的扩展处理
    async fn on_org_delete(
        &self,
        org: &SysOrg,
        ctx: &RequestContext,
    ) -> AppResult<()>;

    /// 获取支持的层级标识（返回 node_level 或 level）
    fn supported_levels(&self) -> Vec<OrgLevel>;
}

/// 组织层级定义
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrgLevel {
    /// 物理层级（node_level）
    NodeLevel(i32),
    /// 业务层级标识（level 字段，如 "school", "college", "major", "class"）
    BusinessLevel(String),
}
