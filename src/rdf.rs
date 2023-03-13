// Copyright © 2023 jld-common. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0
//!
//! # RDF utilities.
//! 

use crate::Error;

use iref::{Iri, IriBuf};
use static_iref::iri;
use rdf_types::{Term, Id, Object};
use std::borrow::Borrow;

/// Constant IRI for rdf `type`. 
pub const RDF_TYPE: Iri<'static> = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
/// Constant IRI for rdf `nil`. 
pub const RDF_NIL: Iri<'static> = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#nil");
/// Constant IRI for rdf `first`. 
pub const RDF_FIRST: Iri<'static> = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#first");
/// Constant IRI for rdf `rdf`. 
pub const RDF_REST: Iri<'static> = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#rest");
/// Constant IRI for rdf `json`. 
pub const RDF_JSON: Iri<'static> = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#JSON");

/// RDF DataSet produced from a JSON-LD document.
pub type DataSet =
    grdf::HashDataset<rdf_types::Subject, IriBuf, rdf_types::Object, rdf_types::GraphLabel>;

/// RDF Graph.
pub type Graph = grdf::HashGraph<rdf_types::Subject, IriBuf, rdf_types::Object>;

/// It extends the RDF graph with utilities to validate it.
trait GraphValidate:
    grdf::Graph<Subject = rdf_types::Subject, Predicate = IriBuf, Object = rdf_types::Object>
    + for<'a> grdf::GraphTake<rdf_types::Subject, Iri<'a>, rdf_types::Object>
{
    /// Take any statement of the form `s p o` for the given `s` and `p`
    /// and call `object_predicate(Some(o))`.
    /// If no statement has the form `s p o`, call `object_predicate(None)`.
    fn take_object_and_assert<E>(
        &mut self,
        s: &Self::Subject,
        p: Iri,
        object_predicate: impl FnOnce(&mut Self, Option<Self::Object>) -> Result<(), E>,
    ) -> Result<(), E> {
        match self.take_match(rdf_types::Triple(Some(s), Some(&p), None)) {
            Some(rdf_types::Triple(_, _, o)) => object_predicate(self, Some(o)),
            None => object_predicate(self, None),
        }
    }

    /// When `expected_o` is `Some(iri)`.
    /// take any statement of the form `s p o` for the given `s` and `p`
    /// and checks that `o` is equal to `iri` according to `eq`.
    /// When `expected_o` is `None`,
    /// checks that no statement has the form `s p o`.
    fn take_object_and_assert_eq<V: ToString>(
        &mut self,
        s: &Self::Subject,
        p: Iri,
        expected_o: Option<V>,
        eq: impl FnOnce(&Self::Object, &V) -> bool,
    ) -> Result<(), Box<Error>> {
        self.take_object_and_assert(s, p, |_, o| match (o, expected_o) {
            (Some(o), Some(expected)) => {
                if eq(&o, &expected) {
                    Ok(())
                } else {
                    Err(Box::new(Error::GraphObjectMismatch(
                        p.to_owned(),
                        expected.to_string(),
                        o.to_string(),
                    )))
                }
            }
            (None, None) => Ok(()),
            (None, Some(expected_iri)) => Err(Box::new(Error::GraphExpectedObject(
                p.to_owned(),
                expected_iri.to_string(),
            ))),
            (Some(o), None) => Err(Box::new(Error::GraphUnexpectedObject(
                p.to_owned(),
                o.to_string(),
            ))),
        })
    }

    /// When `expected_o` is `Some(json)`.
    /// take any statement of the form `s p o` for the given `s` and `p`
    /// and checks that `o` is equal to the JSON array value `json`.
    /// When `expected_o` is `None`,
    /// checks that no statement has the form `s p o`.
    fn take_object_and_assert_eq_list<I: Iterator>(
        &mut self,
        s: &Self::Subject,
        p: Iri,
        expected_o: Option<I>,
        eq: impl Fn(&Self::Object, &I::Item) -> bool,
    ) -> Result<(), Box<Error>>
    where
        I::Item: ToString,
    {
        fn format_list<I: Iterator>(list: I) -> String
        where
            I::Item: ToString,
        {
            let mut expected_string = "[".to_owned();

            for (i, item) in list.enumerate() {
                if i > 0 {
                    expected_string.push(',');
                }

                expected_string.push_str(&item.to_string());
            }

            expected_string.push(']');
            expected_string
        }

        self.take_object_and_assert(s, p, |this, o| match (o, expected_o) {
            (Some(o), Some(expected)) => {
                let mut head = match o {
                    Term::Id(Id::Iri(i)) if i == RDF_NIL => None,
                    Term::Id(Id::Iri(i)) => Some(rdf_types::Subject::Iri(i)),
                    Term::Id(Id::Blank(b)) => Some(rdf_types::Subject::Blank(b)),
                    rdf_types::Term::Literal(l) => {
                        return Err(Box::new(Error::GraphObjectMismatch(
                            p.to_owned(),
                            l.to_string(),
                            format_list(expected),
                        )))
                    }
                };

                for expected_item in expected {
                    match head.take() {
                        Some(id) => {
                            match this.take_match(rdf_types::Triple(
                                Some(&id),
                                Some(&RDF_FIRST),
                                None,
                            )) {
                                Some(rdf_types::Triple(_, _, first)) => {
                                    if !eq(&first, &expected_item) {
                                        return Err(Box::new(Error::GraphListItemMismatch(
                                            first.to_string(),
                                            expected_item.to_string(),
                                        )));
                                    }

                                    match this.take_match(rdf_types::Triple(
                                        Some(&id),
                                        Some(&RDF_REST),
                                        None,
                                    )) {
                                        Some(rdf_types::Triple(_, _, rest)) => {
                                            head = match rest {
                                                Term::Id(Id::Iri(i)) if i == RDF_NIL => None,
                                                Term::Id(Id::Iri(i)) => {
                                                    Some(rdf_types::Subject::Iri(i))
                                                }
                                                Term::Id(Id::Blank(b)) => {
                                                    Some(rdf_types::Subject::Blank(b))
                                                }
                                                Term::Literal(_) => {
                                                    return Err(Box::new(Error::GraphInvalidList))
                                                }
                                            };
                                        }
                                        None => return Err(Box::new(Error::GraphInvalidList)),
                                    }
                                }
                                None => return Err(Box::new(Error::GraphInvalidList)),
                            }
                        }
                        None => return Err(Box::new(Error::GraphUnexpectedEndOfList)),
                    }
                }

                if head.is_some() {
                    return Err(Box::new(Error::GraphExpectedEndOfList));
                }

                Ok(())
            }
            (None, None) => Ok(()),
            (None, Some(expected)) => Err(Box::new(Error::GraphExpectedObject(
                p.to_owned(),
                format_list(expected),
            ))),
            (Some(o), None) => Err(Box::new(Error::GraphUnexpectedObject(
                p.to_owned(),
                o.to_string(),
            ))),
        })
    }

    /// When `expected_o` is `Some(iri)`.
    /// take any statement of the form `s p o` for the given `s` and `p`
    /// and checks that `o` is equal to `iri`.
    /// When `expected_o` is `None`,
    /// checks that no statement has the form `s p o`.
    fn take_object_and_assert_eq_iri(
        &mut self,
        s: &Self::Subject,
        p: Iri,
        expected_o: Option<Iri>,
    ) -> Result<(), Box<Error>> {
        self.take_object_and_assert_eq(s, p, expected_o, |o, expected_iri| match o {
            Object::Id(Id::Iri(i)) => i == expected_iri,
            _ => false,
        })
    }

    /// When `expected_o` is `Some(str)`.
    /// take any statement of the form `s p o` for the given `s` and `p`
    /// and checks that `o` is an IRI or string literal equal to `str`.
    /// When `expected_o` is `None`,
    /// checks that no statement has the form `s p o`.
    fn take_object_and_assert_eq_iri_or_str(
        &mut self,
        s: &Self::Subject,
        p: Iri,
        expected_o: Option<&str>,
    ) -> Result<(), Box<Error>>{
        self.take_object_and_assert_eq(s, p, expected_o, |o, expected| match o {
            Object::Id(Id::Iri(i)) => i.as_str() == *expected,
            Object::Literal(rdf_types::Literal::String(s)) => s.as_str() == *expected,
            _ => false,
        })
    }
    /// When `expected_o` is `Some(json)`.
    /// take any statement of the form `s p o` for the given `s` and `p`
    /// and checks that `o` is equal to the JSON value `json`.
    /// When `expected_o` is `None`,
    /// checks that no statement has the form `s p o`.

    fn take_object_and_assert_eq_json(
        &mut self,
        s: &Self::Subject,
        p: Iri,
        expected_o: Option<&serde_json::Value>,
    ) -> Result<(), Box<Error>> {
        self.take_object_and_assert_eq(s, p, expected_o, |o, expected| match o {
            rdf_types::Object::Literal(rdf_types::Literal::TypedString(json, ty))
                if *ty == RDF_JSON =>
            {
                match serde_json::from_str::<serde_json::Value>(json) {
                    Ok(json) => json == **expected,
                    _ => false,
                }
            }
            _ => false,
        })
    }

}

impl GraphValidate for Graph
where
    for<'a> IriBuf: Borrow<iref::Iri<'a>>,
{}