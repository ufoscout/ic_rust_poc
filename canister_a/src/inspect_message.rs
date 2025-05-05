use ic_cdk::inspect_message;

#[inspect_message]
fn inspect_messages() {
    inspect_message_impl()
}

fn inspect_message_impl() {
    let method = ic_cdk::api::msg_method_name();

    let check_result = match method.as_str() {
        "protected_by_inspect_message" => Err("NotAllowed"),
        _ => Ok(()),
    };

    if let Err(e) = check_result {
        ic_cdk::trap(&format!("Call rejected by inspect check: {e:?}"));
    } else {
        ic_cdk::api::accept_message();
    }
}
