use serde_json::{Map, Value};

pub fn render_json_markdown(value: &Value) -> String {
    let mut markdown = String::new();
    push_line(&mut markdown, "# Cardano Transaction");
    push_line(&mut markdown, "");

    let Some(root) = value.as_object() else {
        render_value(&mut markdown, value, 0);
        return markdown;
    };

    if let Some(transaction_hash) = root.get("transaction_hash") {
        push_line(
            &mut markdown,
            &format!("**Transaction Hash:** {}", scalar(transaction_hash)),
        );
        push_line(&mut markdown, "");
    }

    if let Some(transaction) = root.get("transaction") {
        push_line(&mut markdown, "## Transaction");
        render_transaction(&mut markdown, transaction);
    }

    for (key, value) in root {
        if key == "transaction_hash" || key == "transaction" {
            continue;
        }
        render_entry(&mut markdown, key, value, 0);
    }

    markdown
}

fn render_transaction(markdown: &mut String, transaction: &Value) {
    let Some(object) = transaction.as_object() else {
        render_value(markdown, transaction, 0);
        return;
    };

    if let Some(body) = object.get("body") {
        push_line(markdown, "");
        push_line(markdown, "### Body");
        render_body(markdown, body);
    }

    if let Some(witness_set) = object.get("witness_set") {
        push_line(markdown, "");
        push_line(markdown, "### Witness Set");
        render_value(markdown, witness_set, 0);
    }

    for key in ["is_valid", "auxiliary_data"] {
        if let Some(value) = object.get(key) {
            match key {
                "auxiliary_data" => {
                    push_line(markdown, "");
                    push_line(markdown, "### Auxiliary Data");
                    render_value(markdown, value, 0);
                }
                _ => render_entry(markdown, key, value, 0),
            }
        }
    }

    for (key, value) in object {
        if matches!(
            key.as_str(),
            "body" | "witness_set" | "is_valid" | "auxiliary_data"
        ) {
            continue;
        }
        render_entry(markdown, key, value, 0);
    }
}

fn render_body(markdown: &mut String, body: &Value) {
    let Some(object) = body.as_object() else {
        render_value(markdown, body, 0);
        return;
    };

    if let Some(inputs) = object.get("inputs") {
        push_line(markdown, "");
        push_line(markdown, "#### Inputs");
        render_value(markdown, inputs, 0);
    }

    if let Some(outputs) = object.get("outputs") {
        push_line(markdown, "");
        push_line(markdown, "#### Outputs");
        render_value(markdown, outputs, 0);
    }

    push_line(markdown, "");
    push_line(markdown, "#### Other Body Fields");
    for (key, value) in object {
        if key == "inputs" || key == "outputs" {
            continue;
        }
        render_entry(markdown, key, value, 0);
    }
}

fn render_value(markdown: &mut String, value: &Value, depth: usize) {
    match value {
        Value::Object(object) => render_object(markdown, object, depth),
        Value::Array(array) => render_array(markdown, array, depth),
        _ => push_line(markdown, &format!("{}- {}", indent(depth), scalar(value))),
    }
}

fn render_object(markdown: &mut String, object: &Map<String, Value>, depth: usize) {
    for (key, value) in object {
        render_entry(markdown, key, value, depth);
    }
}

fn render_entry(markdown: &mut String, key: &str, value: &Value, depth: usize) {
    if key == "multiasset" {
        render_multiasset(markdown, value, depth, key);
        return;
    }

    match value {
        Value::Object(child) if child.is_empty() => {
            push_line(
                markdown,
                &format!("{}- **{}:** {{}}", indent(depth), title_key(key)),
            );
        }
        Value::Object(child) => {
            push_line(
                markdown,
                &format!("{}- **{}**", indent(depth), title_key(key)),
            );
            render_object(markdown, child, depth + 1);
        }
        Value::Array(items) if items.is_empty() => {
            push_line(
                markdown,
                &format!("{}- **{}:** []", indent(depth), title_key(key)),
            );
        }
        Value::Array(items) => {
            push_line(
                markdown,
                &format!("{}- **{}**", indent(depth), title_key(key)),
            );
            render_array(markdown, items, depth + 1);
        }
        _ => push_line(
            markdown,
            &format!(
                "{}- **{}:** {}",
                indent(depth),
                title_key(key),
                scalar(value)
            ),
        ),
    }
}

