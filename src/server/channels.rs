use renet::{ChannelConfig, SendType};

pub enum GameChannel {
    PlayerSync,
    PlayerConnection,
}

pub fn generate_channel_config() -> Vec<ChannelConfig> {
    vec![
        ChannelConfig {
            channel_id: 0,
            max_memory_usage_bytes: 5 * 1024 * 1024,
            send_type: SendType::Unreliable
        },
        ChannelConfig {
            channel_id: 1,
            max_memory_usage_bytes: 5 * 1024 * 1024,
            send_type: SendType::ReliableOrdered { resend_time: 300ms }
        },
    ]
}
