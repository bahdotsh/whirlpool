use anyhow::{bail, Context};
use std::{
    collections::HashMap,
    io::{StdoutLock, Write},
};
use crate::payload::Payload;

mod payload;

use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;

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


pub struct EchoNode {
    pub id: usize,
    pub value: usize,
    pub known: HashMap<String, Vec<usize>>,
}

impl EchoNode {
    pub fn step(&mut self, input: Message, output: &mut StdoutLock) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Add { delta } => {
                self.value += delta;
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::AddOk,
                    },
                };
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to Read")?;
                output.write_all(b"\n").context("write trailing line")?;
            }
            Payload::Broadcast { .. } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::BroadcastOk,
                    },
                };
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to Read")?;
                output.write_all(b"\n").context("write trailing line")?;
            }
            Payload::Topology { topology } => {
                self.known = topology;
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::TopologyOk,
                    },
                };
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to topology")?;
                output.write_all(b"\n").context("write trailing line")?;
            }
            Payload::Read { .. } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::ReadOk {
                            value: self.value.clone(),
                        },
                    },
                };

                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to Read")?;
                output.write_all(b"\n").context("write trailing line")?;
            }
            Payload::Generate { .. } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::GenerateOk {
                            id: Uuid::new_v4().to_string(),
                        },
                    },
                };
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to init")?;
                output.write_all(b"\n").context("write trailing new line")?;
            }
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
            Payload::GenerateOk { .. } => bail!("recieved generate_ok Message"),
            Payload::ReadOk { .. } => bail!("recieved read_ok Message"),
            Payload::BroadcastOk { .. } => bail!("recieved BroadcastOk Message"),
            Payload::TopologyOk { .. } => bail!("recieved TopologyOk Message"),
            Payload::AddOk { .. } => bail!("recieved AddOk Message"),
        }
        self.id += 1;
        Ok(())
    }
}
