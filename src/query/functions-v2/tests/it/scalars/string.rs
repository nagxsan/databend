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

use std::io::Write;

use common_expression::types::DataType;
use common_expression::types::NumberDataType;
use common_expression::Column;
use common_expression::ColumnFrom;
use goldenfile::Mint;

use super::run_ast;

#[test]
fn test_string() {
    let mut mint = Mint::new("tests/it/scalars/testdata");
    let file = &mut mint.new_goldenfile("string.txt").unwrap();

    test_upper(file);
    test_lower(file);
    test_bit_length(file);
    test_octet_length(file);
    test_char_length(file);
    test_to_base64(file);
    test_from_base64(file);
    test_quote(file);
    test_reverse(file);
    test_ascii(file);
    test_ltrim(file);
    test_rtrim(file);
    test_trim_leading(file);
    test_trim_trailing(file);
    test_trim_both(file);
    test_trim(file);
    test_concat(file);
    test_bin(file);
    test_oct(file);
    test_hex(file);
    test_unhex(file);
    test_pad(file);
    test_replace(file);
    test_strcmp(file);
    test_locate(file);
    test_char(file);
    test_soundex(file);
    test_ord(file);
}

fn test_upper(file: &mut impl Write) {
    run_ast(file, "upper('Abc')", &[]);
    run_ast(file, "upper('Dobrý den')", &[]);
    run_ast(file, "upper('ß😀山')", &[]);
    run_ast(file, "upper(NULL)", &[]);
    run_ast(file, "ucase(a)", &[(
        "a",
        DataType::String,
        Column::from_data(&["Abc", "Dobrý den", "ß😀山"]),
    )]);
}

fn test_lower(file: &mut impl Write) {
    run_ast(file, "lower('Abc')", &[]);
    run_ast(file, "lower('DOBRÝ DEN')", &[]);
    run_ast(file, "lower('İ😀山')", &[]);
    run_ast(file, "lower(NULL)", &[]);
    run_ast(file, "lcase(a)", &[(
        "a",
        DataType::String,
        Column::from_data(&["Abc", "DOBRÝ DEN", "İ😀山"]),
    )]);
}

fn test_bit_length(file: &mut impl Write) {
    run_ast(file, "bit_length('latin')", &[]);
    run_ast(file, "bit_length(NULL)", &[]);
    run_ast(file, "bit_length(a)", &[(
        "a",
        DataType::String,
        Column::from_data(&["latin", "кириллица", "кириллица and latin"]),
    )]);
}

fn test_octet_length(file: &mut impl Write) {
    run_ast(file, "octet_length('latin')", &[]);
    run_ast(file, "octet_length(NULL)", &[]);
    run_ast(file, "length(a)", &[(
        "a",
        DataType::String,
        Column::from_data(&["latin", "кириллица", "кириллица and latin"]),
    )]);
}

fn test_char_length(file: &mut impl Write) {
    run_ast(file, "char_length('latin')", &[]);
    run_ast(file, "char_length(NULL)", &[]);
    run_ast(file, "character_length(a)", &[(
        "a",
        DataType::String,
        Column::from_data(&["latin", "кириллица", "кириллица and latin"]),
    )]);
}

fn test_to_base64(file: &mut impl Write) {
    run_ast(file, "to_base64('Abc')", &[]);
    run_ast(file, "to_base64('123')", &[]);
    run_ast(file, "to_base64(Null)", &[]);
    run_ast(file, "to_base64(a)", &[(
        "a",
        DataType::String,
        Column::from_data(&["Abc", "123"]),
    )]);
}

fn test_from_base64(file: &mut impl Write) {
    run_ast(file, "from_base64('QWJj')", &[]);
    run_ast(file, "from_base64('MTIz')", &[]);
    run_ast(file, "from_base64(Null)", &[]);
    run_ast(file, "from_base64(a)", &[(
        "a",
        DataType::String,
        Column::from_data(&["QWJj", "MTIz"]),
    )])
}

