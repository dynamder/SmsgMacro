use smsg_macro::smsg;
use soul_msg::{MessageMeta, SmsgEnvelope};
use zenoh_ext::{z_deserialize, z_serialize};

mod file_type_tests {
    use super::*;

    #[smsg(category = file, path = "tests/fixtures/messages.smsg")]
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
}

mod package_type_tests {
    use super::*;

    #[smsg(category = package, path = "tests/fixtures/packages/testpkg")]
    pub mod testpkg {}

    #[test]
    fn test_package_root_message() {
        let person = testpkg::Person::new();
        assert_eq!(person.name, String::default());
        assert_eq!(person.age, 0);
        assert_eq!(person.email, String::default());
    }

    #[test]
    fn test_package_root_message_with_data() {
        let person = testpkg::Person {
            name: "John Doe".to_string(),
            age: 30,
            email: "john@example.com".to_string(),
        };
        assert_eq!(person.name, "John Doe");
        assert_eq!(person.age, 30);
        assert_eq!(person.email, "john@example.com");
    }

    #[test]
    fn test_package_nested_module_inventory() {
        let product = testpkg::inventory::Product::new();
        assert_eq!(product.id, String::default());
        assert_eq!(product.name, String::default());
        assert_eq!(product.price, 0.0);
        assert_eq!(product.stock, 0);
    }

    #[test]
    fn test_package_nested_module_inventory_with_data() {
        let product = testpkg::inventory::Product {
            id: "PROD-001".to_string(),
            name: "Widget".to_string(),
            price: 19.99,
            stock: 100,
        };
        assert_eq!(product.id, "PROD-001");
        assert_eq!(product.name, "Widget");
        assert_eq!(product.price, 19.99);
        assert_eq!(product.stock, 100);
    }

    #[test]
    fn test_package_serialization_root_message() {
        let person = testpkg::Person {
            name: "Alice".to_string(),
            age: 25,
            email: "alice@test.com".to_string(),
        };

        let serialized = z_serialize(&person);
        let deserialized: testpkg::Person = z_deserialize(&serialized).unwrap();
        assert_eq!(deserialized.name, "Alice");
        assert_eq!(deserialized.age, 25);
        assert_eq!(deserialized.email, "alice@test.com");
    }

    #[test]
    fn test_package_serialization_nested_message() {
        let product = testpkg::inventory::Product {
            id: "PROD-001".to_string(),
            name: "Gadget".to_string(),
            price: 49.99,
            stock: 50,
        };

        let serialized = z_serialize(&product);
        let deserialized: testpkg::inventory::Product = z_deserialize(&serialized).unwrap();
        assert_eq!(deserialized.id, "PROD-001");
        assert_eq!(deserialized.name, "Gadget");
        assert_eq!(deserialized.price, 49.99);
        assert_eq!(deserialized.stock, 50);
    }
}

mod legacy_syntax_tests {
    use super::*;

    #[smsg("tests/fixtures/messages.smsg")]
    pub mod legacy_messages {}

    #[test]
    fn test_legacy_string_syntax() {
        let msg = legacy_messages::ChatMessage {
            sender: "Bob".to_string(),
            content: "Test message".to_string(),
            timestamp: 999999,
        };
        assert_eq!(msg.sender, "Bob");
        assert_eq!(msg.content, "Test message");
        assert_eq!(msg.timestamp, 999999);
    }
}

mod smsg_envelope_tests {
    use super::*;

    #[smsg(category = file, path = "tests/fixtures/messages.smsg")]
    pub mod test_messages {}

    #[test]
    fn test_smsg_envelope_new_with_chat_message() {
        let msg = test_messages::ChatMessage {
            sender: "Alice".to_string(),
            content: "Hello World".to_string(),
            timestamp: 1234567890,
        };

        let envelope = SmsgEnvelope::new(msg);

        assert_eq!(
            *envelope.version_hash(),
            test_messages::ChatMessage::version_hash()
        );
        assert_eq!(envelope.payload.sender, "Alice");
        assert_eq!(envelope.payload.content, "Hello World");
        assert_eq!(envelope.payload.timestamp, 1234567890);
    }

