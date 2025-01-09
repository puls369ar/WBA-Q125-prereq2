
#[cfg(test)]
mod tests {
	use bs58;
	use std::io::{self, BufRead};



	#[test]
	fn base58_to_wallet() {
		println!("Input your private key as base58:");
		let stdin = io::stdin();
		let base58 = stdin.lock().lines().next().unwrap().unwrap(); println!("Your wallet file is:");
		let wallet = bs58::decode(base58).into_vec().unwrap(); println!("{:?}", wallet);
	}

}
