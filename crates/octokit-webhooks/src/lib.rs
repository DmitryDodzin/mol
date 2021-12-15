use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BranchProtectionRule;

#[derive(Debug, Deserialize)]
pub struct CheckRun;

#[derive(Debug, Deserialize)]
pub struct CheckSuite;

#[derive(Debug, Deserialize)]
pub struct CodeScanningAlert;

#[derive(Debug, Deserialize)]
pub struct CommitComment;

#[derive(Debug, Deserialize)]
pub struct Create;

#[derive(Debug, Deserialize)]
pub struct Delete;

#[derive(Debug, Deserialize)]
pub struct Deployment;

#[derive(Debug, Deserialize)]
pub struct DeploymentStatus;

#[derive(Debug, Deserialize)]
pub struct DeployKey;

#[derive(Debug, Deserialize)]
pub struct Discussion;

#[derive(Debug, Deserialize)]
pub struct DiscussionComment;

#[derive(Debug, Deserialize)]
pub struct Fork;

#[derive(Debug, Deserialize)]
pub struct Gollum;

#[derive(Debug, Deserialize)]
pub struct Issues;

#[derive(Debug, Deserialize)]
pub struct IssueComment;

#[derive(Debug, Deserialize)]
pub struct Label;

#[derive(Debug, Deserialize)]
pub struct Member;

#[derive(Debug, Deserialize)]
pub struct Membership;

#[derive(Debug, Deserialize)]
pub struct Meta;

#[derive(Debug, Deserialize)]
pub struct Milestone;

#[derive(Debug, Deserialize)]
pub struct Organization;

#[derive(Debug, Deserialize)]
pub struct OrgBlock;

#[derive(Debug, Deserialize)]
pub struct Package;

#[derive(Debug, Deserialize)]
pub struct PageBuild;

#[derive(Debug, Deserialize)]
pub struct Project;

#[derive(Debug, Deserialize)]
pub struct ProjectCard;

#[derive(Debug, Deserialize)]
pub struct ProjectColumn;

#[derive(Debug, Deserialize)]
pub struct Public;

#[derive(Debug, Deserialize)]
pub struct PullRequest;

#[derive(Debug, Deserialize)]
pub struct PullRequestReview;

#[derive(Debug, Deserialize)]
pub struct PullRequestReviewComment;

#[derive(Debug, Deserialize)]
pub struct PullRequestReviewThread;

#[derive(Debug, Deserialize)]
pub struct Push;

#[derive(Debug, Deserialize)]
pub struct RegistryPackage;

#[derive(Debug, Deserialize)]
pub struct Release;

#[derive(Debug, Deserialize)]
pub struct Repository;

#[derive(Debug, Deserialize)]
pub struct RepositoryImport;

#[derive(Debug, Deserialize)]
pub struct RepositoryVulnerabilityAlert;

#[derive(Debug, Deserialize)]
pub struct SecretScanningAlert;

#[derive(Debug, Deserialize)]
pub struct Star;

#[derive(Debug, Deserialize)]
pub struct Status;

#[derive(Debug, Deserialize)]
pub struct Team;

#[derive(Debug, Deserialize)]
pub struct TeamAdd;

#[derive(Debug, Deserialize)]
pub struct Watch;

#[derive(Debug, Deserialize)]
pub struct WorkflowJob;

#[derive(Debug, Deserialize)]
pub struct WorkflowRun;
