use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::Field,
    circuit::Layouter,
    plonk::{Advice, Column, ConstraintSystem, Instance, Selector},
};

#[derive(Clone, Debug)]
pub struct MastermindConfig {
    guess: [Column<Instance>; 4],
    num_reds: Column<Instance>,
    num_whites: Column<Instance>,
    selector: Selector,
    solution: [Column<Advice>; 4],
    solution_hash: Column<Instance>,
}

#[derive(Clone, Debug)]
pub struct MastermindChip<F: Field> {
    config: MastermindConfig,
    _marker: PhantomData<F>,
}

impl<F: Field> MastermindChip<F> {
    pub fn construct(config: MastermindConfig) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    pub fn configure(meta: &mut ConstraintSystem<F>) -> MastermindConfig {
        // Create columns
        let guess: [Column<Instance>; 4] = [
            meta.instance_column(),
            meta.instance_column(),
            meta.instance_column(),
            meta.instance_column(),
        ];
        let num_reds = meta.instance_column();
        let num_whites = meta.instance_column();
        let selector = meta.selector();
        let solution: [Column<Advice>; 4] = [
            meta.advice_column(),
            meta.advice_column(),
            meta.advice_column(),
            meta.advice_column(),
        ];
        let solution_hash = meta.instance_column();

        MastermindConfig {
            guess,
            num_reds,
            num_whites,
            selector,
            solution,
            solution_hash,
        }
    }

    pub fn assign_guess(&self, mut layouter: impl Layouter<F>) {
        layouter.assign_region(
            || "Guess region",
            |mut region| {
                self.config.selector.enable(&mut region, 0)?;

                for i in 0..4 {
                    region.
                }
            },
        );
    }
}

// #[derive(Clone, Debug)]
// struct MastermindCircuit<F: Base> {
//     pub board: Value<F>,
//     solution: Option<F>,
// };

// impl Circuit<pallas::Base> for MastermindCircuit {}
