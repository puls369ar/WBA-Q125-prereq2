
#[cfg(test)]
mod tests {
	use solana_client::rpc_client::RpcClient; use solana_program::{
		pubkey::Pubkey,
		system_instruction::transfer
	};
	
	use solana_sdk::{
		signature::{Keypair, Signer, read_keypair_file}, transaction::Transaction
	};
	
	use std::str::FromStr;

	const RPC_URL: &str = "https://api.devnet.solana.com";	
	
	#[test]
	fn transfer-some() {
		let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
		
		let to_pubkey = Pubkey::from_str("<your Turbin3 public key>").unwrap();
		
		
		let rpc_client = RpcClient::new(RPC_URL);
		
		
		let recent_blockhash = rpc_client .get_latest_blockhash() .expect("Failed to get recent blockhash");
		
		
		let transaction = Transaction::new_signed_with_payer( &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash);
		
		
		let signature = rpc_client
			.send_and_confirm_transaction(&transaction)
			.expect("Failed to send transaction");
		
		println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
	}
}
