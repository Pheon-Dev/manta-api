use super::{Error, Result};
use crate::config;
use crate::crypt::{encrypt_into_b64u, EncryptContent};
use lazy_regex::regex_captures;

pub const DEFAULT_SCHEME: &str = "02";

/// Encrypt the password with the default scheme.
pub fn encrypt_pwd(enc_content: &EncryptContent) -> Result<String> {
	encrypt_for_scheme(DEFAULT_SCHEME, enc_content)
}

#[derive(Debug)]
pub enum SchemeStatus {
	Ok,       // The pwd use the latest scheme. All good.
	Outdated, // The pwd use a old scheme. Would need to be re-encrypted.
}
/// Validate if an EncryptContent matches.
pub fn validate_pwd(
	enc_content: &EncryptContent,
	pwd_ref: &str,
) -> Result<SchemeStatus> {
	let scheme_ref = extract_scheme(pwd_ref)?;
	let pwd_new = encrypt_for_scheme(&scheme_ref, enc_content)?;

	if pwd_new == pwd_ref {
		if scheme_ref == DEFAULT_SCHEME {
			Ok(SchemeStatus::Ok)
		} else {
			Ok(SchemeStatus::Outdated)
		}
	} else {
		Err(Error::PwdNotMatching)
	}
}

// region:    --- Schemes
fn encrypt_scheme_01(enc_content: &EncryptContent) -> Result<String> {
	let key = &config().PWD_KEY;

	encrypt_into_b64u(key, enc_content)
}

// Same as "01", just for demonstration.
fn encrypt_scheme_02(enc_content: &EncryptContent) -> Result<String> {
	let key = &config().PWD_KEY;

	encrypt_into_b64u(key, enc_content)
}

// endregion: --- Schemes

// region:    --- Scheme Infra
/// scheme: e.g., "01"
fn encrypt_for_scheme(scheme: &str, args: &EncryptContent) -> Result<String> {
	let pwd = match scheme {
		"01" => encrypt_scheme_01(args),
		"02" => encrypt_scheme_02(args),
		_ => Err(Error::SchemeUnknown(scheme.to_string())),
	};

	Ok(format!("#{scheme}#{}", pwd?))
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
		let fx_pwd_clear = "welcome";

		let pwd_enc_1 = encrypt_pwd(&EncryptContent {
			salt: fx_salt.to_string(),
			content: fx_pwd_clear.to_string(),
		})?;

		validate_pwd(
			&EncryptContent {
				salt: fx_salt.to_string(),
				content: fx_pwd_clear.to_string(),
			},
			&pwd_enc_1,
		)?;

		Ok(())
	}

	#[test]
	fn test_extract_scheme_ok() -> Result<()> {
		// -- Fixtures
		let fx_pwd = "#01#DdVzPPKKpjs-xuf-Y88t3MpQ5KPDqa7C2gpaTIysHnHIzX_j2IgNb3WtEDHLfF2ps1OWVPKOkgLFvvDMvNrN-A";

		// -- Exec
		let res = extract_scheme(fx_pwd)?;

		// -- Check
		assert_eq!(res, "01");

		Ok(())
	}

	#[test]
	fn test_extract_scheme_err_without() -> Result<()> {
		// -- Fixtures
		let fx_pwd = "DdVzPPKKpjs-xuf-Y88t3MpQ5KPDqa7C2gpaTIysHnHIzX_j2IgNb3WtEDHLfF2ps1OWVPKOkgLFvvDMvNrN-A";

		// -- Exec
		let res = extract_scheme(fx_pwd);

		// -- Check
		assert!(
			matches!(res, Err(Error::SchemeNotFoundInContent)),
			"Error not matching. Actual: {res:?}"
		);

		Ok(())
	}
}
// endregion: --- Tests
