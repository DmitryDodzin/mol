pub mod error;
pub mod macros;
pub mod properties;
pub mod util;

mod events;

pub use events::*;

impl_events_unwrapper! {
  #[derive(Debug)]
  pub enum Events {
    BranchProtectionRule(BranchProtectionRuleEvent),
    CheckRun(CheckRunEvent),
    CheckSuite(CheckSuiteEvent),
    Create(CreateEvent),
    Delete(DeleteEvent),
    IssueComment(IssueCommentEvent),
    Label(LabelEvent),
    Meta(MetaEvent),
    Organization(OrganizationEvent),
    Ping(PingEvent),
    PullRequest(PullRequestEvent),
    PullRequestReview(PullRequestReviewEvent),
    Push(PushEvent),
    Release(ReleaseEvent),
    Repository(RepositoryEvent),
    Star(StarEvent),
    Status(StatusEvent),
    WorkflowJob(WorkflowJobEvent),
    WorkflowRun(WorkflowRunEvent),
  }
}
