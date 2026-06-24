
// use {
//     anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
//     litesvm::LiteSVM,
//     solana_message::{Message, VersionedMessage},
//     solana_signer::Signer,
//     solana_keypair::Keypair,
//     solana_transaction::versioned::VersionedTransaction,
// };

// #[test]
// fn test_initialize() {
//     let program_id = ship_or_slash::id();
//     let payer = Keypair::new();
//     let mut svm = LiteSVM::new();
//     let bytes = include_bytes!("../../../target/deploy/ship_or_slash.so");
//     svm.add_program(program_id, bytes).unwrap();
//     svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    
//     let instruction = Instruction::new_with_bytes(
//         program_id,
//         &ship_or_slash::instruction::Initialize {}.data(),
//         ship_or_slash::accounts::Initialize {}.to_account_metas(None),
//     );

//     let blockhash = svm.latest_blockhash();
//     let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
//     let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer]).unwrap();

//     let res = svm.send_transaction(tx);
//     assert!(res.is_ok());
// }
