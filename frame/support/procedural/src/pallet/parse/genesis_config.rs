// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use syn::spanned::Spanned;
use super::helper;

/// Definition for pallet genesis config type.
///
/// Either:
/// * `type GenesisConfig`
/// * `struct GenesisConfig`
/// * `enum GenesisConfig`
pub struct GenesisConfigDef {
	/// The index of error item in pallet module.
	pub index: usize,
	/// Weither type has instance generic.
	pub has_instance: bool,
	/// Weither type has trait generic.
	pub has_trait: bool,
	/// A set of usage of instance, must be check for consistency with trait.
	pub instances: Vec<helper::InstanceUsage>,
	/// The ident of genesis_config, can be used for span.
	pub genesis_config: syn::Ident,
}

impl GenesisConfigDef {
	pub fn try_from(index: usize, item: &mut syn::Item) -> syn::Result<Self> {
		let item_span = item.span();
		let (vis, ident, generics) = match &item {
			syn::Item::Enum(item) => (&item.vis, &item.ident, &item.generics),
			syn::Item::Struct(item) => (&item.vis, &item.ident, &item.generics),
			syn::Item::Type(item) => (&item.vis, &item.ident, &item.generics),
			_ => {
				let msg = "Invalid pallet::genesis_config, expect enum or struct or type";
				return Err(syn::Error::new(item.span(), msg));
			},
		};

		let has_instance = generics.type_params().any(|t| t.ident == "I");
		let has_trait = generics.type_params().any(|t| t.ident == "T");

		let mut instances = vec![];
		// NOTE: GenesisConfig is not allowed to be only generic on I because it is not supported
		// by construct_runtime.
		if let Some(u) = helper::check_type_def_optional_gen(&generics, ident.span())? {
			instances.push(u);
		}

		if !matches!(vis, syn::Visibility::Public(_)) {
			let msg = "Invalid pallet::genesis_config, GenesisConfig must be public";
			return Err(syn::Error::new(item_span, msg));
		}

		if ident != "GenesisConfig" {
			let msg = "Invalid pallet::genesis_config, ident must `GenesisConfig`";
			return Err(syn::Error::new(ident.span(), msg));
		}

		Ok(GenesisConfigDef {
			index,
			genesis_config: ident.clone(),
			has_instance,
			has_trait,
			instances,
		})
	}
}