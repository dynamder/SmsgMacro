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
        let (version_hash, name_hash, payload) = envelope.into_parts();

        assert_eq!(version_hash, test_messages::ChatMessage::version_hash());
        assert_eq!(name_hash, test_messages::ChatMessage::name_hash());
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

    #[test]
    fn test_version_hash_preserved_after_serialization() {
        let original_sender = "Test".to_string();
        let original_content = "Serialization test".to_string();
        let original_timestamp = 12345i64;

        let original = test_messages::ChatMessage {
            sender: original_sender.clone(),
            content: original_content.clone(),
            timestamp: original_timestamp,
        };

        let envelope = SmsgEnvelope::new(original);
        let original_hash = *envelope.version_hash();

        let serialized = z_serialize(&envelope.payload);
        let deserialized: test_messages::ChatMessage = z_deserialize(&serialized).unwrap();

        let deserialized_sender = deserialized.sender.clone();
        let deserialized_content = deserialized.content.clone();

        let new_envelope = SmsgEnvelope::new(deserialized);
        let deserialized_hash = *new_envelope.version_hash();

        assert_eq!(original_hash, deserialized_hash);
        assert_eq!(original_sender, deserialized_sender);
        assert_eq!(original_content, deserialized_content);
    }

    #[test]
    fn test_version_mismatch_detection_between_different_messages() {
        let chat_msg = test_messages::ChatMessage {
            sender: "Alice".to_string(),
            content: "Hello".to_string(),
            timestamp: 100,
        };
        let chat_envelope = SmsgEnvelope::new(chat_msg);

        let pos_msg = test_messages::Position {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let pos_envelope = SmsgEnvelope::new(pos_msg);

        let chat_hash = *chat_envelope.version_hash();
        let pos_hash = *pos_envelope.version_hash();

        assert_ne!(chat_hash, pos_hash);
    }

    #[test]
    fn test_version_hash_matches_trait_method() {
        let msg = test_messages::RobotState {
            name: "TestBot".to_string(),
            position: test_messages::Position {
                x: 10.0,
                y: 20.0,
                z: 30.0,
            },
            status: 5,
        };

        let envelope = SmsgEnvelope::new(msg);

        let expected_hash = test_messages::RobotState::version_hash();
        let envelope_hash = *envelope.version_hash();

        assert_eq!(envelope_hash, expected_hash);
    }

    #[test]
    fn test_message_name_matches_trait_method() {
        assert_eq!(test_messages::ChatMessage::message_name(), "ChatMessage");
    }

    #[test]
    fn test_version_comparison_after_roundtrip() {
        let original_msg = test_messages::Position {
            x: 42.0,
            y: 43.0,
            z: 44.0,
        };

        let original_envelope = SmsgEnvelope::new(original_msg);
        let original_version = *original_envelope.version_hash();

        let serialized = z_serialize(&original_envelope.payload);
        let deserialized: test_messages::Position = z_deserialize(&serialized).unwrap();

        let reconstructed_envelope = SmsgEnvelope::new(deserialized);
        let reconstructed_version = *reconstructed_envelope.version_hash();

        assert_eq!(original_version, reconstructed_version);
        assert_eq!(original_envelope.payload.x, reconstructed_envelope.payload.x);
        assert_eq!(original_envelope.payload.y, reconstructed_envelope.payload.y);
        assert_eq!(original_envelope.payload.z, reconstructed_envelope.payload.z);
    }

    #[test]
    fn test_multiple_messages_same_type_same_hash() {
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
        let msg3 = test_messages::ChatMessage {
            sender: "Third".to_string(),
            content: "Message 3".to_string(),
            timestamp: 3,
        };

        let envelope1 = SmsgEnvelope::new(msg1);
        let envelope2 = SmsgEnvelope::new(msg2);
        let envelope3 = SmsgEnvelope::new(msg3);

        let hash1 = *envelope1.version_hash();
        let hash2 = *envelope2.version_hash();
        let hash3 = *envelope3.version_hash();

        assert_eq!(hash1, hash2);
        assert_eq!(hash2, hash3);
    }

    #[test]
    fn test_serialization_preserves_all_fields() {
        let original = test_messages::RobotState {
            name: "R2-D2".to_string(),
            position: test_messages::Position {
                x: 100.5,
                y: 200.5,
                z: 300.5,
            },
            status: 42,
        };

        let envelope = SmsgEnvelope::new(original);
        
        let serialized = z_serialize(&envelope.payload);
        let deserialized: test_messages::RobotState = z_deserialize(&serialized).unwrap();

        assert_eq!(deserialized.name, "R2-D2");
        assert_eq!(deserialized.position.x, 100.5);
        assert_eq!(deserialized.position.y, 200.5);
        assert_eq!(deserialized.position.z, 300.5);
        assert_eq!(deserialized.status, 42);
    }

    #[test]
    fn test_verify_version_at_runtime() {
        fn verify_version<M: MessageMeta + zenoh_ext::Serialize + zenoh_ext::Deserialize>(
            msg: M,
            expected_hash: [u8; 32],
        ) -> bool {
            let envelope = SmsgEnvelope::new(msg);
            let version = *envelope.version_hash();
            
            let serialized = z_serialize(&envelope.payload);
            let deserialized: M = z_deserialize(&serialized).unwrap();
            
            let reconstructed_envelope = SmsgEnvelope::new(deserialized);
            let reconstructed_version = *reconstructed_envelope.version_hash();
            
            version == expected_hash && reconstructed_version == expected_hash
        }

        let msg = test_messages::ChatMessage {
            sender: "Runtime".to_string(),
            content: "Verification".to_string(),
            timestamp: 99999,
        };

        let expected_hash = test_messages::ChatMessage::version_hash();
        assert!(verify_version(msg, expected_hash));
    }
}

