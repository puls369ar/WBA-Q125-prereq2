
#[cfg(test)]
mod tests {
	use solana_client::rpc_client::RpcClient; use solana_program::{
		pubkey::Pubkey,
		system_instruction::transfer
	};
	
	use solana_sdk::{
		message::Message,
		signature::{Keypair, Signer, read_keypair_file}, transaction::Transaction
	};
	
	use std::str::FromStr;

	const RPC_URL: &str = "https://api.devnet.solana.com";	
	
	#[test]
	fn transferall() {
		
		let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
		
		let to_pubkey = Pubkey::from_str("83wPgXhEscwj6aMZy3cDZB98HE9HS6E9ax4PcitMiSA2").unwrap();
		
		
		let rpc_client = RpcClient::new(RPC_URL);
		let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");	
		let balance = rpc_client
		.get_balance(&keypair.pubkey())
		.expect("Failed to get balance");
		
		
		let message = Message::new_with_blockhash(&[transfer( &keypair.pubkey(), &to_pubkey, balance,)], Some(&keypair.pubkey()), &recent_blockhash);
		
		let fee = rpc_client
			.get_fee_for_message(&message) .expect("Failed to get fee calculator");
	
		let transaction = Transaction::new_signed_with_payer(&[transfer( &keypair.pubkey(), &to_pubkey, balance - fee,)], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash);
	

		let signature = rpc_client
			.send_and_confirm_transaction(&transaction)
			.expect("Failed to send transaction");
		
		println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
	}
}