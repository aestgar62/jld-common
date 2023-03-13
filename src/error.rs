// Copyright © 2023 jld-common. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0
//!
//! # Errors
//! 

use thiserror::Error;

use iref::IriBuf;
use locspan::{Meta, Span};

/// Error enumeration
#[derive(Debug, Error)]
pub enum Error {

	/// Invalid URI format
    #[error("Invalid URI format: {0}")]
    InvalidURIFormat(String),

	/// Invalid context
    #[error("Invalid context.")]
    InvalidContext,

	/// Missing context.
    #[error("Missing context.")]
    MissingContext,

    /// Invalid JSON.
    #[error("Invalid JSON: {0}")]
    InvalidJson(#[from] json_syntax::parse::MetaError<Span>),

    /// Invalid JSON-LD context.
    #[error("Invalid JSON-LD context: {0}")]
    InvalidJldContext(#[from] Meta<json_ld::syntax::context::InvalidContext, Span>),

    /// Graph incossistency: Object mismatch.
    #[error("Graph Object mismatch => {0}, {1}, {2}.")]
    GraphObjectMismatch(IriBuf, String, String),

    /// Graph inconsistency: Expected object.
    #[error("Graph inconsistency: Expected object => {0}, {1}.")]
    GraphExpectedObject(IriBuf, String),

    /// Graph inconsistency: Unexpected object.
    #[error("Graph inconsistency: Unexpected object => {0}, {1}.")]
    GraphUnexpectedObject(IriBuf, String),

    /// Graph inconsistency: List item mismatch.
    #[error("Graph inconsistency: List item mismatch => {0}, {1}.")]
    GraphListItemMismatch(String, String),

    /// Graph inconsistency: Invalid list.
    #[error("Graph inconsistency: Invalid list.")]
    GraphInvalidList,

    /// Graph inconsistency: Unexpected end of list.
    #[error("Graph inconsistency: Unexpected end of list.")]
    GraphUnexpectedEndOfList,

    /// Graph inconsistency: Expected end of list.
    #[error("Graph inconsistency: Expected end of list.")]
    GraphExpectedEndOfList,

}