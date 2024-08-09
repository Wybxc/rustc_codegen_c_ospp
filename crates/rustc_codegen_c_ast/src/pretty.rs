//! Pretty printing

// TODO: pretty printing test

use std::borrow::Cow;

use rustc_ast_pretty::pp;

pub const INDENT: isize = 2;

/// Pretty printer
pub struct Printer {
    pp: pp::Printer,
}

impl Default for Printer {
    fn default() -> Self {
        Self::new()
    }
}

impl Printer {
    pub fn new() -> Self {
        Self { pp: pp::Printer::new() }
    }

    pub fn finish(self) -> String {
        self.pp.eof()
    }

    pub(crate) fn seperated<T>(
        &mut self,
        sep: &'static str,
        elements: &[T],
        mut op: impl FnMut(&mut Self, &T),
    ) {
        if let Some((first, rest)) = elements.split_first() {
            op(self, first);
            for elt in rest {
                self.pp.word_space(sep);
                op(self, elt);
            }
        }
    }

    pub(crate) fn ibox(&mut self, indent: isize, op: impl FnOnce(&mut Self)) {
        self.pp.ibox(indent);
        op(self);
        self.pp.end();
    }

    pub(crate) fn ibox_delim(
        &mut self,
        indent: isize,
        delim: (&'static str, &'static str),
        op: impl FnOnce(&mut Self),
    ) {
        self.word(delim.0);
        self.ibox(indent, op);
        self.word(delim.1);
    }

    pub(crate) fn cbox(&mut self, indent: isize, op: impl FnOnce(&mut Self)) {
        self.pp.cbox(indent);
        op(self);
        self.pp.end();
    }

    pub(crate) fn cbox_delim(
        &mut self,
        indent: isize,
        delim: (&'static str, &'static str),
        padding: usize,
        op: impl FnOnce(&mut Self),
    ) {
        self.word(delim.0);
        self.pp.break_offset(padding, indent);
        self.cbox(indent, |this| {
            op(this);
            this.pp.break_offset(padding, -indent);
        });
        self.word(delim.1);
    }

    pub(crate) fn valign(&mut self, op: impl FnOnce(&mut Self)) {
        self.pp.visual_align();
        op(self);
        self.pp.end();
    }

    pub(crate) fn valign_delim(
        &mut self,
        delim: (&'static str, &'static str),
        op: impl FnOnce(&mut Self),
    ) {
        self.valign(|this| {
            this.word(delim.0);
            op(this);
            this.word(delim.1);
        });
    }

    /// Soft break: space if fits, otherwise newline
    pub(crate) fn softbreak(&mut self) {
        self.pp.space()
    }

    /// Hard break: always newline
    pub(crate) fn hardbreak(&mut self) {
        self.pp.hardbreak();
    }

    /// Zero break: nothing if fits, otherwise newline
    pub(crate) fn zerobreak(&mut self) {
        self.pp.zerobreak();
    }

    pub(crate) fn break_offset(&mut self, n: usize, offset: isize) {
        self.pp.break_offset(n, offset)
    }

    /// Print a string
    pub(crate) fn word(&mut self, s: impl Into<Cow<'static, str>>) {
        self.pp.word(s)
    }

    /// Non-breaking space, the same as `word(" ")`
    pub(crate) fn nbsp(&mut self) {
        self.pp.nbsp()
    }
}
