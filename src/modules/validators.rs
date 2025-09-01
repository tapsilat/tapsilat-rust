use crate::error::{Result, TapsilatError};
use regex::Regex;

pub struct Validators;

impl Validators {
    /// Validates Turkish GSM numbers
    /// Accepts formats: +90XXXXXXXXXX, 90XXXXXXXXXX, 0XXXXXXXXXX, XXXXXXXXXX
    pub fn validate_gsm_number(gsm: &str) -> Result<String> {
        Self::validate_gsm(gsm)
    }

    /// Validates Turkish GSM numbers
    /// Accepts formats: +90XXXXXXXXXX, 90XXXXXXXXXX, 0XXXXXXXXXX, XXXXXXXXXX
    pub fn validate_gsm(gsm: &str) -> Result<String> {
        let gsm = gsm.trim().replace(" ", "").replace("-", "");

        // Remove country code variations
        let normalized = if gsm.starts_with("+90") {
            gsm.strip_prefix("+90").unwrap()
        } else if gsm.starts_with("90") {
            gsm.strip_prefix("90").unwrap()
        } else if gsm.starts_with("0") {
            gsm.strip_prefix("0").unwrap()
        } else {
            &gsm
        };

        // Check if it's exactly 10 digits and starts with 5
        if normalized.len() != 10 {
            return Err(TapsilatError::ValidationError(
                "GSM number must be 10 digits long".to_string(),
            ));
        }

        if !normalized.starts_with("5") {
            return Err(TapsilatError::ValidationError(
                "Turkish mobile numbers must start with 5".to_string(),
            ));
        }

        // Check if all characters are digits
        if !normalized.chars().all(|c| c.is_ascii_digit()) {
            return Err(TapsilatError::ValidationError(
                "GSM number must contain only digits".to_string(),
            ));
        }

        Ok(format!("90{}", normalized))
    }

    /// Validates installment count
    pub fn validate_installments(installments: u8) -> Result<()> {
        const VALID_INSTALLMENTS: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        if !VALID_INSTALLMENTS.contains(&installments) {
            return Err(TapsilatError::ValidationError(format!(
                "Invalid installment count: {}. Valid values are 1-12",
                installments
            )));
        }

        Ok(())
    }

    /// Validates email address
    pub fn validate_email(email: &str) -> Result<()> {
        let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$")
            .map_err(|e| TapsilatError::ValidationError(format!("Email regex error: {}", e)))?;

        if !email_regex.is_match(email) {
            return Err(TapsilatError::ValidationError(
                "Invalid email format".to_string(),
            ));
        }

        Ok(())
    }

    /// Validates Turkish identity number (TC Kimlik No)
    pub fn validate_identity_number(identity: &str) -> Result<()> {
        let identity = identity.trim();

        if identity.len() != 11 {
            return Err(TapsilatError::ValidationError(
                "Identity number must be 11 digits".to_string(),
            ));
        }

        if !identity.chars().all(|c| c.is_ascii_digit()) {
            return Err(TapsilatError::ValidationError(
                "Identity number must contain only digits".to_string(),
            ));
        }

        let digits: Vec<u8> = identity
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        // First digit cannot be 0
        if digits[0] == 0 {
            return Err(TapsilatError::ValidationError(
                "Identity number cannot start with 0".to_string(),
            ));
        }

        // Validate checksum algorithm
        let sum_odd: u8 = digits[0] + digits[2] + digits[4] + digits[6] + digits[8];
        let sum_even: u8 = digits[1] + digits[3] + digits[5] + digits[7];

        let check_digit_10 = (sum_odd * 7 - sum_even) % 10;
        if check_digit_10 != digits[9] {
            return Err(TapsilatError::ValidationError(
                "Invalid identity number checksum".to_string(),
            ));
        }

        let total_sum: u8 = digits[0..10].iter().sum();
        let check_digit_11 = total_sum % 10;
        if check_digit_11 != digits[10] {
            return Err(TapsilatError::ValidationError(
                "Invalid identity number checksum".to_string(),
            ));
        }

        Ok(())
    }

    /// Validates amount (must be positive and have max 2 decimal places)
    pub fn validate_amount(amount: f64) -> Result<()> {
        if amount <= 0.0 {
            return Err(TapsilatError::ValidationError(
                "Amount must be greater than 0".to_string(),
            ));
        }

        // Check decimal places
        let decimal_places = format!("{:.10}", amount)
            .trim_end_matches('0')
            .split('.')
            .nth(1)
            .map(|s| s.len())
            .unwrap_or(0);

        if decimal_places > 2 {
            return Err(TapsilatError::ValidationError(
                "Amount cannot have more than 2 decimal places".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gsm_validation() {
        assert!(Validators::validate_gsm("+905551234567").is_ok());
        assert!(Validators::validate_gsm("905551234567").is_ok());
        assert!(Validators::validate_gsm("05551234567").is_ok());
        assert!(Validators::validate_gsm("5551234567").is_ok());

        assert!(Validators::validate_gsm("123456789").is_err()); // Too short
        assert!(Validators::validate_gsm("4551234567").is_err()); // Doesn't start with 5
    }

    #[test]
    fn test_installment_validation() {
        assert!(Validators::validate_installments(1).is_ok());
        assert!(Validators::validate_installments(12).is_ok());
        assert!(Validators::validate_installments(13).is_err());
        assert!(Validators::validate_installments(0).is_err());
    }

    #[test]
    fn test_email_validation() {
        assert!(Validators::validate_email("test@example.com").is_ok());
        assert!(Validators::validate_email("invalid-email").is_err());
        assert!(Validators::validate_email("@invalid.com").is_err());
    }

    #[test]
    fn test_amount_validation() {
        assert!(Validators::validate_amount(10.50).is_ok());
        assert!(Validators::validate_amount(0.01).is_ok());
        assert!(Validators::validate_amount(-5.0).is_err());
        assert!(Validators::validate_amount(0.0).is_err());
        assert!(Validators::validate_amount(10.555).is_err()); // Too many decimals
    }
}
