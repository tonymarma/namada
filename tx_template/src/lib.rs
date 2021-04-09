use anoma_data_template::*;
use anoma_vm_env::{transaction, tx_prelude::*};

transaction! {
    fn apply_tx(tx_data: memory::Data) {
        let tx = TxData::try_from_slice(&tx_data[..]).unwrap();
        for Transfer {
            source,
            target,
            token,
            amount,
        } in tx.transfers
        {
            apply_transfer(source, target, token, amount);
        }
    }
}

fn apply_transfer(src: String, dest: String, token: String, amount: u64) -> bool {
    let src_key = vec![format!("@{}", src), String::from("balance"), token.clone()].join("/");
    let dest_key = vec![format!("@{}", dest), String::from("balance"), token].join("/");

    let src_bal_buf: Vec<u8> = Vec::with_capacity(0);
    let result = unsafe {
        read(
            src_key.as_ptr() as _,
            src_key.len() as _,
            src_bal_buf.as_ptr() as _,
        )
    };
    if result == 1 {
        let mut slice = unsafe { slice::from_raw_parts(src_bal_buf.as_ptr(), size_of::<u64>()) };
        let src_bal: u64 = u64::deserialize(&mut slice).unwrap();

        let dest_bal_buf: Vec<u8> = Vec::with_capacity(0);
        let result = unsafe {
            read(
                dest_key.as_ptr() as _,
                dest_key.len() as _,
                dest_bal_buf.as_ptr() as _,
            )
        };
        if result == 1 {
            let mut slice =
                unsafe { slice::from_raw_parts(dest_bal_buf.as_ptr(), size_of::<u64>()) };
            let dest_bal: u64 = u64::deserialize(&mut slice).unwrap();

            let src_new_bal = src_bal - amount;
            let dest_new_bal = dest_bal + amount;
            let mut src_new_bal_buf: Vec<u8> = Vec::with_capacity(0);
            src_new_bal.serialize(&mut src_new_bal_buf).unwrap();
            let mut dest_new_bal_buf: Vec<u8> = Vec::with_capacity(0);
            dest_new_bal.serialize(&mut dest_new_bal_buf).unwrap();

            unsafe {
                write(
                    src_key.as_ptr() as _,
                    src_key.len() as _,
                    src_new_bal_buf.as_ptr() as _,
                    src_new_bal_buf.len() as _,
                )
            };
            unsafe {
                write(
                    dest_key.as_ptr() as _,
                    dest_key.len() as _,
                    dest_new_bal_buf.as_ptr() as _,
                    dest_new_bal_buf.len() as _,
                )
            };
            true
        } else {
            false
        }
    } else {
        false
    }
}
