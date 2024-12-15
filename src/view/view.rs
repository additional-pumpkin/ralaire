// Most of this file is adapted from xilem_core/src/sequence.rs
// Copyright 2024 the Xilem Authors
use crate::AsAny;

/// A type which can be a [`View`]. Imposes no requirements on the underlying type.
/// Should be implemented alongside every `View` implementation:
/// ```ignore
/// impl<...> ViewMarker for Button<...> {}
/// impl<...> View<...> for Button<...> {...}
/// ```
///
/// ## Details
///
/// Because `View` is generic, Rust [allows you](https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules) to implement this trait for certain non-local types.
/// These non-local types can include `Vec<_>` and `Option<_>`.
/// If this trait were not present, those implementations of `View` would conflict with those types' implementations of `ViewSequence`.
/// This is because every `View` type also implementations `ViewSequence`.
/// Since `ViewMarker` is not generic, these non-local implementations are not permitted for this trait, which means that the conflicting implementation cannot happen.
pub trait ViewMarker {}

pub trait View<State: 'static>: AsAny {
    type Element;
    fn build(&self) -> Self::Element;
    fn rebuild(&self, old: &Self, element: &mut Self::Element);
    fn teardown(&self, element: &mut Self::Element);
    fn get_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}
pub trait ViewSequence<State, Element> {
    fn seq_build(&self, elements: &mut Vec<Element>);
    fn seq_rebuild(&self, old: &Self, elements: &mut impl ElementSplice<Element>);
    fn seq_teardown(&self, elements: &mut impl ElementSplice<Element>);
}

/// A temporary "splice" to add, update and delete in an (ordered) sequence of elements.
/// It is mainly intended for view sequences.
pub trait ElementSplice<Element> {
    /// Run a function with access to the associated [`AppendVec`].
    ///
    /// Each element [pushed](AppendVec::push) to the provided vector will be logically
    /// [inserted](ElementSplice::insert) into `self`.
    fn with_scratch<R>(&mut self, f: impl FnOnce(&mut Vec<Element>) -> R) -> R;
    /// Insert a new element at the current index in the resulting collection.
    fn insert(&mut self, element: Element);
    /// Mutate the next existing element.
    fn mutate<R>(&mut self, f: impl FnOnce(&mut Element) -> R) -> R;
    /// Don't make any changes to the next n existing elements.
    fn skip(&mut self, n: usize);
    /// Remove the next existing element, after running a function on it.
    fn remove<R>(&mut self, f: impl FnOnce(&mut Element) -> R) -> R;
}

pub trait SuperElement<Child> {
    /// Convert from the child to this element type.
    fn upcast(child: Child) -> Self;

    /// Perform a reborrowing downcast to the child reference type.
    ///
    /// This may panic if `this` is not the reference form of a value created by
    /// `Self::upcast`.
    /// For example, this may perform a downcasting operation, which would fail
    /// if the value is not of the expected type.
    /// You can safely use this methods in contexts where it is known that the
    ///
    /// If you need to return a value, see [`with_downcast_val`](SuperElement::with_downcast_val).
    fn with_downcast(&mut self, f: impl FnOnce(&mut Child)) -> &mut Self {
        let (this, ()) = Self::with_downcast_val(self, f);
        this
    }
    /// Perform a reborrowing downcast.
    ///
    /// This may panic if `this` is not the reference form of a value created by
    /// `Self::upcast`.
    ///
    /// If you don't need to return a value, see [`with_downcast`](SuperElement::with_downcast).
    fn with_downcast_val<R>(&mut self, f: impl FnOnce(&mut Child) -> R) -> (&mut Self, R);
}

impl<State: 'static, V, Element> ViewSequence<State, Element> for V
where
    V: View<State> + ViewMarker,
    Element: SuperElement<V::Element>,
{
    fn seq_build(&self, elements: &mut Vec<Element>) {
        let element = self.build();
        elements.push(Element::upcast(element));
    }
    fn seq_rebuild(&self, old: &Self, elements: &mut impl ElementSplice<Element>) {
        // Mutate the item we added in `seq_build`
        elements.mutate(|this_element| {
            Element::with_downcast(this_element, |element| {
                self.rebuild(old, element);
            });
        });
    }

    fn seq_teardown(&self, elements: &mut impl ElementSplice<Element>) {
        elements.remove(|this_element| {
            Element::with_downcast(this_element, |element| {
                self.teardown(element);
            });
        })
    }
}

