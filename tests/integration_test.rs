use soul_msg::smsg;
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
