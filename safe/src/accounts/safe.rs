use solana_client_gen::solana_sdk::pubkey::Pubkey;

/// Safe is the account representing an instance of this program (akin to an
/// SPL token's "mint").
#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct Safe {
    /// The mint of the SPL token the safe is storing, e.g., the SRM mint.
    pub mint: Pubkey,
    /// The key with the ability to migrate or change the authority.
    pub authority: Pubkey,
    /// Is `true` if this structure has been initialized
    pub initialized: bool,
    /// The nonce to use for the program-derived-address owning the Safe's
    /// token vault.
    pub nonce: u8,
}

serum_common::packable!(Safe);

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;
    use solana_client_gen::solana_sdk::program_error::ProgramError;
    use solana_client_gen::solana_sdk::signature::{Keypair, Signer};

    #[test]
    fn pack_unpack() {
        let mint = Keypair::generate(&mut OsRng).pubkey();
        let authority = Keypair::generate(&mut OsRng).pubkey();
        let initialized = true;
        let safe = Safe {
            mint: mint.clone(),
            authority: authority.clone(),
            initialized,
            nonce: 33,
        };

        let mut dst = Vec::new();
        dst.resize(Safe::default().size().unwrap() as usize, 0u8);
        Safe::pack(safe, &mut dst).unwrap();

        let new_safe = Safe::unpack(&dst).unwrap();

        assert_eq!(new_safe.mint, mint);
        assert_eq!(new_safe.authority, authority);
        assert_eq!(new_safe.initialized, initialized);
        assert_eq!(new_safe.nonce, 33);
    }

    #[test]
    fn unpack_too_small() {
        let size = Safe::default().size().unwrap() - 10;
        let data = vec![0; size as usize];
        let result = Safe::unpack(&data);
        match result {
            Ok(_) => panic!("expect error"),
            Err(e) => assert_eq!(e, ProgramError::InvalidAccountData),
        }
    }

    #[test]
    fn unpack_too_large() {
        let size = Safe::default().size().unwrap() + 10;
        let data = vec![0; size as usize];
        let result = Safe::unpack(&data);
        match result {
            Ok(_) => panic!("expect error"),
            Err(e) => assert_eq!(e, ProgramError::InvalidAccountData),
        }
    }

    #[test]
    fn unpack_zeroes_size() {
        let og_size = Safe::default().size().unwrap();
        let zero_data = vec![0; og_size as usize];
        let r = Safe::unpack_unchecked(&zero_data).unwrap();
        assert_eq!(r.mint, Pubkey::new(&[0; 32]));
        assert_eq!(r.initialized, false);
        assert_eq!(r.authority, Pubkey::new(&[0; 32]));
        assert_eq!(r.nonce, 0);
    }
}
