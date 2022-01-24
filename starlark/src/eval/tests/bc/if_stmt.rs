/*
 * Copyright 2018 The Starlark in Rust Authors.
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::eval::{bc::opcode::BcOpcode, tests::bc::test_instrs};

#[test]
fn test_if_x_and_true() {
    test_instrs(
        &[
            BcOpcode::LoadLocal,
            BcOpcode::IfNotBr,
            BcOpcode::CallFrozenNativePos,
            BcOpcode::Pop,
            BcOpcode::ReturnNone,
        ],
        "def test(x):\n  if x and True: print()",
    )
}

#[test]
fn test_if_x_and_false() {
    test_instrs(
        &[
            BcOpcode::LoadLocal,
            BcOpcode::IfNotBr,
            // TODO(nga): Missing optimization: we only need to evaluate `x` for effect.
            BcOpcode::Const,
            BcOpcode::IfNotBr,
            BcOpcode::CallFrozenNativePos,
            BcOpcode::Pop,
            BcOpcode::ReturnNone,
        ],
        "def test(x):\n  if x and False: print()",
    )
}

#[test]
fn test_if_x_or_true() {
    test_instrs(
        &[
            BcOpcode::LoadLocal,
            // TODO(nga): Missing optimization:
            //   we need to evaluate `x` for effect,
            //   and then unconditionally evaluate `print()`.
            BcOpcode::Dup,
            BcOpcode::IfBr,
            BcOpcode::Pop,
            BcOpcode::Const,
            BcOpcode::IfNotBr,
            BcOpcode::CallFrozenNativePos,
            BcOpcode::Pop,
            BcOpcode::ReturnNone,
        ],
        "def test(x):\n  if x or True: print()",
    )
}

#[test]
fn test_if_x_or_false() {
    test_instrs(
        &[
            BcOpcode::LoadLocal,
            BcOpcode::IfNotBr,
            BcOpcode::CallFrozenNativePos,
            BcOpcode::Pop,
            BcOpcode::ReturnNone,
        ],
        "def test(x):\n  if x or False: print()",
    )
}

#[test]
fn test_if_true_and_x() {
    test_instrs(
        &[
            BcOpcode::LoadLocal,
            BcOpcode::IfNotBr,
            BcOpcode::CallFrozenNativePos,
            BcOpcode::Pop,
            BcOpcode::ReturnNone,
        ],
        "def test(x):\n  if True and x: print()",
    )
}

#[test]
fn test_if_false_and_x() {
    test_instrs(
        &[BcOpcode::ReturnNone],
        "def test(x):\n  if False and x: print()",
    )
}

#[test]
fn test_if_true_or_x() {
    test_instrs(
        &[
            BcOpcode::CallFrozenNativePos,
            BcOpcode::Pop,
            BcOpcode::ReturnNone,
        ],
        "def test(x):\n  if True or x: print()",
    )
}

#[test]
fn test_if_false_or_x() {
    test_instrs(
        &[
            BcOpcode::LoadLocal,
            BcOpcode::IfNotBr,
            BcOpcode::CallFrozenNativePos,
            BcOpcode::Pop,
            BcOpcode::ReturnNone,
        ],
        "def test(x):\n  if False or x: print()",
    )
}

#[test]
fn test_and_stmt() {
    test_instrs(
        &[
            BcOpcode::LoadLocal,
            BcOpcode::IfNotBr,
            BcOpcode::CallFrozenNativePos,
            BcOpcode::Pop,
            BcOpcode::ReturnNone,
        ],
        "def test(x):\n  x and print()",
    )
}

#[test]
fn test_or_stmt() {
    test_instrs(
        &[
            BcOpcode::LoadLocal,
            BcOpcode::IfBr,
            BcOpcode::CallFrozenNativePos,
            BcOpcode::Pop,
            BcOpcode::ReturnNone,
        ],
        "def test(x):\n  x or print()",
    )
}