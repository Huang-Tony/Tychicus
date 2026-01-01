#[starknet::interface]
trait ITychicusVerifier<TContractState> {
    fn verify_compliance(ref self: TContractState, image_id: felt252);
}

#[starknet::contract]
mod TychicusVerifier {
    #[storage]
    struct Storage {
        verified_audits: LegacyMap::<felt252, bool>,
    }

    #[abi(embed_v0)]
    impl TychicusVerifierImpl of super::ITychicusVerifier<ContractState> {
        fn verify_compliance(ref self: ContractState, image_id: felt252) {
            self.verified_audits.write(image_id, true);
        }
    }
}
