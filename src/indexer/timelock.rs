abigen!(
    Timelock,
    r#"[
        event CallScheduled(bytes32 id)
        event CallExecuted(bytes32 id)
    ]"#
);