/// The implementation for an `Option` of a `ViewSequence`.
impl<State: 'static, Element, Seq> ViewSequence<State, Element> for Option<Seq>
where
    Seq: ViewSequence<State, Element>,
{
    fn seq_build(&self, elements: &mut Vec<Element>) {
        if let Some(seq) = self {
            seq.seq_build(elements);
        }
    }

    #[doc(hidden)]
    fn seq_rebuild(&self, prev: &Self, elements: &mut impl ElementSplice<Element>) {
        match (self, prev) {
            (None, None) => {
                // Nothing to do, there is no corresponding element
            }
            (Some(seq), Some(prev)) => {
                // Perform a normal rebuild
                seq.seq_rebuild(prev, elements);
            }
            (Some(seq), None) => {
                // The sequence is newly re-added, build the inner sequence
                // We don't increment the generation here, as that was already done in the below case
                elements.with_scratch(|elements| seq.seq_build(elements));
            }
            (None, Some(prev)) => {
                prev.seq_teardown(elements);
            }
        }
    }

    fn seq_teardown(&self, elements: &mut impl ElementSplice<Element>) {
        if let Some(seq) = self {
            seq.seq_teardown(elements);
        }
    }
}

/// The implementation for an `Vec` of a `ViewSequence`.
impl<State: 'static, Element, Seq> ViewSequence<State, Element> for Vec<Seq>
where
    Seq: ViewSequence<State, Element>,
{
    fn seq_build(&self, elements: &mut Vec<Element>) {
        for element in self {
            element.seq_build(elements);
        }
    }

    fn seq_rebuild(&self, prev: &Self, elements: &mut impl ElementSplice<Element>) {
        for (child, child_prev) in self.iter().zip(prev) {
            // Rebuild the items which are common to both vectors
            child.seq_rebuild(child_prev, elements);
        }
        let n = self.len();
        let prev_n = prev.len();
        if n > prev_n {
            elements.with_scratch(|elements| {
                for child in self[prev_n..].iter() {
                    child.seq_build(elements);
                }
            });
        } else if n < prev_n {
            for child in prev[n..].iter() {
                child.seq_teardown(elements);
            }
        }
    }
    fn seq_teardown(&self, elements: &mut impl ElementSplice<Element>) {
        for seq in self {
            seq.seq_teardown(elements);
        }
    }
}

/// The implementation for an Array of a `ViewSequence`.
impl<State: 'static, Element, Seq, const N: usize> ViewSequence<State, Element> for [Seq; N]
where
    Seq: ViewSequence<State, Element>,
{
    fn seq_build(&self, elements: &mut Vec<Element>) {
        for child in self {
            child.seq_build(elements);
        }
    }

    fn seq_rebuild(&self, prev: &Self, elements: &mut impl ElementSplice<Element>) {
        for (seq, prev_seq) in self.iter().zip(prev) {
            seq.seq_rebuild(prev_seq, elements);
        }
    }
    fn seq_teardown(&self, elements: &mut impl ElementSplice<Element>) {
        for seq in self {
            seq.seq_teardown(elements);
        }
    }
}

impl<State: 'static, Element> ViewSequence<State, Element> for () {
    fn seq_build(&self, _: &mut Vec<Element>) {}

    fn seq_rebuild(&self, _: &Self, _: &mut impl ElementSplice<Element>) {}

    fn seq_teardown(&self, _: &mut impl ElementSplice<Element>) {}
}
impl<State: 'static, Element, Seq> ViewSequence<State, Element> for (Seq,)
where
    Seq: ViewSequence<State, Element>,
{
    fn seq_build(&self, elements: &mut Vec<Element>) {
        self.0.seq_build(elements);
    }

    fn seq_rebuild(&self, prev: &Self, elements: &mut impl ElementSplice<Element>) {
        self.0.seq_rebuild(&prev.0, elements);
    }

    fn seq_teardown(&self, elements: &mut impl ElementSplice<Element>) {
        self.0.seq_teardown(elements);
    }
}

