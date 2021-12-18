use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationReason {
  ExpiredKey,
  NotSigningKey,
  GpgverifyError,
  GpgverifyUnavailable,
  Unsigned,
  UnknownSignatureType,
  NoUser,
  UnverifiedEmail,
  BadEmail,
  UnknownKey,
  MalformedSignature,
  Invalid,
  Valid,
}

#[derive(Debug, Deserialize)]
pub struct Verification {
  pub verified: bool,
  pub reason: VerificationReason,
  pub signature: Option<String>,
  pub payload: Option<String>,
}
