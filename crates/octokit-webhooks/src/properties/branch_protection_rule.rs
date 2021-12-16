use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BranchProtectionRuleEnforcementLevel {
  Off,
  NonAdmins,
  Everyone,
}

#[derive(Debug, Deserialize)]
pub struct BranchProtectionRule {
  pub id: u32,
  pub repository_id: u32,
  pub name: String,
  pub created_at: String,
  pub updated_at: String,
  pub pull_request_reviews_enforcement_level: BranchProtectionRuleEnforcementLevel,
  pub required_approving_review_count: u32,
  pub dismiss_stale_reviews_on_push: bool,
  pub require_code_owner_review: bool,
  pub authorized_dismissal_actors_only: bool,
  pub ignore_approvals_from_contributors: bool,
  pub required_status_checks: Vec<String>,
  pub required_status_checks_enforcement_level: BranchProtectionRuleEnforcementLevel,
  pub strict_required_status_checks_policy: bool,
  pub signature_requirement_enforcement_level: BranchProtectionRuleEnforcementLevel,
  pub linear_history_requirement_enforcement_level: BranchProtectionRuleEnforcementLevel,
  pub admin_enforced: bool,
  pub allow_force_pushes_enforcement_level: BranchProtectionRuleEnforcementLevel,
  pub allow_deletions_enforcement_level: BranchProtectionRuleEnforcementLevel,
  pub merge_queue_enforcement_level: BranchProtectionRuleEnforcementLevel,
  pub required_deployments_enforcement_level: BranchProtectionRuleEnforcementLevel,
  pub required_conversation_resolution_level: BranchProtectionRuleEnforcementLevel,
  pub authorized_actors_only: bool,
  pub authorized_actor_names: Vec<String>,
}