fn render_array(markdown: &mut String, items: &[Value], depth: usize) {
    for (index, item) in items.iter().enumerate() {
        match item {
            Value::Object(object) => {
                push_line(markdown, &format!("{}- **#{}**", indent(depth), index));
                render_object(markdown, object, depth + 1);
            }
            Value::Array(array) => {
                push_line(markdown, &format!("{}- **#{}**", indent(depth), index));
                render_array(markdown, array, depth + 1);
            }
            _ => push_line(markdown, &format!("{}- {}", indent(depth), scalar(item))),
        }
    }
}

fn render_multiasset(markdown: &mut String, value: &Value, depth: usize, key: &str) {
    let Some(policies) = value.as_object() else {
        push_line(
            markdown,
            &format!(
                "{}- **{}:** {}",
                indent(depth),
                title_key(key),
                scalar(value)
            ),
        );
        return;
    };

    if policies.is_empty() {
        push_line(
            markdown,
            &format!("{}- **{}:** {{}}", indent(depth), title_key(key)),
        );
        return;
    }

    push_line(
        markdown,
        &format!("{}- **{}**", indent(depth), title_key(key)),
    );
    for (policy_id, assets) in policies {
        push_line(
            markdown,
            &format!("{}- policy `{}`", indent(depth + 1), policy_id),
        );
        let Some(asset_map) = assets.as_object() else {
            render_value(markdown, assets, depth + 2);
            continue;
        };
        for (asset_hex, quantity) in asset_map {
            push_line(
                markdown,
                &format!(
                    "{}- asset `{}` ({}) qty {}",
                    indent(depth + 2),
                    asset_hex,
                    human_asset_name(asset_hex),
                    scalar(quantity)
                ),
            );
        }
    }
}

fn human_asset_name(asset_hex: &str) -> String {
    let Some(bytes) = decode_hex(asset_hex) else {
        return "`invalid hex`".to_string();
    };

    match String::from_utf8(bytes) {
        Ok(value)
            if !value.is_empty()
                && value.chars().all(|character| {
                    !character.is_control() || character == '\n' || character == '\t'
                }) =>
        {
            format!("`{}`", value.replace('`', "\\`"))
        }
        _ => "`non-utf8`".to_string(),
    }
}

fn decode_hex(value: &str) -> Option<Vec<u8>> {
    if !value.len().is_multiple_of(2) {
        return None;
    }

    value
        .as_bytes()
        .chunks(2)
        .map(|chunk| {
            let high = hex_digit(chunk[0])?;
            let low = hex_digit(chunk[1])?;
            Some((high << 4) | low)
        })
        .collect()
}

fn hex_digit(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

fn scalar(value: &Value) -> String {
    match value {
        Value::Null => "`null`".to_string(),
        Value::Bool(value) => format!("`{value}`"),
        Value::Number(value) => format!("`{value}`"),
        Value::String(value) => format!("`{}`", value.replace('`', "\\`")),
        Value::Array(_) | Value::Object(_) => unreachable!("handled by render_value"),
    }
}

fn title_key(key: &str) -> String {
    key.split('_')
        .map(|word| {
            let mut characters = word.chars();
            match characters.next() {
                Some(first) => first.to_uppercase().collect::<String>() + characters.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn indent(depth: usize) -> String {
    "  ".repeat(depth)
}

fn push_line(markdown: &mut String, line: &str) {
    markdown.push_str(line);
    markdown.push('\n');
}
