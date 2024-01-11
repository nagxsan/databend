// Copyright 2022 Datafuse Labs.
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

use databend_common_expression::build_select_expr;
use databend_common_expression::filter::FilterExecutor;
use databend_common_expression::types::BooleanType;
use databend_common_expression::types::DataType;
use databend_common_expression::types::DecimalDataType;
use databend_common_expression::types::DecimalSize;
use databend_common_expression::types::NumberDataType;
use databend_common_expression::Column;
use databend_common_expression::DataBlock;
use databend_common_expression::Evaluator;
use databend_common_expression::FunctionContext;
use databend_common_functions::BUILTIN_FUNCTIONS;
use itertools::Itertools;
use rand::Rng;

use super::random_filter_expr;

// Test the result of `FilterExecutor` is the same as `Evaluator`.
#[test]
pub fn test_filter_executor() -> databend_common_exception::Result<()> {
    let mut rng = rand::thread_rng();
    // For EmptyMap, Map, Bitmap comparison, it is not supported by Evaluator.
    let data_types = get_filter_data_types();
    for _ in 0..100 {
        for data_type in data_types.iter() {
            // Random number of rows, number of columns and random max depth of the filter expr.
            let num_rows = rng.gen_range(0..10000);
            let num_columns = rng.gen_range(1..10);
            let max_depth = rng.gen_range(1..5);

            // 1. Generate a random `DataBlock`.
            let columns = (0..num_columns)
                .map(|_| Column::random(data_type, num_rows))
                .collect_vec();
            let block = DataBlock::new_from_columns(columns);
            block.check_valid()?;

            // 2. Generate a random filter expr with the given depth.
            let expr = random_filter_expr(data_type, max_depth, num_columns)?;

            // 3.1 Execute the filter expr by `Evaluator`.
            let func_ctx = &FunctionContext::default();
            let evaluator = Evaluator::new(&block, func_ctx, &BUILTIN_FUNCTIONS);
            let filter = evaluator.run(&expr)?.try_downcast::<BooleanType>().unwrap();
            let block_1 = block.clone().filter_boolean_value(&filter)?;

            // 3.2 Execute the filter expr by `FilterExecutor`.
            let (select_expr, has_or) = build_select_expr(&expr).into();
            let mut filter_executor = FilterExecutor::new(
                select_expr,
                func_ctx.clone(),
                has_or,
                num_rows,
                None,
                &BUILTIN_FUNCTIONS,
                true,
            );
            let block_2 = filter_executor.filter(block.clone())?;

            // 4. Check if the result block generated by `Evaluator` is the same as the result block generated by `FilterExecutor`.
            assert_eq!(block_1.num_columns(), block_2.num_columns());
            assert_eq!(block_1.num_rows(), block_2.num_rows());
            let columns_1 = block_1.columns();
            let columns_2 = block_2.columns();
            for idx in 0..columns_1.len() {
                assert_eq!(columns_1[idx].data_type, columns_2[idx].data_type);
                assert_eq!(columns_1[idx].value, columns_2[idx].value);
            }
        }
    }
    Ok(())
}

// For EmptyMap, Map, Bitmap comparison, it is not supported by `Evaluator`.
fn get_filter_data_types() -> Vec<DataType> {
    vec![
        DataType::EmptyArray,
        DataType::Boolean,
        DataType::String,
        DataType::Variant,
        DataType::Timestamp,
        DataType::Date,
        DataType::Number(NumberDataType::UInt8),
        DataType::Number(NumberDataType::UInt16),
        DataType::Number(NumberDataType::UInt32),
        DataType::Number(NumberDataType::UInt64),
        DataType::Number(NumberDataType::Int8),
        DataType::Number(NumberDataType::Int16),
        DataType::Number(NumberDataType::Int32),
        DataType::Number(NumberDataType::Int64),
        DataType::Number(NumberDataType::Float32),
        DataType::Number(NumberDataType::Float64),
        DataType::Decimal(DecimalDataType::Decimal128(DecimalSize {
            precision: 10,
            scale: 2,
        })),
        DataType::Decimal(DecimalDataType::Decimal128(DecimalSize {
            precision: 35,
            scale: 3,
        })),
        DataType::Array(Box::new(DataType::Number(NumberDataType::UInt32))),
        DataType::Null,
        DataType::Nullable(Box::new(DataType::Number(NumberDataType::UInt32))),
        DataType::Nullable(Box::new(DataType::String)),
        DataType::Tuple(vec![
            DataType::Number(NumberDataType::UInt32),
            DataType::Boolean,
            DataType::Array(Box::new(DataType::Number(NumberDataType::UInt32))),
            DataType::Tuple(vec![
                DataType::Nullable(Box::new(DataType::String)),
                DataType::Number(NumberDataType::UInt32),
                DataType::Decimal(DecimalDataType::Decimal128(DecimalSize {
                    precision: 10,
                    scale: 2,
                })),
            ]),
        ]),
    ]
}
