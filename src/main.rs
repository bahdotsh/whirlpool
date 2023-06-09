use anyhow::Context;
use std::collections::HashMap;
use whirlpool::{EchoNode, Message};

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let mut stdout = std::io::stdout().lock();

    let mut state = EchoNode {
        id: 0,
        value: 0,
        known: HashMap::new(),
    };
    for input in inputs {
        let input = input.context("Maelstrom input could not be deserialized")?;
        state
            .step(input, &mut stdout)
            .context("Node step function failed")?;
    }

    Ok(())
}
