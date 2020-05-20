use snmp_usm::{SecurityError, SecurityParams};

#[test]
fn it_encodes_empty_security_params() {
    let result = SecurityParams::discovery().encode();
    let expected = [
        0x30, 0x0E, 0x04, 0x00, 0x02, 0x01, 0x00, 0x02, 0x01, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04,
        0x00,
    ];

    assert_eq!(result, expected);
}

#[test]
fn it_creates_security_params_with_username_and_auth_params() {
    let incoming_security_params = vec![
        0x30, 0x21, 0x04, 0x11, 0x80, 0x00, 0x1F, 0x88, 0x80, 0xFA, 0xA8, 0x11, 0x60, 0x0F, 0xA2,
        0xC5, 0x5E, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01, 0x18, 0x02, 0x03, 0x01, 0x85, 0xFC, 0x04,
        0x00, 0x04, 0x00, 0x04, 0x00,
    ];
    let result = SecurityParams::decode(&incoming_security_params)
        .unwrap()
        .set_username(b"username")
        .set_auth_params_placeholder()
        .encode();
    let expected = vec![
        0x30, 0x35, 0x04, 0x11, 0x80, 0x00, 0x1F, 0x88, 0x80, 0xFA, 0xA8, 0x11, 0x60, 0x0F, 0xA2,
        0xC5, 0x5E, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01, 0x18, 0x02, 0x03, 0x01, 0x85, 0xFC, 0x04,
        0x08, 0x75, 0x73, 0x65, 0x72, 0x6E, 0x61, 0x6D, 0x65, 0x04, 0x0C, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00,
    ];

    assert_eq!(result, expected);
}

#[test]
fn it_returns_malformed_security_params_error_for_empty_security_params() {
    let incoming_security_params = [];
    let result = SecurityParams::decode(&incoming_security_params);

    assert_eq!(result, Err(SecurityError::MalformedSecurityParams));
}

#[test]
fn it_returns_malformed_security_params_error_for_negative_engine_boots() {
    let security_params = [
        0x30, 0x0E, 0x04, 0x00, 0x02, 0x01, 0xFF, 0x02, 0x01, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04,
        0x00,
    ];
    let result = SecurityParams::decode(&security_params);

    assert_eq!(result, Err(SecurityError::MalformedSecurityParams));
}

#[test]
fn it_returns_malformed_security_params_error_for_negative_engine_time() {
    let security_params = [
        0x30, 0x0E, 0x04, 0x00, 0x02, 0x01, 0x00, 0x02, 0x01, 0xFF, 0x04, 0x00, 0x04, 0x00, 0x04,
        0x00,
    ];
    let result = SecurityParams::decode(&security_params);

    assert_eq!(result, Err(SecurityError::MalformedSecurityParams));
}
