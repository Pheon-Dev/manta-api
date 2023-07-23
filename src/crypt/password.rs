use super::{Error, Result};
use crate::config;
use crate::crypt::{encrypt_into_b64u, EncryptContent};
use lazy_regex::regex_captures;

pub const DEFAULT_SCHEME: &str = "02";

/// Encrypt the password with the default scheme.
pub fn encrypt_password(enc_content: &EncryptContent) -> Result<String> {
	encrypt_for_scheme(DEFAULT_SCHEME, enc_content)
}

#[derive(Debug)]
pub enum SchemeStatus {
	Ok,       // The password use the latest scheme. All good.
	Outdated, // The password use a old scheme. Would need to be re-encrypted.
}
/// Validate if an EncryptContent matches.
pub fn validate_password(
	enc_content: &EncryptContent,
	password_ref: &str,
) -> Result<SchemeStatus> {
	let scheme_ref = extract_scheme(password_ref)?;
	let password_new = encrypt_for_scheme(&scheme_ref, enc_content)?;

	if password_new == password_ref {
		if scheme_ref == DEFAULT_SCHEME {
			Ok(SchemeStatus::Ok)
		} else {
			Ok(SchemeStatus::Outdated)
		}
	} else {
		Err(Error::PasswordNotMatching)
	}
}

// region:    --- Schemes
fn encrypt_scheme_01(enc_content: &EncryptContent) -> Result<String> {
	let key = &config().PASSWORD_KEY;

	encrypt_into_b64u(key, enc_content)
}

// Same as "01", just for demonstration.
fn encrypt_scheme_02(enc_content: &EncryptContent) -> Result<String> {
	let key = &config().PASSWORD_KEY;

	encrypt_into_b64u(key, enc_content)
}

// endregion: --- Schemes

// region:    --- Scheme Infra
/// scheme: e.g., "01"
fn encrypt_for_scheme(scheme: &str, args: &EncryptContent) -> Result<String> {
	let password = match scheme {
		"01" => encrypt_scheme_01(args),
		"02" => encrypt_scheme_02(args),
		_ => Err(Error::SchemeUnknown(scheme.to_string())),
	};

	Ok(format!("#{scheme}#{}", password?))
}

fn extract_scheme(enc_content: &str) -> Result<String> {
	regex_captures!(
		r#"^#(\w+)#.*"#, // a literal regex
		enc_content
	)
	.map(|(_whole, scheme)| scheme.to_string())
	.ok_or(Error::SchemeNotFoundInContent)
}
// endregion: --- Scheme Infra

// region:    --- Tests
#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::Result;

	#[test]
	fn test_validate() -> Result<()> {
		// -- Setup & Fixtures
		let fx_salt = "some-salt";
		let fx_password_clear = "welcome";

		let password_enc_1 = encrypt_password(&EncryptContent {
			salt: fx_salt.to_string(),
			content: fx_password_clear.to_string(),
		})?;

		validate_password(
			&EncryptContent {
				salt: fx_salt.to_string(),
				content: fx_password_clear.to_string(),
			},
			&password_enc_1,
		)?;

		Ok(())
	}

	#[test]
	fn test_extract_scheme_ok() -> Result<()> {
		// -- Fixtures
		let fx_password = "#01#DdVzPPKKpjs-xuf-Y88t3MpQ5KPDqa7C2gpaTIysHnHIzX_j2IgNb3WtEDHLfF2ps1OWVPKOkgLFvvDMvNrN-A";

		// -- Exec
		let res = extract_scheme(fx_password)?;

		// -- Check
		assert_eq!(res, "01");

		Ok(())
	}

	#[test]
	fn test_extract_scheme_err_without() -> Result<()> {
		// -- Fixtures
		let fx_password = "DdVzPPKKpjs-xuf-Y88t3MpQ5KPDqa7C2gpaTIysHnHIzX_j2IgNb3WtEDHLfF2ps1OWVPKOkgLFvvDMvNrN-A";

		// -- Exec
		let res = extract_scheme(fx_password);

		// -- Check
		assert!(
			matches!(res, Err(Error::SchemeNotFoundInContent)),
			"Error not matching. Actual: {res:?}"
		);

		Ok(())
	}
}
// endregion: --- Tests
