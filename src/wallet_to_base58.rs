
#[cfg(test)]
mod tests {
	use bs58;
	use std::io::{self, BufRead};


	#[test]	
	fn wallet_to_base58() {

		println!("Input your private key as a wallet file byte array:"); 
		let stdin = io::stdin(); let wallet = stdin.lock().lines().next().unwrap().unwrap().trim_start_matches('[').trim_end_matches(']').split(',') .map(|s| s.trim()
			.parse::<u8>().unwrap())
			.collect::<Vec<u8>>();

		println!("Your private key is:");
		let base58 = bs58::encode(wallet).into_string(); println!("{:?}", base58);

	}	
}
