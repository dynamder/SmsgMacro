use soul_msg::smsg;
use zenoh_ext::{z_deserialize, z_serialize};

#[smsg("tests/fixtures/messages.smsg")]
pub mod test_messages {}

#[test]
fn test_generated_chat_message() {
    let msg = test_messages::ChatMessage::new();
    assert_eq!(msg.sender, String::default());
    assert_eq!(msg.content, String::default());
    assert_eq!(msg.timestamp, 0);
}

#[test]
fn test_generated_position() {
    let pos = test_messages::Position::new();
    assert_eq!(pos.x, 0.0);
    assert_eq!(pos.y, 0.0);
    assert_eq!(pos.z, 0.0);
}

#[test]
fn test_generated_robot_state() {
    let robot = test_messages::RobotState::new();
    assert_eq!(robot.name, String::default());
    assert_eq!(robot.position, test_messages::Position::default());
    assert_eq!(robot.status, 0);
}

#[test]
fn test_chat_message_serialization() {
    let msg = test_messages::ChatMessage {
        sender: "Alice".to_string(),
        content: "Hello".to_string(),
        timestamp: 12345,
    };

    let serialized = z_serialize(&msg);
    let deserialized: test_messages::ChatMessage = z_deserialize(&serialized).unwrap();
    assert_eq!(deserialized.sender, "Alice");
    assert_eq!(deserialized.content, "Hello");
    assert_eq!(deserialized.timestamp, 12345);
}
