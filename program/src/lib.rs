use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    log::sol_log_compute_units,
    account_info::{ next_account_info, AccountInfo },
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ChatMessage {
    pub archive_id: String
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ChatMessageContainer {
    pub archive_ids: Vec<ChatMessage>
}

// example arweave tx (length 43)
// 1seRanklLU_1VTGkEk7P0xAwMJfA7owA1JHW5KyZKlY
// ReUohI9tEmXQ6EN9H9IkRjY9bSdgql_OdLUCOeMEte0
const DUMMY_TX_ID: &str = "0000000000000000000000000000000000000000000";
pub fn get_init_chat_message() -> ChatMessage {
    ChatMessage{ archive_id: String::from(DUMMY_TX_ID) }
}
pub fn get_init_chat_messages() -> Vec<ChatMessage> {
    let mut messages = Vec::new();
    for _ in 0..10 {
        messages.push(get_init_chat_message());
    }
    return messages;
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    msg!("Start program to save message.");

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    if account.owner != program_id {
        msg!("This account {} is not owned by this program {} and cannot be updated!", account.key, program_id);
    }

    sol_log_compute_units();

    let instruction_data_message = ChatMessage::try_from_slice(instruction_data).map_err(|err| {
        msg!("Attempt to deserialize instruction data has failed. {:?}", err);
        ProgramError::InvalidInstructionData
    })?;
    msg!("Instruction_data message object {:?}", instruction_data_message);

    let mut existing_data_messages = ChatMessageContainer::try_from_slice(&account.data.borrow_mut()).map_err(|err| {
        msg!("Failed to decode account data. {:?}", err);
        ProgramError::InvalidInstructionData
    })?;
    let index = existing_data_messages.archive_ids.iter().position(|p| p.archive_id == String::from(DUMMY_TX_ID)).unwrap(); // find first dummy data entry
    msg!("Existing archive_id {}", existing_data_messages.archive_ids[index].archive_id);
    existing_data_messages.archive_ids[index] = instruction_data_message; // set dummy data to new entry
    msg!("Set existing_data_message");
    let updated_data = existing_data_messages.try_to_vec().expect("Failed to encode data."); // set messages object back to vector data

    // data algorithm for storing data into account and then archiving into Arweave
    // 1. Each ChatMessage object will be prepopulated for txt field having 43 characters (length of a arweave tx).
    // Each ChatMessageContainer will be prepopulated with 10 ChatMessage objects with dummy data.
    // 2. Client will submit an arweave tx for each message; get back the tx id; and submit it to our program.
    // 3. This tx id will be saved to the Solana program and be used for querying back to arweave to get actual data.
    let data = &mut &mut account.data.borrow_mut();
    msg!("Attempting save data.");
    data[..updated_data.len()].copy_from_slice(&updated_data);    
    msg!("ChatMessage has been saved to account data.");
    sol_log_compute_units();

    msg!("End program.");
    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    //use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let data = ChatMessageContainer{ archive_ids: get_init_chat_messages() }; // vec![0; get_init_chat_messages().len()];
        let mut data_data = data.try_to_vec().unwrap();
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data_data,
            &owner,
            false,
            Epoch::default(),
        );
        
        let tx_id = "abcdefghijabcdefghijabcdefghijabcdefghijabc";
        let instruction_data_chat_message = ChatMessage{ archive_id: String::from(tx_id) };
        let instruction_data = instruction_data_chat_message.try_to_vec().unwrap();

        let accounts = vec![account];

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        let result = &ChatMessageContainer::try_from_slice(&accounts[0].data.borrow())
            .unwrap()
            .archive_ids[0].archive_id;
        println!("archive_id {}", result);
        // I added first data and expect it to contain the given data
        assert_eq!(
            String::from(tx_id).eq(result),
            true
        );
        // process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        // assert_eq!(
        //     ChatMessageContainer::try_from_slice(&accounts[0].data.borrow())
        //         .unwrap()
        //         .txt,
        //     1
        // );
        // process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        // assert_eq!(
        //     ChatMessageContainer::try_from_slice(&accounts[0].data.borrow())
        //         .unwrap()
        //         .txt,
        //     2
        // );
    }
}