// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::f64::consts::PI;

use common_datavalues::prelude::*;
use common_exception::Result;
use common_functions::scalars::*;

use crate::scalars::scalar_function2_test::test_scalar_functions;
use crate::scalars::scalar_function2_test::ScalarFunctionTest;

#[test]
fn test_pi_function() -> Result<()> {
    let tests = vec![ScalarFunctionTest {
        name: "pi-function-passed",
        // TODO: this should be `vec![]`, consider adding the `input_row` field to `ScalarFunctionTest`.
        columns: vec![Series::from_data(vec![PI])],
        expect: Series::from_data(vec![PI]),
        error: "",
    }];

    test_scalar_functions(PiFunction::try_create("pi()")?, &tests, false)
}
