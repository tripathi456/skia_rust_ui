use skia_rust_ui::StateStore;
use skia_rust_ui::WidgetId;
use pretty_assertions::assert_eq;

#[test]
fn test_empty_store_returns_none() {
    // Red: Empty StateStore should return None for any WidgetId
    let store = StateStore::new();
    let id = WidgetId::new();
    
    assert!(store.get_state(&id).is_none());
}

#[test]
fn test_store_string_type() {
    // Red: Store a String type and retrieve it
    let mut store = StateStore::new();
    let id = WidgetId::new();
    let test_value = "test_string".to_string();
    
    store.set_state(id.clone(), Box::new(test_value.clone()));
    
    let retrieved = store.get_state_as::<String>(&id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), &test_value);
}

#[test]
fn test_store_different_types() {
    // Red: Test storing different types with different WidgetIds
    let mut store = StateStore::new();
    let id1 = WidgetId::new();
    let id2 = WidgetId::new();
    
    let string_value = "test_string".to_string();
    let int_value = 42;
    
    store.set_state(id1.clone(), Box::new(string_value.clone()));
    store.set_state(id2.clone(), Box::new(int_value));
    
    // Check string value
    let retrieved_string = store.get_state_as::<String>(&id1);
    assert!(retrieved_string.is_some());
    assert_eq!(retrieved_string.unwrap(), &string_value);
    
    // Check integer value
    let retrieved_int = store.get_state_as::<i32>(&id2);
    assert!(retrieved_int.is_some());
    assert_eq!(retrieved_int.unwrap(), &int_value);
    
    // Trying to get string as int should return None
    let wrong_type = store.get_state_as::<i32>(&id1);
    assert!(wrong_type.is_none());
}
