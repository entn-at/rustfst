use failure::{format_err, Fallible};
use serde_derive::{Deserialize, Serialize};

use rustfst::algorithms::{shortest_path, DeterminizeType};
use rustfst::fst_traits::MutableFst;
use rustfst::fst_traits::TextParser;
use rustfst::semirings::Semiring;
use rustfst::semirings::WeaklyDivisibleSemiring;
use rustfst::semirings::WeightQuantize;

use crate::TestData;

#[derive(Serialize, Deserialize, Debug)]
pub struct ShorestPathOperationResult {
    unique: bool,
    nshortest: usize,
    result: String,
}

pub struct ShortestPathTestData<F>
where
    F: TextParser,
    F::W: Semiring<Type = f32>,
{
    unique: bool,
    nshortest: usize,
    result: Fallible<F>,
}

impl ShorestPathOperationResult {
    pub fn parse<F>(&self) -> ShortestPathTestData<F>
    where
        F: TextParser,
        F::W: Semiring<Type = f32>,
    {
        ShortestPathTestData {
            unique: self.unique,
            nshortest: self.nshortest,
            result: match self.result.as_str() {
                "error" => Err(format_err!("lol")),
                _ => F::from_text_string(self.result.as_str()),
            },
        }
    }
}

pub fn test_shortest_path<F>(test_data: &TestData<F>) -> Fallible<()>
where
    F: TextParser + MutableFst,
    F::W: Semiring<Type = f32> + WeaklyDivisibleSemiring + WeightQuantize + 'static,
{
    for data in &test_data.shortest_path {
        let fst_res: Fallible<F> = shortest_path(&test_data.raw, data.nshortest, data.unique);
        match (&data.result, fst_res) {
            (Ok(fst_expected), Ok(ref fst_shortest)) => {
                assert_eq!(
                    fst_expected,
                    fst_shortest,
                    "{}",
                    error_message_fst!(
                        fst_expected,
                        fst_shortest,
                        format!(
                            "ShortestPath fail for nshortest = {:?} and unique = {:?}",
                            data.nshortest, data.unique
                        )
                    )
                );
            }
            (Ok(_fst_expected), Err(_)) => panic!(
                "ShortestPath fail for nshortest = {:?} and unique = {:?}. Got Err. Expected Ok",
                data.nshortest, data.unique
            ),
            (Err(_), Ok(_fst_shortest)) => panic!(
                "ShortestPath fail for nshortest = {:?} and unique = {:?}. Got Ok. Expected Err",
                data.nshortest, data.unique
            ),
            (Err(_), Err(_)) => {
                // Ok
            }
        };
    }

    Ok(())
}
