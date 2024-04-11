use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> entrypoint::ProgramResult {
    let ix_count = instruction_data[0] as usize;
    let (accounts_sizes, rest) = instruction_data[1..].split_at(ix_count);
    let (data_sizes, ix_datas) = rest.split_at(2 * ix_count);

    let mut accounts_start = 0;
    let mut data_start = 0;
    for i in 0..ix_count {
        let accounts_size = accounts_sizes[i] as usize;
        let data_size = u16::from_le_bytes(data_sizes[2 * i..2 * (i + 1)].try_into().unwrap()) as usize;

        let ix_accounts = &accounts[accounts_start..accounts_start + accounts_size];
        let ix_data = &ix_datas[data_start..data_start + data_size];

        let instruction = Instruction {
            program_id: *ix_accounts[0].key,
            accounts: ix_accounts[1..]
                .iter()
                .map(|ai| AccountMeta {
                    pubkey: *ai.key,
                    is_signer: ai.is_signer,
                    is_writable: ai.is_writable,
                })
                .collect::<Vec<_>>(),
            data: ix_data.to_vec(),
        };

        invoke(&instruction, ix_accounts)?;

        accounts_start += accounts_size;
        data_start += data_size;
    }

    Ok(())
}
