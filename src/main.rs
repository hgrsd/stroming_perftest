use std::time::Instant;

use stroming::{
    MemoryStreamStore, Message, ReadDirection, ReadFromStream, StreamVersion, WriteResult,
    WriteToStream,
};

fn main() {
    let events: Vec<Message> = (0..10000)
        .map(|_| Message {
            message_type: "TestMessage".to_string(),
            data: r#"{"some":"key", "another": "key", "what_an": "excellent event"}"#
                .as_bytes()
                .to_vec(),
        })
        .collect();
    let mut store = MemoryStreamStore::new();
    let start = Instant::now();
    let result = store.write_to_stream("TestStream-1", StreamVersion::NoStream, &events);
    let duration = start.elapsed();

    if let WriteResult::WrongExpectedVersion = result {
        panic!("Error");
    }

    let (version, stream) = store.read_from_stream("TestStream-1", ReadDirection::Forwards);
    assert_eq!(stream.len(), 10000);
    assert_eq!(version, StreamVersion::Revision(9999));

    println!("Inserted 10000 events in {:?}", duration);
}
