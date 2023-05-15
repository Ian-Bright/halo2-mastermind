use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::Field,
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Instance, Selector},
    poly::Rotation,
};

#[derive(Clone, Debug)]
pub struct MastermindConfig {
    guess: [Column<Instance>; 4],
    // num_reds: Column<Instance>,
    // num_whites: Column<Instance>,
    selector: Selector,
    solution: [Column<Advice>; 4],
    // solution_hash: Column<Instance>,
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

        meta.create_gate("Check exact match", |meta| {
            let guess_selector = meta.query_selector(selector);
            let mut constraints = vec![];
            for i in 0..4 {
                let guess_peg = meta.query_instance(guess[i], Rotation::cur());
                let sol_peg = meta.query_advice(solution[i], Rotation::next());
                constraints.push(guess_selector.clone() * (guess_peg - sol_peg));
            }
            constraints
        });

        MastermindConfig {
            guess,
            // num_reds,
            // num_whites,
            selector,
            solution,
            // solution_hash,
        }
    }

    pub fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        solution: [Column<Advice>; 4],
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "Guess region",
            |mut region| {
                self.config.selector.enable(&mut region, 0)?;

                for i in 0..4 {
                    region.assign_advice(
                        || "Solution peg",
                        self.config.solution[i],
                        0,
                        || Value::known(solution[i]),
                    )?;
                }
                Ok(())
            },
        )
    }
}

#[derive(Clone, Default)]
pub struct MastermindCircuit<F: Field> {
    pub guess: [Value<F>; 4],
    solution: [Value<F>; 4],
}

impl<F: Field> Circuit<F> for MastermindCircuit<F> {
    type Config = MastermindConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        MastermindChip::configure(meta)
    }

    fn synthesize(&self, config: Self::Config, layouter: impl Layouter<F>) -> Result<(), Error> {
        let chip = MastermindChip::construct(config);

        chip.assign(layouter, config.solution);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use super::MastermindCircuit;
    use halo2_proofs::{dev::MockProver, pasta::Fp};

    #[test]
    fn test_mastermind_1() {
        let guess = [Fp::from(1), Fp::from(1), Fp::from(1), Fp::from(1)];
        let solution = [Fp::from(1), Fp::from(1), Fp::from(1), Fp::from(1)];

        // let circuit = MastermindCircuit =
    }
}