fn test_quote(file: &mut impl Write) {
    run_ast(file, r#"quote('a\0b')"#, &[]);
    run_ast(file, r#"quote('a\'b')"#, &[]);
    run_ast(file, r#"quote('a\"b')"#, &[]);
    run_ast(file, r#"quote('a\bb')"#, &[]);
    run_ast(file, r#"quote('a\nb')"#, &[]);
    run_ast(file, r#"quote('a\rb')"#, &[]);
    run_ast(file, r#"quote('a\tb')"#, &[]);
    run_ast(file, r#"quote('a\\b')"#, &[]);
    run_ast(file, "quote('你好')", &[]);
    run_ast(file, "quote('ß😀山')", &[]);
    run_ast(file, "quote('Dobrý den')", &[]);
    run_ast(file, "quote(Null)", &[]);
    run_ast(file, "quote(a)", &[(
        "a",
        DataType::String,
        Column::from_data(&[r#"a\0b"#, r#"a\'b"#, r#"a\nb"#]),
    )])
}

fn test_reverse(file: &mut impl Write) {
    run_ast(file, "reverse('abc')", &[]);
    run_ast(file, "reverse('a')", &[]);
    run_ast(file, "reverse('')", &[]);
    run_ast(file, "reverse('你好')", &[]);
    run_ast(file, "reverse('ß😀山')", &[]);
    run_ast(file, "reverse('Dobrý den')", &[]);
    run_ast(file, "reverse(Null)", &[]);
    run_ast(file, "reverse(a)", &[(
        "a",
        DataType::String,
        Column::from_data(&["abc", "a", ""]),
    )])
}

fn test_ascii(file: &mut impl Write) {
    run_ast(file, "ascii('1')", &[]);
    run_ast(file, "ascii('123')", &[]);
    run_ast(file, "ascii('-1')", &[]);
    run_ast(file, "ascii('')", &[]);
    run_ast(file, "ascii('你好')", &[]);
    run_ast(file, "ascii('😀123')", &[]);
    run_ast(file, "ascii(Null)", &[]);
    run_ast(file, "ascii(a)", &[(
        "a",
        DataType::String,
        Column::from_data(&["1", "123", "-1", "你好"]),
    )]);
    run_ast(file, "ascii(b)", &[(
        "b",
        DataType::String,
        Column::from_data(&[""]),
    )]);
}

fn test_ltrim(file: &mut impl Write) {
    run_ast(file, "ltrim('   abc   ')", &[]);
    run_ast(file, "ltrim('  ')", &[]);
    run_ast(file, "ltrim(NULL)", &[]);
    run_ast(file, "ltrim(a)", &[(
        "a",
        DataType::String,
        Column::from_data(&["abc", "   abc", "   abc   ", "abc   "]),
    )]);
}

fn test_rtrim(file: &mut impl Write) {
    run_ast(file, "rtrim('   abc   ')", &[]);
    run_ast(file, "rtrim('  ')", &[]);
    run_ast(file, "rtrim(NULL)", &[]);
    run_ast(file, "rtrim(a)", &[(
        "a",
        DataType::String,
        Column::from_data(&["abc", "   abc", "   abc   ", "abc   "]),
    )]);
}

fn test_trim_leading(file: &mut impl Write) {
    run_ast(file, "trim_leading('aaabbaaa', 'a')", &[]);
    run_ast(file, "trim_leading('aaabbaaa', 'aa')", &[]);
    run_ast(file, "trim_leading('aaaaaaaa', 'a')", &[]);
    run_ast(file, "trim_leading('aaabbaaa', 'b')", &[]);
    run_ast(file, "trim_leading(NULL, 'a')", &[]);
    run_ast(file, "trim_leading('aaaaaaaa', NULL)", &[]);

    let table = [
        (
            "a",
            DataType::String,
            Column::from_data(&["aabbaa", "bbccbb", "ccddcc"]),
        ),
        ("b", DataType::String, Column::from_data(&["a", "b", "c"])),
    ];

    run_ast(file, "trim_leading(a, 'a')", &table);
    run_ast(file, "trim_leading(a, b)", &table);
    run_ast(file, "trim_leading('aba', b)", &table);
}

fn test_trim_trailing(file: &mut impl Write) {
    run_ast(file, "trim_trailing('aaabbaaa', 'a')", &[]);
    run_ast(file, "trim_trailing('aaabbaaa', 'aa')", &[]);
    run_ast(file, "trim_trailing('aaaaaaaa', 'a')", &[]);
    run_ast(file, "trim_trailing('aaabbaaa', 'b')", &[]);
    run_ast(file, "trim_trailing(NULL, 'a')", &[]);
    run_ast(file, "trim_trailing('aaaaaaaa', NULL)", &[]);

    let table = [
        (
            "a",
            DataType::String,
            Column::from_data(&["aabbaa", "bbccbb", "ccddcc"]),
        ),
        ("b", DataType::String, Column::from_data(&["a", "b", "c"])),
    ];

    run_ast(file, "trim_trailing(a, 'b')", &table);
    run_ast(file, "trim_trailing(a, b)", &table);
    run_ast(file, "trim_trailing('aba', b)", &table);
}

fn test_trim_both(file: &mut impl Write) {
    run_ast(file, "trim_both('aaabbaaa', 'a')", &[]);
    run_ast(file, "trim_both('aaabbaaa', 'aa')", &[]);
    run_ast(file, "trim_both('aaaaaaaa', 'a')", &[]);
    run_ast(file, "trim_both('aaabbaaa', 'b')", &[]);
    run_ast(file, "trim_both(NULL, 'a')", &[]);
    run_ast(file, "trim_both('aaaaaaaa', NULL)", &[]);

    let table = [
        (
            "a",
            DataType::String,
            Column::from_data(&["aabbaa", "bbccbb", "ccddcc"]),
        ),
        ("b", DataType::String, Column::from_data(&["a", "b", "c"])),
    ];

    run_ast(file, "trim_both(a, 'a')", &table);
    run_ast(file, "trim_both(a, b)", &table);
    run_ast(file, "trim_both('aba', b)", &table);
}

fn test_trim_with_from(file: &mut impl Write, trim_where: &str) {
    assert!(matches!(trim_where, "both" | "leading" | "trailing"));

    run_ast(
        file,
        format!("trim({} 'a' from 'aaabbaaa')", trim_where).as_str(),
        &[],
    );
    run_ast(
        file,
        format!("trim({} 'aa' from 'aaabbaaa')", trim_where).as_str(),
        &[],
    );
    run_ast(
        file,
        format!("trim({} 'a' from 'aaaaaaaa')", trim_where).as_str(),
        &[],
    );
    run_ast(
        file,
        format!("trim({} 'b' from 'aaabbaaa')", trim_where).as_str(),
        &[],
    );
    run_ast(
        file,
        format!("trim({} 'a' from NULL)", trim_where).as_str(),
        &[],
    );
    run_ast(
        file,
        format!("trim({} NULL from 'aaaaaaaa')", trim_where).as_str(),
        &[],
    );

    let table = [
        (
            "a",
            DataType::String,
            Column::from_data(&["aabbaa", "bbccbb", "ccddcc"]),
        ),
        ("b", DataType::String, Column::from_data(&["a", "b", "c"])),
    ];

    run_ast(
        file,
        format!("trim({} 'a' from a)", trim_where).as_str(),
        &table,
    );
    run_ast(
        file,
        format!("trim({} b from a)", trim_where).as_str(),
        &table,
    );
    run_ast(
        file,
        format!("trim({} a from a)", trim_where).as_str(),
        &table,
    );
    run_ast(
        file,
        format!("trim({} b from 'aba')", trim_where).as_str(),
        &table,
    );
}

fn test_trim(file: &mut impl Write) {
    // TRIM(<expr>)
    run_ast(file, "trim('   abc   ')", &[]);
    run_ast(file, "trim('  ')", &[]);
    run_ast(file, "trim(NULL)", &[]);
    run_ast(file, "trim(a)", &[(
        "a",
        DataType::String,
        Column::from_data(&["abc", "   abc", "   abc   ", "abc   "]),
    )]);

    // TRIM([[BOTH | LEADING | TRAILING] <expr> FROM] <expr>)
    test_trim_with_from(file, "both");
    test_trim_with_from(file, "leading");
    test_trim_with_from(file, "trailing");
}

fn test_concat(file: &mut impl Write) {
    run_ast(file, "concat('5', '3', '4')", &[]);
    run_ast(file, "concat(NULL, '3', '4')", &[]);
    run_ast(file, "concat(a, '3', '4', '5')", &[(
        "a",
        DataType::String,
        Column::from_data(&["abc", "   abc", "   abc   ", "abc   "]),
    )]);

    run_ast(file, "concat(a, '3')", &[(
        "a",
        DataType::Nullable(Box::new(DataType::String)),
        Column::from_data_with_validity(&["a", "b", "c", "d"], vec![true, true, false, true]),
    )]);

    run_ast(file, "concat_ws('-', '3', null, '4', null, '5')", &[]);
    run_ast(file, "concat_ws(NULL, '3', '4')", &[]);
    run_ast(file, "concat_ws(a, '3', '4', '5')", &[(
        "a",
        DataType::String,
        Column::from_data(&[",", "-", ",", "-"]),
    )]);

    run_ast(file, "concat_ws(a, '3')", &[(
        "a",
        DataType::Nullable(Box::new(DataType::String)),
        Column::from_data_with_validity(&["a", "b", "c", "d"], vec![true, true, false, true]),
    )]);
}

fn test_bin(file: &mut impl Write) {
    let columns = &[
        (
            "a",
            DataType::Number(NumberDataType::Int8),
            Column::from_data(vec![-1i8, 2, 3]),
        ),
        (
            "a2",
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::UInt8))),
            Column::from_data_with_validity(vec![1u8, 2, 3], vec![true, true, false]),
        ),
        (
            "b",
            DataType::Number(NumberDataType::Int16),
            Column::from_data(vec![2i16, 4, 6]),
        ),
        (
            "c",
            DataType::Number(NumberDataType::UInt32),
            Column::from_data(vec![10u32, 20, 30]),
        ),
        (
            "d",
            DataType::Number(NumberDataType::Float64),
            Column::from_data(vec![10f64, -20f64, 30f64]),
        ),
        (
            "e",
            DataType::String,
            Column::from_data(vec!["abc", "def", "databend"]),
        ),
    ];
    run_ast(file, "bin(a)", columns);
    run_ast(file, "bin(a2)", columns);
    run_ast(file, "bin(b)", columns);
    run_ast(file, "bin(c)", columns);
    run_ast(file, "bin(d)", columns);
    run_ast(file, "bin(e)", columns);
}

fn test_oct(file: &mut impl Write) {
    let columns = &[
        (
            "a",
            DataType::Number(NumberDataType::Int8),
            Column::from_data(vec![-1i8, 2, 3]),
        ),
        (
            "a2",
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::UInt8))),
            Column::from_data_with_validity(vec![1u8, 2, 3], vec![true, true, false]),
        ),
        (
            "b",
            DataType::Number(NumberDataType::Int16),
            Column::from_data(vec![2i16, 4, 6]),
        ),
        (
            "c",
            DataType::Number(NumberDataType::UInt32),
            Column::from_data(vec![10u32, 20, 30]),
        ),
        (
            "d",
            DataType::Number(NumberDataType::Float64),
            Column::from_data(vec![10f64, -20f64, 30f64]),
        ),
        (
            "e",
            DataType::String,
            Column::from_data(vec!["abc", "def", "databend"]),
        ),
    ];
    run_ast(file, "oct(a)", columns);
    run_ast(file, "oct(a2)", columns);
    run_ast(file, "oct(b)", columns);
    run_ast(file, "oct(c)", columns);
    run_ast(file, "oct(d)", columns);
    run_ast(file, "oct(e)", columns);
}

fn test_hex(file: &mut impl Write) {
    let columns = &[
        (
            "a",
            DataType::Number(NumberDataType::Int8),
            Column::from_data(vec![-1i8, 2, 3]),
        ),
        (
            "a2",
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::UInt8))),
            Column::from_data_with_validity(vec![1u8, 2, 3], vec![true, true, false]),
        ),
        (
            "b",
            DataType::Number(NumberDataType::Int16),
            Column::from_data(vec![2i16, 4, 6]),
        ),
        (
            "c",
            DataType::Number(NumberDataType::UInt32),
            Column::from_data(vec![10u32, 20, 30]),
        ),
        (
            "d",
            DataType::Number(NumberDataType::Float64),
            Column::from_data(vec![10f64, -20f64, 30f64]),
        ),
        (
            "e",
            DataType::String,
            Column::from_data(vec!["abc", "def", "databend"]),
        ),
    ];
    run_ast(file, "hex(a)", columns);
    run_ast(file, "hex(a2)", columns);
    run_ast(file, "hex(b)", columns);
    run_ast(file, "hex(c)", columns);
    run_ast(file, "hex(d)", columns);
    run_ast(file, "hex(e)", columns);
}

fn test_unhex(file: &mut impl Write) {
    run_ast(file, "unhex('6461746162656e64')", &[]);

    let columns = &[(
        "s",
        DataType::String,
        Column::from_data(vec!["abc", "def", "databend"]),
    )];
    run_ast(file, "unhex(hex(s))", columns);

    let columns = &[(
        "s",
        DataType::String,
        Column::from_data(vec!["616263", "646566", "6461746162656e64"]),
    )];
    run_ast(file, "unhex(s)", columns);
}

fn test_pad(file: &mut impl Write) {
    run_ast(file, "lpad('hi', 2, '?')", &[]);
    run_ast(file, "lpad('hi', 4, '?')", &[]);
    run_ast(file, "lpad('hi', 0, '?')", &[]);
    run_ast(file, "lpad('hi', 1, '?')", &[]);
    run_ast(file, "lpad('hi', -1, '?')", &[]);
    let table = [
        (
            "a",
            DataType::String,
            Column::from_data(&["hi", "test", "cc"]),
        ),
        (
            "b",
            DataType::Number(NumberDataType::UInt8),
            Column::from_data(vec![0, 3, 5]),
        ),
        ("c", DataType::String, Column::from_data(&["?", "x", "bb"])),
    ];
    run_ast(file, "lpad(a, b, c)", &table);
    run_ast(file, "rpad('hi', 2, '?')", &[]);
    run_ast(file, "rpad('hi', 4, '?')", &[]);
    run_ast(file, "rpad('hi', 0, '?')", &[]);
    run_ast(file, "rpad('hi', 1, '?')", &[]);
    run_ast(file, "rpad('hi', -1, '?')", &[]);
    run_ast(file, "rpad(a, b, c)", &table);
}

fn test_replace(file: &mut impl Write) {
    run_ast(file, "replace('hi', '', '?')", &[]);
    run_ast(file, "replace('hi', '', 'hi')", &[]);
    run_ast(file, "replace('hi', 'i', '?')", &[]);
    run_ast(file, "replace('hi', 'x', '?')", &[]);

    let table = [
        (
            "a",
            DataType::String,
            Column::from_data(&["hi", "test", "cc", "q"]),
        ),
        (
            "b",
            DataType::String,
            Column::from_data(&["i", "te", "cc", ""]),
        ),
        (
            "c",
            DataType::String,
            Column::from_data(&["?", "x", "bb", "q"]),
        ),
    ];
    run_ast(file, "replace(a, b, c)", &table);
}

fn test_strcmp(file: &mut impl Write) {
    run_ast(file, "strcmp('text', 'text2')", &[]);
    run_ast(file, "strcmp('text2', 'text')", &[]);
    run_ast(file, "strcmp('hii', 'hii')", &[]);

    let table = [
        (
            "a",
            DataType::String,
            Column::from_data(&["hi", "test", "cc"]),
        ),
        (
            "b",
            DataType::String,
            Column::from_data(&["i", "test", "ccb"]),
        ),
    ];
    run_ast(file, "strcmp(a, b)", &table);
}

fn test_locate(file: &mut impl Write) {
    run_ast(file, "locate('bar', 'foobarbar')", &[]);
    run_ast(file, "instr('foobarbar', 'bar')", &[]);
    run_ast(file, "position('bar' IN 'foobarbar')", &[]);
    run_ast(file, "position('foobarbar' IN 'bar')", &[]);
    run_ast(file, "locate('bar', 'foobarbar', 5)", &[]);

    let table = [
        (
            "a",
            DataType::String,
            Column::from_data(&["bar", "cc", "cc", "q"]),
        ),
        (
            "b",
            DataType::String,
            Column::from_data(&["foobarbar", "bdccacc", "xx", "56"]),
        ),
        (
            "c",
            DataType::Number(NumberDataType::UInt8),
            Column::from_data(vec![1, 2, 0, 1]),
        ),
    ];
    run_ast(file, "locate(a, b, c)", &table);
}

fn test_char(file: &mut impl Write) {
    run_ast(file, "char(65,66,67)", &[]);
    run_ast(file, "char(65, null)", &[]);

    let table = [
        (
            "a",
            DataType::Number(NumberDataType::UInt8),
            Column::from_data(vec![66u8, 67]),
        ),
        (
            "b",
            DataType::Number(NumberDataType::UInt8),
            Column::from_data(vec![98u8, 99]),
        ),
        (
            "c",
            DataType::Number(NumberDataType::UInt8),
            Column::from_data(vec![68u8, 69]),
        ),
        (
            "c2",
            DataType::Number(NumberDataType::UInt16),
            Column::from_data(vec![68u16, 69]),
        ),
        (
            "a2",
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::UInt8))),
            Column::from_data_with_validity(vec![66u8, 67], vec![true, false]),
        ),
    ];
    run_ast(file, "char(a, b, c)", &table);
    run_ast(file, "char(a2, b, c)", &table);
    run_ast(file, "char(c2)", &table);
}

fn test_soundex(file: &mut impl Write) {
    run_ast(file, "soundex('你好中国北京')", &[]);
    run_ast(file, "soundex('')", &[]);
    run_ast(file, "soundex('hello all folks')", &[]);
    run_ast(file, "soundex('#3556 in bugdb')", &[]);

    let table = [(
        "a",
        DataType::String,
        Column::from_data(&["#🐑🐑he🐑llo🐑", "🐑he🐑llo🐑", "teacher", "TEACHER"]),
    )];
    run_ast(file, "soundex(a)", &table);
}

fn test_ord(file: &mut impl Write) {
    run_ast(file, "ord(NULL)", &[]);
    run_ast(file, "ord('и')", &[]);
    run_ast(file, "ord('早ab')", &[]);
    run_ast(file, "ord('💖')", &[]);
}
