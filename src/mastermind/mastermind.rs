use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::Field,
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Expression, Selector},
    poly::Rotation,
};

#[derive(Clone, Debug)]
pub struct MastermindConfig {
    // guess: [Column<Instance>; 4],
    selector: Selector,
    // num_reds: Column<Instance>,
    // num_whites: Column<Instance>,
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
        // let guess: [Column<Instance>; 4] = [
        //     meta.instance_column(),
        //     meta.instance_column(),
        //     meta.instance_column(),
        //     meta.instance_column(),
        // ];
        let selector = meta.selector();
        // let num_reds = meta.instance_column();
        // let num_whites = meta.instance_column();
        // let selector = meta.selector();
        let solution: [Column<Advice>; 4] = [
            meta.advice_column(),
            meta.advice_column(),
            meta.advice_column(),
            meta.advice_column(),
        ];
        // // let solution_hash = meta.instance_column();

        meta.create_gate("Check valid guess range", |meta| {
            let guess_range_selector = meta.query_selector(selector);
            let mut constraints = vec![];
            for i in 0..4 {
                let value = meta.query_advice(solution[i], Rotation::cur());
                let range_check = |range: usize, value: Expression<F>| {
                    assert!(range > 0);
                    (1..range).fold(value.clone(), |expr, index| {
                        expr * (Expression::Constant(F::from(index)) - value.clone())
                    })
                };
                constraints.push(guess_range_selector.clone() * range_check(6, value.clone()));
            }
            constraints
        });

        // meta.create_gate("Check exact match", |meta| {
        //     let guess_selector = meta.query_selector(selector);
        //     let mut constraints = vec![];
        //     for i in 0..4 {
        //         let guess_peg = meta.query_instance(guess[i], Rotation::cur());
        //         let sol_peg = meta.query_advice(solution[i], Rotation::cur());
        //         constraints.push(guess_selector.clone() * (guess_peg - sol_peg));
        //     }
        //     constraints
        // });

        MastermindConfig {
            // guess,
            selector,
            // num_reds,
            // num_whites,
            solution,
            // solution_hash,
        }
    }

    pub fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        solution: [Value<F>; 4],
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
                        || solution[i],
                    )?;
                }
                Ok(())
            },
        )
    }
}

#[derive(Clone, Default)]
pub struct MastermindCircuit<F: Field> {
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
        chip.assign(layouter, self.solution)?;
        // for i in 0..4 {
        //     layouter.constrain_instance(layouter.namespace(|| "guess peg"), config.guess[i], 0);
        // }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::MastermindCircuit;
    use halo2_proofs::{circuit::Value, dev::MockProver, pasta::Fp};

    #[test]
    fn test_mastermind_1() {
        // let guess = vec![vec![Fp::from(1), Fp::from(1), Fp::from(1), Fp::from(1)]];
        let solution = [
            Value::known(Fp::from(1)),
            Value::known(Fp::from(1)),
            Value::known(Fp::from(1)),
            Value::known(Fp::from(1)),
        ];

        let circuit = MastermindCircuit { solution };

        let prover = MockProver::run(4, &circuit, vec![]).unwrap();
        prover.assert_satisfied();
    }
}