mod version_mismatch_tests {
    use super::*;

    #[smsg(category = file, path = "tests/fixtures/messages_old.smsg")]
    pub mod old_messages {}

    #[smsg(category = file, path = "tests/fixtures/messages.smsg")]
    pub mod new_messages {}

    #[test]
    fn test_version_hash_changes_when_schema_changes() {
        let old_chat = old_messages::ChatMessage {
            sender: "Test".to_string(),
            content: "Hello".to_string(),
            timestamp: 123,
            version: 1,
        };
        let old_envelope = SmsgEnvelope::new(old_chat);

        let new_chat = new_messages::ChatMessage {
            sender: "Test".to_string(),
            content: "Hello".to_string(),
            timestamp: 123,
        };
        let new_envelope = SmsgEnvelope::new(new_chat);

        assert_ne!(*old_envelope.version_hash(), *new_envelope.version_hash());
    }

    #[test]
    fn test_name_hash_identifies_message_type() {
        let old_chat = old_messages::ChatMessage {
            sender: "Test".to_string(),
            content: "Hello".to_string(),
            timestamp: 123,
            version: 1,
        };
        let old_envelope = SmsgEnvelope::new(old_chat);

        let new_chat = new_messages::ChatMessage {
            sender: "Test".to_string(),
            content: "Hello".to_string(),
            timestamp: 123,
        };
        let new_envelope = SmsgEnvelope::new(new_chat);

        assert_eq!(*old_envelope.name_hash(), *new_envelope.name_hash());
    }

    #[test]
    fn test_different_message_types_have_different_name_hashes() {
        let chat = new_messages::ChatMessage {
            sender: "Test".to_string(),
            content: "Hello".to_string(),
            timestamp: 123,
        };
        let chat_envelope = SmsgEnvelope::new(chat);

        let pos = new_messages::Position {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let pos_envelope = SmsgEnvelope::new(pos);

        assert_ne!(*chat_envelope.name_hash(), *pos_envelope.name_hash());
    }

    #[test]
    fn test_version_verify_method() {
        let msg = new_messages::ChatMessage {
            sender: "Test".to_string(),
            content: "Version check".to_string(),
            timestamp: 456,
        };
        let envelope = SmsgEnvelope::new(msg);

        let correct_version = new_messages::ChatMessage::version_hash();
        assert!(envelope.verify_version(&correct_version));

        let wrong_version = old_messages::ChatMessage::version_hash();
        assert!(!envelope.verify_version(&wrong_version));
    }

    #[test]
    fn test_name_verify_method() {
        let msg = new_messages::ChatMessage {
            sender: "Test".to_string(),
            content: "Name check".to_string(),
            timestamp: 789,
        };
        let envelope = SmsgEnvelope::new(msg);

        let correct_name_hash = new_messages::ChatMessage::name_hash();
        assert!(envelope.verify_name(&correct_name_hash));

        let wrong_name_hash = new_messages::Position::name_hash();
        assert!(!envelope.verify_name(&wrong_name_hash));
    }

    #[test]
    fn test_serialize_with_old_version_store_hash_for_later_verification() {
        let original_msg = old_messages::ChatMessage {
            sender: "Alice".to_string(),
            content: "Old message".to_string(),
            timestamp: 1000,
            version: 1,
        };

        let envelope = SmsgEnvelope::new(original_msg);
        let stored_version_hash = *envelope.version_hash();
        let stored_name_hash = *envelope.name_hash();

        let serialized = z_serialize(&envelope.payload);

        let new_msg = new_messages::ChatMessage {
            sender: "Alice".to_string(),
            content: "Old message".to_string(),
            timestamp: 1000,
        };
        let new_envelope = SmsgEnvelope::new(new_msg);

        assert_ne!(stored_version_hash, *new_envelope.version_hash());
        assert_eq!(stored_name_hash, *new_envelope.name_hash());

        assert!(!new_envelope.verify_version(&stored_version_hash));
        assert!(new_envelope.verify_name(&stored_name_hash));
    }

    #[test]
    fn test_into_parts_includes_name_hash() {
        let msg = new_messages::RobotState {
            name: "Bot".to_string(),
            position: new_messages::Position { x: 1.0, y: 2.0, z: 3.0 },
            status: 5,
        };
        let envelope = SmsgEnvelope::new(msg);

        let (version_hash, name_hash, payload) = envelope.into_parts();

        assert_eq!(version_hash, new_messages::RobotState::version_hash());
        assert_eq!(name_hash, new_messages::RobotState::name_hash());
        assert_eq!(payload.name, "Bot");
    }
}
