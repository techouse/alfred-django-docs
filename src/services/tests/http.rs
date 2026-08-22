use super::*;
use std::time::Duration;

#[test]
fn platform_agent_uses_platform_roots_and_bounded_timeouts() {
    let connect_timeout = Duration::from_secs(2);
    let global_timeout = Duration::from_secs(5);
    let agent = platform_agent(connect_timeout, global_timeout);

    assert!(matches!(
        agent.config().tls_config().root_certs(),
        ureq::tls::RootCerts::PlatformVerifier
    ));
    assert_eq!(agent.config().timeouts().connect, Some(connect_timeout));
    assert_eq!(agent.config().timeouts().global, Some(global_timeout));
}
