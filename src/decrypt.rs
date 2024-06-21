use tls_parser::{parse_tls_plaintext, Err};

pub fn handle_tls_packet(tls_data: &[u8]) {
    let mut remaining_data = tls_data.as_ref();
    while !remaining_data.is_empty() {
        match parse_tls_plaintext(tls_data) {
            Ok((remaining, tls_message)) => {
                println!(" *** Parsed TLS message: {:#?}", tls_message);
                println!(" *** Remaining data: {:?}", remaining);
                remaining_data = remaining;
            }
            Err(Err::Incomplete(needed)) => {
                println!(" *** Incomplete TLS data, need more: {:?}", needed);
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!(" *** Failed to parse TLS data: {:?}", e);
            }
        }
    }
}
