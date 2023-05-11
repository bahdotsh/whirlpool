use anyhow::{bail, Context};
use std::io::{StdoutLock, Write};

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    src: String,
    dest: String,
    body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    #[serde(rename = "msg_id")]
    id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

pub struct EchoNode {
    pub id: usize,
}

impl EchoNode {
    pub fn step(&mut self, input: Message, output: &mut StdoutLock) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Init { .. } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::InitOk,
                    },
                };
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to init")?;
                output.write_all(b"\n").context("write trailing new line")?;
            }
            Payload::Echo { echo } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::EchoOk { echo },
                    },
                };
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to init")?;
                output.write_all(b"\n").context("write trailing new line")?;
            }
            Payload::EchoOk { .. } => bail!("recieved init_ok Message"),
            Payload::InitOk { .. } => {}
        }
        self.id += 1;
        Ok(())
    }
}
