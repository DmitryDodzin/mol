use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BranchProtectionRuleEvent;

#[derive(Debug, Deserialize)]
pub struct CheckRunEvent;

#[derive(Debug, Deserialize)]
pub struct CheckSuiteEvent;

#[derive(Debug, Deserialize)]
pub struct CodeScanningAlertEvent;

#[derive(Debug, Deserialize)]
pub struct CommitCommentEvent;

#[derive(Debug, Deserialize)]
pub struct CreateEvent;

#[derive(Debug, Deserialize)]
pub struct DeleteEvent;

#[derive(Debug, Deserialize)]
pub struct DeploymentEvent;

#[derive(Debug, Deserialize)]
pub struct DeploymentStatusEvent;

#[derive(Debug, Deserialize)]
pub struct DeployKeyEvent;

#[derive(Debug, Deserialize)]
pub struct DiscussionEvent;

#[derive(Debug, Deserialize)]
pub struct DiscussionCommentEvent;

#[derive(Debug, Deserialize)]
pub struct ForkEvent;

#[derive(Debug, Deserialize)]
pub struct GollumEvent;

#[derive(Debug, Deserialize)]
pub struct IssuesEvent;

#[derive(Debug, Deserialize)]
pub struct IssueCommentEvent;

#[derive(Debug, Deserialize)]
pub struct LabelEvent;

#[derive(Debug, Deserialize)]
pub struct MemberEvent;

#[derive(Debug, Deserialize)]
pub struct MembershipEvent;

#[derive(Debug, Deserialize)]
pub struct MetaEvent;

#[derive(Debug, Deserialize)]
pub struct MilestoneEvent;

#[derive(Debug, Deserialize)]
pub struct OrganizationEvent;

#[derive(Debug, Deserialize)]
pub struct OrgBlockEvent;

#[derive(Debug, Deserialize)]
pub struct PackageEvent;

#[derive(Debug, Deserialize)]
pub struct PageBuildEvent;

#[derive(Debug, Deserialize)]
pub struct ProjectEvent;

#[derive(Debug, Deserialize)]
pub struct ProjectCardEvent;

#[derive(Debug, Deserialize)]
pub struct ProjectColumnEvent;

#[derive(Debug, Deserialize)]
pub struct PublicEvent;

#[derive(Debug, Deserialize)]
pub struct PullRequestReviewEvent;

#[derive(Debug, Deserialize)]
pub struct PullRequestReviewCommentEvent;

#[derive(Debug, Deserialize)]
pub struct PullRequestReviewThreadEvent;

#[derive(Debug, Deserialize)]
pub struct PushEvent;

#[derive(Debug, Deserialize)]
pub struct RegistryPackageEvent;

#[derive(Debug, Deserialize)]
pub struct ReleaseEvent;

#[derive(Debug, Deserialize)]
pub struct RepositoryEvent;

#[derive(Debug, Deserialize)]
pub struct RepositoryImportEvent;

#[derive(Debug, Deserialize)]
pub struct RepositoryVulnerabilityAlertEvent;

#[derive(Debug, Deserialize)]
pub struct SecretScanningAlertEvent;

#[derive(Debug, Deserialize)]
pub struct StarEvent;

#[derive(Debug, Deserialize)]
pub struct StatusEvent;

#[derive(Debug, Deserialize)]
pub struct TeamEvent;

#[derive(Debug, Deserialize)]
pub struct TeamAddEvent;

#[derive(Debug, Deserialize)]
pub struct WatchEvent;

#[derive(Debug, Deserialize)]
pub struct WorkflowJobEvent;

#[derive(Debug, Deserialize)]
pub struct WorkflowRunEvent;
