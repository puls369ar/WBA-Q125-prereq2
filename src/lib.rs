mod programs;

#[cfg(test)]
mod tests {

    use crate::programs::Turbin3_prereq::{WbaPrereqProgram, CompleteArgs, UpdateArgs};
    use solana_client::rpc_client::RpcClient;
    use solana_program::{
        pubkey::Pubkey,
        system_instruction::transfer,
        system_program,
    };

    use solana_sdk::{
        signature::{Keypair, Signer, read_keypair_file},
        transaction::Transaction,
    };

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);

        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");

        let prereq = WbaPrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);

        let args = CompleteArgs { github: b"puls369ar".to_vec() };

        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        if !rpc_client.get_account(&prereq).is_ok() {
            println!("Account does not exist. Proceeding with allocation and initialization.");

            let transaction = WbaPrereqProgram::complete(
                &[&signer.pubkey(), &prereq, &system_program::id()],
                &args,
                Some(&signer.pubkey()),
                &[&signer],
                blockhash,
            );

            let signature = rpc_client
                .send_and_confirm_transaction(&transaction)
                .expect("Failed to send transaction");

            println!(
                "Success! Check out your TX here:https://explorer.solana.com/tx/{}/?cluster=devnet",
                signature
            );
        } else {
            println!("Account already exists. Skipping allocation and initialization.");

            // Handle the case where the PDA already exists
            // You can perform updates, re-use the existing account, or return an error.
            let update_args = UpdateArgs { github: b"puls369ar".to_vec() };

            let transaction = WbaPrereqProgram::update(
                &[&signer.pubkey(), &prereq, &system_program::id()],
                &update_args,
                Some(&signer.pubkey()),
                &[&signer],
                blockhash,
            );

            let signature = rpc_client
                .send_and_confirm_transaction(&transaction)
                .expect("Failed to send update transaction");

            println!(
                "Success! Check out your Update TX here:https://explorer.solana.com/tx/{}/?cluster=devnet",
                signature
            );
        }
    }
}

