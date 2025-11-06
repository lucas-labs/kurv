use {
    kurv::kurv::{Egg, EggState, EggStatus, KurvState},
    std::{collections::BTreeMap, path::PathBuf},
    tempfile::TempDir,
};

#[test]
fn test_state_save_and_load() {
    let temp_dir = TempDir::new().unwrap();
    let state_path = temp_dir.path().join(".kurv");

    // create a state with an egg
    let mut eggs = BTreeMap::new();
    let egg = Egg {
        name: "test-egg".to_string(),
        command: "echo".to_string(),
        id: Some(1),
        state: Some(EggState {
            status: EggStatus::Running,
            start_time: None,
            try_count: 0,
            error: None,
            pid: 1234,
        }),
        args: Some(vec!["hello".to_string()]),
        cwd: Some(PathBuf::from("/tmp")),
        env: None,
        paths: None,
        plugin: None,
        plugin_path: None,
    };

    eggs.insert("test-egg".to_string(), egg);

    let state = KurvState { eggs };

    // save state
    state.save(&state_path).unwrap();
    assert!(state_path.exists());

    // load state
    let loaded = KurvState::load(&state_path).unwrap();
    assert_eq!(loaded.eggs.len(), 1);
    assert!(loaded.eggs.contains_key("test-egg"));

    let loaded_egg = loaded.eggs.get("test-egg").unwrap();
    assert_eq!(loaded_egg.name, "test-egg");
    assert_eq!(loaded_egg.command, "echo");
    assert_eq!(loaded_egg.id, Some(1));
    assert_eq!(loaded_egg.state.as_ref().unwrap().pid, 1234);
}

#[test]
fn test_state_load_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let state_path = temp_dir.path().join(".kurv");

    // should return empty state if file doesn't exist
    let result = KurvState::load(&state_path);
    assert!(result.is_ok());

    let state = result.unwrap();
    assert_eq!(state.eggs.len(), 0);
}

#[test]
fn test_state_assigns_ids() {
    let temp_dir = TempDir::new().unwrap();
    let state_path = temp_dir.path().join(".kurv");

    // create eggs without IDs
    let mut eggs = BTreeMap::new();
    eggs.insert(
        "egg1".to_string(),
        Egg {
            name: "egg1".to_string(),
            command: "echo".to_string(),
            id: None,
            state: None,
            args: None,
            cwd: None,
            env: None,
            paths: None,
            plugin: None,
            plugin_path: None,
        },
    );
    eggs.insert(
        "egg2".to_string(),
        Egg {
            name: "egg2".to_string(),
            command: "ls".to_string(),
            id: None,
            state: None,
            args: None,
            cwd: None,
            env: None,
            paths: None,
            plugin: None,
            plugin_path: None,
        },
    );

    let state = KurvState { eggs };
    state.save(&state_path).unwrap();

    // load and verify IDs are assigned
    let loaded = KurvState::load(&state_path).unwrap();
    assert_eq!(loaded.eggs.len(), 2);

    for (_, egg) in loaded.eggs.iter() {
        assert!(egg.id.is_some());
        assert!(egg.id.unwrap() > 0);
    }
}

#[test]
fn test_state_multiple_eggs() {
    let temp_dir = TempDir::new().unwrap();
    let state_path = temp_dir.path().join(".kurv");

    // create multiple eggs
    let mut eggs = BTreeMap::new();
    for i in 1..=5 {
        eggs.insert(
            format!("egg{}", i),
            Egg {
                name: format!("egg{}", i),
                command: "echo".to_string(),
                id: Some(i),
                state: None,
                args: None,
                cwd: None,
                env: None,
                paths: None,
                plugin: None,
                plugin_path: None,
            },
        );
    }

    let state = KurvState { eggs };
    state.save(&state_path).unwrap();

    // load and verify all eggs are present
    let loaded = KurvState::load(&state_path).unwrap();
    assert_eq!(loaded.eggs.len(), 5);

    for i in 1..=5 {
        let key = format!("egg{}", i);
        assert!(loaded.eggs.contains_key(&key));
        assert_eq!(loaded.eggs.get(&key).unwrap().id, Some(i));
    }
}