    #[test]
    fn test_smsg_envelope_new_with_position() {
        let pos = test_messages::Position {
            x: 1.5,
            y: 2.5,
            z: 3.5,
        };

        let envelope = SmsgEnvelope::new(pos);

        assert_eq!(
            *envelope.version_hash(),
            test_messages::Position::version_hash()
        );
        assert_eq!(envelope.payload.x, 1.5);
        assert_eq!(envelope.payload.y, 2.5);
        assert_eq!(envelope.payload.z, 3.5);
    }

    #[test]
    fn test_smsg_envelope_new_with_robot_state() {
        let robot = test_messages::RobotState {
            name: "Robot1".to_string(),
            position: test_messages::Position {
                x: 10.0,
                y: 20.0,
                z: 30.0,
            },
            status: 1,
        };

        let envelope = SmsgEnvelope::new(robot);

        assert_eq!(
            *envelope.version_hash(),
            test_messages::RobotState::version_hash()
        );
        assert_eq!(envelope.payload.name, "Robot1");
        assert_eq!(envelope.payload.position.x, 10.0);
        assert_eq!(envelope.payload.status, 1);
    }

    #[test]
    fn test_smsg_envelope_into_parts() {
        let msg = test_messages::ChatMessage {
            sender: "Bob".to_string(),
            content: "Test".to_string(),
            timestamp: 999,
        };

        let envelope = SmsgEnvelope::new(msg);
        let (hash, payload) = envelope.into_parts();

        assert_eq!(hash, test_messages::ChatMessage::version_hash());
        assert_eq!(payload.sender, "Bob");
        assert_eq!(payload.content, "Test");
        assert_eq!(payload.timestamp, 999);
    }

    #[test]
    fn test_smsg_envelope_into_payload() {
        let msg = test_messages::Position {
            x: 100.0,
            y: 200.0,
            z: 300.0,
        };

        let envelope = SmsgEnvelope::new(msg);
        let payload = envelope.into_payload();

        assert_eq!(payload.x, 100.0);
        assert_eq!(payload.y, 200.0);
        assert_eq!(payload.z, 300.0);
    }

