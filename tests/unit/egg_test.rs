use {
    chrono::Local,
    kurv::kurv::{Egg, EggStateUpsert, EggStatus},
};

#[test]
fn test_egg_state_transitions() {
    let mut egg = Egg {
        name: "test".to_string(),
        command: "echo".to_string(),
        id: Some(1),
        state: None,
        args: None,
        cwd: None,
        env: None,
        paths: None,
        plugin: None,
        plugin_path: None,
    };

    // initially should have no state
    assert!(egg.state.is_none());

    // set as running should create state and set fields
    egg.set_as_running(1234);
    assert!(egg.is_running());
    assert_eq!(egg.state.as_ref().unwrap().pid, 1234);
    assert!(egg.state.as_ref().unwrap().start_time.is_some());

    // set as stopped
    egg.set_as_stopped();
    assert!(egg.is_stopped());
    assert_eq!(egg.state.as_ref().unwrap().pid, 0);
    assert!(egg.state.as_ref().unwrap().start_time.is_none());
}

#[test]
fn test_egg_should_spawn() {
    let mut egg = Egg {
        name: "test".to_string(),
        command: "echo".to_string(),
        id: Some(1),
        state: None,
        args: None,
        cwd: None,
        env: None,
        paths: None,
        plugin: None,
        plugin_path: None,
    };

    // new eggs without state should spawn
    assert!(egg.should_spawn());

    // set as running - should NOT spawn
    egg.set_status(EggStatus::Running);
    assert!(!egg.should_spawn());

    // set as errored - should spawn (retry)
    egg.set_status(EggStatus::Errored);
    assert!(egg.should_spawn());

    // set as pending - should spawn
    egg.set_status(EggStatus::Pending);
    assert!(egg.should_spawn());

    // set as stopped - should NOT spawn
    egg.set_status(EggStatus::Stopped);
    assert!(!egg.should_spawn());
}

#[test]
fn test_egg_upsert_state() {
    let mut egg = Egg {
        name: "test".to_string(),
        command: "echo".to_string(),
        id: Some(1),
        state: None,
        args: None,
        cwd: None,
        env: None,
        paths: None,
        plugin: None,
        plugin_path: None,
    };

    // upsert on egg without state should create state
    egg.upsert_state(EggStateUpsert {
        status: Some(EggStatus::Running),
        pid: Some(5678),
        start_time: Some(Local::now()),
        try_count: Some(1),
        error: Some("test error".to_string()),
    });

    let state = egg.state.as_ref().unwrap();
    assert_eq!(state.status, EggStatus::Running);
    assert_eq!(state.pid, 5678);
    assert_eq!(state.try_count, 1);
    assert_eq!(state.error, Some("test error".to_string()));

    // partial upsert should only update specified fields
    egg.upsert_state(EggStateUpsert {
        status: Some(EggStatus::Stopped),
        pid: None,
        start_time: None,
        try_count: None,
        error: None,
    });

    let state = egg.state.as_ref().unwrap();
    assert_eq!(state.status, EggStatus::Stopped);
    assert_eq!(state.pid, 5678); // should remain unchanged
    assert_eq!(state.try_count, 1); // should remain unchanged
}

#[test]
fn test_egg_set_as_errored() {
    let mut egg = Egg {
        name: "test".to_string(),
        command: "echo".to_string(),
        id: Some(1),
        state: None,
        args: None,
        cwd: None,
        env: None,
        paths: None,
        plugin: None,
        plugin_path: None,
    };

    // set as running first
    egg.set_as_running(1234);
    assert_eq!(egg.state.as_ref().unwrap().try_count, 0);

    // set as errored should increment try count
    egg.set_as_errored("Process crashed".to_string());
    assert_eq!(egg.state.as_ref().unwrap().status, EggStatus::Errored);
    assert_eq!(egg.state.as_ref().unwrap().pid, 0);
    assert_eq!(egg.state.as_ref().unwrap().try_count, 1);
    assert_eq!(egg.state.as_ref().unwrap().error, Some("Process crashed".to_string()));
}

#[test]
fn test_egg_status_checks() {
    let mut egg = Egg {
        name: "test".to_string(),
        command: "echo".to_string(),
        id: Some(1),
        state: None,
        args: None,
        cwd: None,
        env: None,
        paths: None,
        plugin: None,
        plugin_path: None,
    };

    egg.set_status(EggStatus::Running);
    assert!(egg.is_running());
    assert!(!egg.is_stopped());
    assert!(!egg.is_pending_removal());
    assert!(!egg.is_restarting());

    egg.set_status(EggStatus::Stopped);
    assert!(!egg.is_running());
    assert!(egg.is_stopped());

    egg.set_status(EggStatus::PendingRemoval);
    assert!(egg.is_pending_removal());

    egg.set_status(EggStatus::Restarting);
    assert!(egg.is_restarting());
}

#[test]
fn test_egg_reset_state() {
    let mut egg = Egg {
        name: "test".to_string(),
        command: "echo".to_string(),
        id: Some(1),
        state: None,
        args: None,
        cwd: None,
        env: None,
        paths: None,
        plugin: None,
        plugin_path: None,
    };

    // set as running with some state
    egg.set_as_running(1234);
    egg.set_error("some error".to_string());

    // reset state
    egg.reset_state();

    let state = egg.state.as_ref().unwrap();
    assert_eq!(state.status, EggStatus::Pending);
    assert_eq!(state.pid, 0);
    assert_eq!(state.error, Some("".to_string()));
    assert!(state.start_time.is_none());
}

#[test]
fn test_egg_increment_and_reset_try_count() {
    let mut egg = Egg {
        name: "test".to_string(),
        command: "echo".to_string(),
        id: Some(1),
        state: None,
        args: None,
        cwd: None,
        env: None,
        paths: None,
        plugin: None,
        plugin_path: None,
    };

    egg.set_status(EggStatus::Pending);
    assert_eq!(egg.state.as_ref().unwrap().try_count, 0);

    egg.increment_try_count();
    assert_eq!(egg.state.as_ref().unwrap().try_count, 1);

    egg.increment_try_count();
    assert_eq!(egg.state.as_ref().unwrap().try_count, 2);

    egg.reset_try_count();
    assert_eq!(egg.state.as_ref().unwrap().try_count, 0);
}
