mod branch_protection_rule;
mod create;
mod delete;
mod events;
mod issue_comment;
mod label;
mod meta;
mod organization;
mod ping;
mod pull_request;
mod pull_request_review;
mod push;
mod release;
mod repository;
mod star;
mod status;

pub use branch_protection_rule::*;
pub use create::*;
pub use delete::*;
pub use events::*;
pub use issue_comment::*;
pub use label::*;
pub use meta::*;
pub use organization::*;
pub use ping::*;
pub use pull_request::*;
pub use pull_request_review::*;
pub use push::*;
pub use release::*;
pub use repository::*;
pub use star::*;
pub use status::*;

crate::impl_events_unwrapper! {
  #[derive(Debug)]
  pub enum Events {
    BranchProtectionRule(BranchProtectionRuleEvent),
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
  }
}