    #[test]
    fn test_message_meta_version_hash_length() {
        let hash = test_messages::ChatMessage::version_hash();
        assert_eq!(hash.len(), 32);

        let hash = test_messages::Position::version_hash();
        assert_eq!(hash.len(), 32);

        let hash = test_messages::RobotState::version_hash();
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_message_meta_message_name() {
        assert_eq!(test_messages::ChatMessage::message_name(), "ChatMessage");
        assert_eq!(test_messages::Position::message_name(), "Position");
        assert_eq!(test_messages::RobotState::message_name(), "RobotState");
    }

    #[test]
    fn test_different_messages_have_different_hashes() {
        let chat_hash = test_messages::ChatMessage::version_hash();
        let position_hash = test_messages::Position::version_hash();
        let robot_hash = test_messages::RobotState::version_hash();

        assert_ne!(chat_hash, position_hash);
        assert_ne!(chat_hash, robot_hash);
        assert_ne!(position_hash, robot_hash);
    }

    #[test]
    fn test_envelope_clone() {
        let msg = test_messages::ChatMessage {
            sender: "Alice".to_string(),
            content: "Clone test".to_string(),
            timestamp: 555,
        };

        let envelope = SmsgEnvelope::new(msg);
        let cloned = envelope.clone();

        assert_eq!(*cloned.version_hash(), *envelope.version_hash());
        assert_eq!(cloned.payload.sender, envelope.payload.sender);
    }

    #[test]
    fn test_envelope_debug_format() {
        let msg = test_messages::ChatMessage {
            sender: "Debug".to_string(),
            content: "Test".to_string(),
            timestamp: 0,
        };

        let envelope = SmsgEnvelope::new(msg);
        let debug_str = format!("{:?}", envelope);

        assert!(debug_str.contains("SmsgEnvelope"));
        assert!(debug_str.contains("version_hash"));
        assert!(debug_str.contains("payload"));
    }

    #[test]
    fn test_smsg_envelope_with_default_values() {
        let msg = test_messages::ChatMessage::new();
        let envelope = SmsgEnvelope::new(msg);

        assert_eq!(envelope.payload.sender, String::default());
        assert_eq!(envelope.payload.content, String::default());
        assert_eq!(envelope.payload.timestamp, 0);
    }

    #[test]
    fn test_smsg_envelope_partial_eq() {
        let msg1 = test_messages::ChatMessage {
            sender: "Same".to_string(),
            content: "Same".to_string(),
            timestamp: 123,
        };
        let msg2 = test_messages::ChatMessage {
            sender: "Same".to_string(),
            content: "Same".to_string(),
            timestamp: 123,
        };

        let envelope1 = SmsgEnvelope::new(msg1);
        let envelope2 = SmsgEnvelope::new(msg2);

        assert_eq!(envelope1, envelope2);
    }

    #[test]
    fn test_smsg_envelope_partial_eq_different() {
        let msg1 = test_messages::ChatMessage {
            sender: "Alice".to_string(),
            content: "Hello".to_string(),
            timestamp: 1,
        };
        let msg2 = test_messages::ChatMessage {
            sender: "Bob".to_string(),
            content: "World".to_string(),
            timestamp: 2,
        };

        let envelope1 = SmsgEnvelope::new(msg1);
        let envelope2 = SmsgEnvelope::new(msg2);

        assert_ne!(envelope1, envelope2);
    }

    #[test]
    fn test_smsg_envelope_with_serialization_roundtrip() {
        let original = test_messages::ChatMessage {
            sender: "Serialization".to_string(),
            content: "Test content".to_string(),
            timestamp: 123456,
        };

        let envelope = SmsgEnvelope::new(original);
        let serialized = z_serialize(&envelope.payload);
        let deserialized: test_messages::ChatMessage = z_deserialize(&serialized).unwrap();

        assert_eq!(deserialized.sender, "Serialization");
        assert_eq!(deserialized.content, "Test content");
        assert_eq!(deserialized.timestamp, 123456);
    }

    #[test]
    fn test_multiple_envelopes_same_type() {
        let msg1 = test_messages::ChatMessage {
            sender: "First".to_string(),
            content: "Message 1".to_string(),
            timestamp: 1,
        };
        let msg2 = test_messages::ChatMessage {
            sender: "Second".to_string(),
            content: "Message 2".to_string(),
            timestamp: 2,
        };

        let envelope1 = SmsgEnvelope::new(msg1);
        let envelope2 = SmsgEnvelope::new(msg2);

        assert_eq!(*envelope1.version_hash(), *envelope2.version_hash());
        assert_ne!(envelope1.payload.sender, envelope2.payload.sender);
    }

    #[test]
    fn test_smsg_envelope_preserves_hash_across_creations() {
        let msg1 = test_messages::ChatMessage {
            sender: "Test1".to_string(),
            content: "Content1".to_string(),
            timestamp: 111,
        };
        let msg2 = test_messages::ChatMessage {
            sender: "Test2".to_string(),
            content: "Content2".to_string(),
            timestamp: 222,
        };

        let envelope1 = SmsgEnvelope::new(msg1);
        let envelope2 = SmsgEnvelope::new(msg2);

        let expected_hash = test_messages::ChatMessage::version_hash();
        assert_eq!(*envelope1.version_hash(), expected_hash);
        assert_eq!(*envelope2.version_hash(), expected_hash);
    }
}
