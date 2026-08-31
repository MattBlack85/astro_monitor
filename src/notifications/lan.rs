use super::Notification;
use log::{error, info};
use std::net::UdpSocket;

/// Delivers notifications as a UDP datagram on the local network.
///
/// At a dark site there is usually no connectivity, so `TelegramNotifier`
/// cannot reach its relay exactly when an unattended session is most likely
/// to go wrong. A datagram needs no relay, no broker and no internet: a
/// listener on the same machine or on the same LAN picks it up and surfaces
/// the alert.
///
/// The payload is the message itself, as plain text, so that the simplest
/// possible listener works:
///
/// ```shell
/// nc -ul 5005
/// ```
pub struct LanNotifier {
    pub target_addr: String,
}

impl LanNotifier {
    pub fn new(target_addr: impl Into<String>) -> Self {
        Self {
            target_addr: target_addr.into(),
        }
    }
}

impl Notification for LanNotifier {
    fn send(&self, message: &str) -> Result<(), String> {
        // Bind an ephemeral port: we only ever send.
        let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| {
            error!("LAN notification: cannot open socket: {}", e);
            e.to_string()
        })?;

        // Required before sending to a broadcast address, and harmless for a
        // unicast one, so it is set unconditionally rather than by sniffing
        // the target address.
        socket.set_broadcast(true).map_err(|e| {
            error!("LAN notification: cannot enable broadcast: {}", e);
            e.to_string()
        })?;

        match socket.send_to(message.as_bytes(), &self.target_addr) {
            Ok(_) => {
                info!("LAN notification sent to {}.", self.target_addr);
                Ok(())
            }
            Err(e) => {
                error!(
                    "LAN notification: send to {} failed: {}",
                    self.target_addr, e
                );
                Err(e.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sends_the_message_verbatim() {
        // Bind a listener on a free port and check the datagram arrives.
        let listener = UdpSocket::bind("127.0.0.1:0").expect("bind listener");
        let addr = listener.local_addr().expect("local addr");

        let notifier = LanNotifier::new(addr.to_string());
        notifier.send("KStars has stopped.").expect("send");

        let mut buf = [0u8; 1024];
        let (len, _) = listener.recv_from(&mut buf).expect("recv");
        assert_eq!(&buf[..len], b"KStars has stopped.");
    }

    #[test]
    fn unresolvable_target_is_an_error_not_a_panic() {
        let notifier = LanNotifier::new("not a valid address");
        assert!(notifier.send("anything").is_err());
    }
}
