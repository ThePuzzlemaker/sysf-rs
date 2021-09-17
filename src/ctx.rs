use std::{collections::HashMap, ops::Range};

use crate::trace;
#[cfg(feature = "trace")]
use tracing::instrument;

use crate::ast::core::{Term, Ty};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TyCtxtEntry {
    UnsolvedExst(usize),
    ExstMarker(usize),
    SolvedExst(usize, Ty),
    Uvar,
    TermVar(Ty),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TyCtxt {
    arr: Vec<TyCtxtEntry>,
    fresh_evar: usize,
}

impl TyCtxt {
    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn get_uvar(&self, idx: usize) -> Option<usize> {
        trace!("ctx/get_uvar/enter");
        let res = self
            .arr
            .iter()
            .rev()
            .enumerate()
            .filter_map(|(idx, x)| {
                if x == &TyCtxtEntry::Uvar {
                    Some(idx)
                } else {
                    None
                }
            })
            .nth(idx);
        trace!(?res, "ctx/get_uvar/leave");
        res
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn get_term_var(&self, idx: usize) -> Option<&Ty> {
        trace!("ctx/get_term_var/enter");
        let res = self
            .arr
            .iter()
            .rev()
            .enumerate()
            .filter_map(|(idx, x)| {
                if let TyCtxtEntry::TermVar(ty) = x {
                    Some(ty)
                } else {
                    None
                }
            })
            .nth(idx);
        trace!(?res, "ctx/get_term_var/leave");
        res
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn get_term_var_idx(&mut self, idx: usize) -> Option<usize> {
        trace!("ctx/get_term_var_mut/enter");
        let res = self
            .arr
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(idx, x)| {
                if let TyCtxtEntry::TermVar(ty) = x {
                    Some(idx)
                } else {
                    None
                }
            })
            .nth(idx);
        trace!(?res, "ctx/get_term_var_idx/leave");
        res
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    fn get_unsolved_evar_mut(&mut self, idx: usize) -> Option<&mut TyCtxtEntry> {
        trace!("ctx/get_unsolved_evar_mut/enter");
        let res = self.arr.iter_mut().find_map(|x| {
            if let TyCtxtEntry::UnsolvedExst(eidx) = x {
                if *eidx == idx {
                    return Some(x);
                }
            }
            None
        });
        trace!(?res, "ctx/get_unsolved_evar_mut/leave");
        res
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn contains_uvar(&self, idx: usize) -> bool {
        trace!("ctx/contains_uvar/enter");
        let res = self.get_uvar(idx).is_some();
        trace!(?res, "ctx/contains_uvar/leave");
        res
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn contains_evar(&self, idx: usize) -> bool {
        trace!("ctx/contains_evar/enter");
        let res = self.arr.iter().any(|x| match x {
            TyCtxtEntry::UnsolvedExst(i)
            | TyCtxtEntry::ExstMarker(i)
            | TyCtxtEntry::SolvedExst(i, _)
                if *i == idx =>
            {
                true
            }
            _ => false,
        });
        trace!(?res, "ctx/contains_evar/leave");
        res
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn drop_after_marker(&mut self, evar: usize) -> Option<()> {
        trace!("ctx/drop_after_marker/enter");
        let marker_idx = self.arr.iter().enumerate().find_map(|(idx, ty)| {
            if ty == &TyCtxtEntry::ExstMarker(evar) {
                Some(idx)
            } else {
                None
            }
        })?;

        // Drop everything after the marker.
        self.arr.truncate(marker_idx);

        trace!("ctx/drop_after_marker/leave: ok");
        Some(())
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn drop_after_uvar(&mut self, idx: usize) -> Option<()> {
        trace!("ctx/drop_after_uvar/enter");
        let uvar_idx = self.get_uvar(idx)?;

        // Drop everything after the uvar.
        self.arr.truncate(uvar_idx);

        trace!("ctx/drop_after_uvar/leave: ok");
        Some(())
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn drop_after_term_var(&mut self, idx: usize) -> Option<()> {
        trace!("ctx/drop_after_term_var/enter");
        let term_var_idx = self.get_term_var_idx(idx)?;

        // Drop everything after the term var.
        self.arr.truncate(term_var_idx);

        trace!("ctx/drop_after_term_var/leave: ok");
        Some(())
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn fresh_evar(&mut self) -> usize {
        trace!("ctx/fresh_evar/enter");
        let fresh_evar = self.fresh_evar;
        self.fresh_evar += 1;
        trace!(%fresh_evar, "ctx/fresh_evar/leave");
        fresh_evar
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn add_marker(&mut self, evar: usize) {
        trace!("ctx/add_marker");
        self.arr.push(TyCtxtEntry::ExstMarker(evar));
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn add_unsolved(&mut self, evar: usize) {
        trace!("ctx/add_unsolved");
        self.arr.push(TyCtxtEntry::UnsolvedExst(evar));
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn add_uvar(&mut self) {
        trace!("ctx/add_uvar");
        self.arr.push(TyCtxtEntry::Uvar);
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn add_solved(&mut self, evar: usize, ty: Ty) {
        trace!("ctx/add_solved");
        self.arr.push(TyCtxtEntry::SolvedExst(evar, ty));
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn add_term_var(&mut self, ty: Ty) {
        trace!("ctx/add_term_var");
        self.arr.push(TyCtxtEntry::TermVar(ty));
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn solve_evar(&mut self, evar: usize, ty: Ty) -> Option<()> {
        trace!("ctx/solve_evar/enter");
        let ent = self.get_unsolved_evar_mut(evar)?;
        *ent = TyCtxtEntry::SolvedExst(evar, ty);
        trace!("ctx/solve_evar/leave: ok");
        Some(())
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn insert_unsolved_before_evar(&mut self, evar: usize, unsolved: usize) -> Option<()> {
        trace!("ctx/insert_unsolved_before_evar/enter");
        let evar_idx = self.arr.iter().enumerate().find_map(|(idx, x)| {
            if let TyCtxtEntry::UnsolvedExst(eidx) = x {
                if *eidx == evar {
                    return Some(idx);
                }
            }
            None
        })?;

        self.arr
            .insert(evar_idx, TyCtxtEntry::UnsolvedExst(unsolved));

        trace!("ctx/insert_unsolved_before_evar/leave");
        Some(())
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn get_solved(&self) -> HashMap<usize, &Ty> {
        trace!("ctx/get_solved/enter");
        let res = self
            .arr
            .iter()
            .filter_map(|x| {
                if let TyCtxtEntry::SolvedExst(idx, ty) = x {
                    Some((*idx, ty))
                } else {
                    None
                }
            })
            .collect();
        trace!(?res, "ctx/get_solved/leave");
        res
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn clear(&mut self) {
        trace!("ctx/clear");
        self.arr.clear();
        self.fresh_evar = 0;
    }

    pub fn slice(&'_ self, range: Range<usize>) -> TyCtxtView<'_> {
        TyCtxtView {
            arr: &self.arr[range],
        }
    }

    pub fn slice_until_evar(&'_ self, evar: usize) -> Option<TyCtxtView<'_>> {
        let evar_idx = self.arr.iter().enumerate().find_map(|(idx, x)| match x {
            TyCtxtEntry::UnsolvedExst(i)
            | TyCtxtEntry::ExstMarker(i)
            | TyCtxtEntry::SolvedExst(i, _)
                if *i == evar =>
            {
                Some(idx)
            }
            _ => None,
        })?;
        Some(self.slice(0..evar_idx))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TyCtxtView<'a> {
    arr: &'a [TyCtxtEntry],
}

impl<'a> TyCtxtView<'a> {
    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn contains_evar(&self, idx: usize) -> bool {
        trace!("ctxview/contains_evar/enter");
        let res = self.arr.iter().any(|x| match x {
            TyCtxtEntry::UnsolvedExst(i)
            | TyCtxtEntry::ExstMarker(i)
            | TyCtxtEntry::SolvedExst(i, _)
                if *i == idx =>
            {
                true
            }
            _ => false,
        });
        trace!(?res, "ctxview/contains_evar/leave");
        res
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn contains_uvar(&self, idx: usize) -> bool {
        trace!("ctxview/contains_uvar/enter");
        let res = self.get_uvar(idx).is_some();
        trace!(?res, "ctxview/contains_uvar/leave");
        res
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(self)))]
    pub fn get_uvar(&self, idx: usize) -> Option<usize> {
        trace!("ctxview/get_uvar/enter");
        let res = self
            .arr
            .iter()
            .rev()
            .enumerate()
            .filter_map(|(idx, x)| {
                if x == &TyCtxtEntry::Uvar {
                    Some(idx)
                } else {
                    None
                }
            })
            .nth(idx);
        trace!(?res, "ctxview/get_uvar/leave");
        res
    }
}

impl<'a> From<&'a TyCtxt> for TyCtxtView<'a> {
    fn from(ctx: &'a TyCtxt) -> Self {
        Self { arr: &ctx.arr }
    }
}
