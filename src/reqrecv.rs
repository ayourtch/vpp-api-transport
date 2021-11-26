#![allow(
    dead_code,
    unused_mut,
    unused_variables,
    unused_must_use,
    non_camel_case_types,
    unused_imports
)]
use vpp_api_message::VppApiMessage;
use bincode::Options;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::ops::Add;
use std::time::{Duration, SystemTime};
use crate::VppApiTransport;


fn get_encoder() -> impl bincode::config::Options {
    bincode::DefaultOptions::new()
        .with_big_endian()
        .with_fixint_encoding()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlPing {
    pub client_index: u32,
    pub context: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlPingReply {
    pub context: u32,
    pub retval: i32,
    pub client_index: u32,
    pub vpe_pid: u32,
}

pub fn send_recv_one<
    'a,
    T: Serialize + Deserialize<'a> + VppApiMessage,
    TR: Serialize + DeserializeOwned + VppApiMessage,
>(
    m: &T,
    t: &mut dyn VppApiTransport,
) -> TR {
    send_recv_msg(
        &T::get_message_name_and_crc(),
        m,
        t,
        &TR::get_message_name_and_crc(),
    )
}
pub fn send_recv_many<
    'a,
    T: Serialize + Deserialize<'a> + VppApiMessage,
    TR: Serialize + DeserializeOwned + VppApiMessage + std::fmt::Debug + Clone,
>(
    m: &T,
    t: &mut dyn VppApiTransport,
) -> Vec<TR> {
    send_bulk_msg(
        &T::get_message_name_and_crc(),
        m,
        t,
        &TR::get_message_name_and_crc(),
    )
}

pub fn send_recv_msg<'a, T: Serialize + Deserialize<'a>, TR: Serialize + DeserializeOwned>(
    name: &str,
    m: &T,
    t: &mut dyn VppApiTransport,
    reply_name: &str,
) -> TR {
    let vl_msg_id = t.get_msg_index(name).unwrap();
    let reply_vl_msg_id = t.get_msg_index(reply_name).unwrap();
    let enc = get_encoder();
    let mut v = enc.serialize(&vl_msg_id).unwrap();
    let enc = get_encoder();
    let msg = enc.serialize(&m).unwrap();

    v.extend_from_slice(&msg);
    t.write(&v);
    loop {
        let res = t.read_one_msg_id_and_msg();
        // dbg!(&res);
        if let Ok((msg_id, data)) = res {
            println!("id: {} data: {:x?}", msg_id, &data);
            if msg_id == reply_vl_msg_id {
                let res = get_encoder()
                    .allow_trailing_bytes()
                    .deserialize::<TR>(&data)
                    .unwrap();
                return res;
            }
        } else {
            panic!("Result is an error: {:?}", &res);
        }
    }
}
pub fn send_bulk_msg<
    'a,
    T: Serialize + Deserialize<'a>,
    TR: Serialize + DeserializeOwned + std::fmt::Debug + Clone,
>(
    name: &str,
    m: &T,
    t: &mut dyn VppApiTransport,
    reply_name: &str,
) -> Vec<TR> {
    let control_ping_id = t.get_msg_index("control_ping_51077d14").unwrap();
    let control_ping_id_reply = t.get_msg_index("control_ping_reply_f6b0b8ca").unwrap();
    let vl_msg_id = t.get_msg_index(name).unwrap();
    let reply_vl_msg_id = t.get_msg_index(reply_name).unwrap();
    let enc = get_encoder();
    let mut v = enc.serialize(&vl_msg_id).unwrap();
    let enc = get_encoder();
    let msg = enc.serialize(&m).unwrap(); /////
    let control_ping = ControlPing {
        client_index: t.get_client_index(),
        context: 0,
    };
    let enc = get_encoder();
    let mut c = enc.serialize(&control_ping_id).unwrap();
    let enc = get_encoder();
    let control_ping_message = enc.serialize(&control_ping).unwrap();
    c.extend_from_slice(&control_ping_message);
    v.extend_from_slice(&msg);
    let mut out: Vec<u8> = vec![];
    t.write(&v); // Dump message
    t.write(&c); // Ping message
    dbg!(control_ping_id_reply);
    let mut out: Vec<TR> = vec![];
    let mut count = 0;
    loop {
        println!("Reached loop");
        let res = t.read_one_msg_id_and_msg();
        if let Ok((msg_id, data)) = res {
            println!("id: {} data: {:x?}", msg_id, &data);
            println!("{}", data.len());
            if msg_id == control_ping_id_reply {
                return out;
            }
            if msg_id == reply_vl_msg_id {
                println!("Received the intended message");
                let res = get_encoder()
                    .allow_trailing_bytes()
                    .deserialize::<TR>(&data)
                    .unwrap();
                println!("Next thing will be the reply");
                out.extend_from_slice(&[res]);
            } else {
                println!("Checking the next message for the reply id");
            }
        } else {
            panic!("Result is an error: {:?}", &res);
        }
    }
}
