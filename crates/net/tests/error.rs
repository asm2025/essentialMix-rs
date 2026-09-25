use emixnet::{Error, NetError};

#[test]
fn test_core_error_to_net_error_keeps_http_and_network() {
    assert!(matches!(NetError::from(Error::Http("500".into())), NetError::Http(m) if m == "500"));
    assert!(matches!(NetError::from(Error::Network("down".into())), NetError::Network(m) if m == "down"));
}

#[test]
fn test_core_error_to_net_error_falls_back_to_operation_failed() {
    let err = NetError::from(Error::Parse("bad json".into()));
    match err {
        NetError::OperationFailed(msg) => assert!(msg.contains("bad json"), "{msg}"),
        other => panic!("unexpected variant: {other:?}"),
    }
}

#[test]
fn test_net_error_round_trips_through_core_error() {
    let core: Error = NetError::http("timeout").into();
    assert!(matches!(NetError::from(core), NetError::Http(m) if m == "timeout"));
}