macro_rules! impl_view_tuple {
    (
        // We could use the ${index} metavariable here once it's stable
        // https://veykril.github.io/tlborm/decl-macros/minutiae/metavar-expr.html
        $($seq: ident, $idx: tt);+
    ) => {
        impl<
                State:'static,
                Element,
                $($seq: ViewSequence<State, Element>,)+
            > ViewSequence<State, Element> for ($($seq,)+)

        {
            fn seq_build(
                &self,
                elements: &mut Vec<Element>,
            ) {
                $(
                    self.$idx.seq_build(elements);
                )+
            }

            fn seq_rebuild(
                &self,
                prev: &Self,
                elements: &mut impl ElementSplice<Element>,
            ) {
                $(
                        self.$idx.seq_rebuild(&prev.$idx, elements);
                )+
            }
            fn seq_teardown(
                &self,
                elements: &mut impl ElementSplice<Element>,
            ) {
                $(
                        self.$idx.seq_teardown(elements);
                )+
            }
        }
    };
}

// We implement for tuples of length up to 15. 0 and 1 are special cased to be more efficient
impl_view_tuple!(Seq0, 0; Seq1, 1);
impl_view_tuple!(Seq0, 0; Seq1, 1; Seq2, 2);
impl_view_tuple!(Seq0, 0; Seq1, 1; Seq2, 2; Seq3, 3);
impl_view_tuple!(Seq0, 0; Seq1, 1; Seq2, 2; Seq3, 3; Seq4, 4);
impl_view_tuple!(Seq0, 0; Seq1, 1; Seq2, 2; Seq3, 3; Seq4, 4; Seq5, 5);
impl_view_tuple!(Seq0, 0; Seq1, 1; Seq2, 2; Seq3, 3; Seq4, 4; Seq5, 5; Seq6, 6);
impl_view_tuple!(Seq0, 0; Seq1, 1; Seq2, 2; Seq3, 3; Seq4, 4; Seq5, 5; Seq6, 6; Seq7, 7);
impl_view_tuple!(Seq0, 0; Seq1, 1; Seq2, 2; Seq3, 3; Seq4, 4; Seq5, 5; Seq6, 6; Seq7, 7; Seq8, 8);
impl_view_tuple!(Seq0, 0; Seq1, 1; Seq2, 2; Seq3, 3; Seq4, 4; Seq5, 5; Seq6, 6; Seq7, 7; Seq8, 8; Seq9, 9);
impl_view_tuple!(Seq0, 0; Seq1, 1; Seq2, 2; Seq3, 3; Seq4, 4; Seq5, 5; Seq6, 6; Seq7, 7; Seq8, 8; Seq9, 9; Seq10, 10);
impl_view_tuple!(Seq0, 0; Seq1, 1; Seq2, 2; Seq3, 3; Seq4, 4; Seq5, 5; Seq6, 6; Seq7, 7; Seq8, 8; Seq9, 9; Seq10, 10; Seq11, 11);
impl_view_tuple!(Seq0, 0; Seq1, 1; Seq2, 2; Seq3, 3; Seq4, 4; Seq5, 5; Seq6, 6; Seq7, 7; Seq8, 8; Seq9, 9; Seq10, 10; Seq11, 11; Seq12, 12);
impl_view_tuple!(Seq0, 0; Seq1, 1; Seq2, 2; Seq3, 3; Seq4, 4; Seq5, 5; Seq6, 6; Seq7, 7; Seq8, 8; Seq9, 9; Seq10, 10; Seq11, 11; Seq12, 12; Seq13, 13);
impl_view_tuple!(Seq0, 0; Seq1, 1; Seq2, 2; Seq3, 3; Seq4, 4; Seq5, 5; Seq6, 6; Seq7, 7; Seq8, 8; Seq9, 9; Seq10, 10; Seq11, 11; Seq12, 12; Seq13, 13; Seq14, 14);
impl_view_tuple!(Seq0, 0; Seq1, 1; Seq2, 2; Seq3, 3; Seq4, 4; Seq5, 5; Seq6, 6; Seq7, 7; Seq8, 8; Seq9, 9; Seq10, 10; Seq11, 11; Seq12, 12; Seq13, 13; Seq14, 14; Seq15, 15);
