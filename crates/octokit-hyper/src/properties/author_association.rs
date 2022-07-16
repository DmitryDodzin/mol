use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthorAssociation {
  Collaborator,
  Contributor,
  FirstTimer,
  FirstTimeContributor,
  Mannequin,
  Member,
  None,
  Owner,
}
