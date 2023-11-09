(*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree.
 *
 *)
val find :
  ctx:Provider_context.t ->
  entry:Provider_context.entry ->
  Pos.t ->
  Code_action_types.Quickfix.t list
