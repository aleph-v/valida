use crate::config::StarkConfig;
use crate::program::ProgramROM;
use crate::proof::MachineProof;
use crate::AdviceProvider;
use p3_field::Field;

pub trait Machine<F: Field>: Sync {
    fn run<Adv>(&mut self, program: &ProgramROM<i32>, advice: &mut Adv)
    where
        Adv: AdviceProvider;

    fn prove<SC>(&self, config: &SC) -> MachineProof<SC>
    where
        SC: StarkConfig<Val = F>;

    fn verify<SC>(config: &SC, proof: &MachineProof<SC>) -> Result<(), ()>
    where
        SC: StarkConfig<Val = F>;
}
