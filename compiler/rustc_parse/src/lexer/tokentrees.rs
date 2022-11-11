use super::{StringReader, UnmatchedBrace};
use rustc_ast::token::{self, Delimiter, Token};
use rustc_ast::tokenstream::{DelimSpan, Spacing, TokenStream, TokenTree};
use rustc_ast_pretty::pprust::token_to_string;
use rustc_data_structures::fx::FxHashMap;
use rustc_errors::{Diagnostic, PErr, PResult};
use rustc_span::Span;

pub(super) struct TokenTreesReader<'a> {
    string_reader: StringReader<'a>,
    /// The "next" token, which has been obtained from the `StringReader` but
    /// not yet handled by the `TokenTreesReader`.
    token: Token,
    /// Stack of open delimiters and their spans. Used for error message.
    open_braces: Vec<(Delimiter, Span)>,
    unmatched_braces: Vec<UnmatchedBrace>,
    /// The type and spans for all braces
    ///
    /// Used only for error recovery when arriving to EOF with mismatched braces.
    matching_delim_spans: Vec<(Delimiter, Span, Span)>,
    last_unclosed_found_span: Option<Span>,

    /// Collect empty block spans that might have been auto-inserted by editors.
    empty_block_spans: FxHashMap<Span, Delimiter>,

    /// Collect the spans of braces (Open, Close). Used only
    /// for detecting if blocks are empty and only braces.
    matching_block_spans: Vec<(Span, Span)>,
}

impl<'a> TokenTreesReader<'a> {
    pub(super) fn parse_all_token_trees(
        string_reader: StringReader<'a>,
    ) -> (PResult<'a, TokenStream>, Vec<UnmatchedBrace>) {
        let mut tt_reader = TokenTreesReader {
            string_reader,
            token: Token::dummy(),
            open_braces: Vec::new(),
            unmatched_braces: Vec::new(),
            matching_delim_spans: Vec::new(),
            last_unclosed_found_span: None,
            empty_block_spans: FxHashMap::default(),
            matching_block_spans: Vec::new(),
        };
        let res = tt_reader.parse_token_trees(/* is_delimited */ false);
        (res, tt_reader.unmatched_braces)
    }

    // Parse a stream of tokens into a list of `TokenTree`s.
    fn parse_token_trees(&mut self, is_delimited: bool) -> PResult<'a, TokenStream> {
        self.token = self.string_reader.next_token().0;
        let mut buf = Vec::new();
        loop {
            match self.token.kind {
                token::OpenDelim(delim) => buf.push(self.parse_token_tree_open_delim(delim)),
                token::CloseDelim(delim) => {
                    return if is_delimited {
                        Ok(TokenStream::new(buf))
                    } else {
                        Err(self.close_delim_err(delim))
                    };
                }
                token::Eof => {
                    if is_delimited {
                        self.eof_err().emit();
                    }
                    return Ok(TokenStream::new(buf));
                }
                _ => {
                    // Get the next normal token. This might require getting multiple adjacent
                    // single-char tokens and joining them together.
                    let (this_spacing, next_tok) = loop {
                        let (next_tok, is_next_tok_preceded_by_whitespace) =
                            self.string_reader.next_token();
                        if !is_next_tok_preceded_by_whitespace {
                            if let Some(glued) = self.token.glue(&next_tok) {
                                self.token = glued;
                            } else {
                                let this_spacing =
                                    if next_tok.is_op() { Spacing::Joint } else { Spacing::Alone };
                                break (this_spacing, next_tok);
                            }
                        } else {
                            break (Spacing::Alone, next_tok);
                        }
                    };
                    let this_tok = std::mem::replace(&mut self.token, next_tok);
                    buf.push(TokenTree::Token(this_tok, this_spacing));
                }
            }
        }
    }

    fn eof_err(&mut self) -> PErr<'a> {
        let msg = "this file contains an unclosed delimiter";
        let mut err = self.string_reader.sess.span_diagnostic.struct_span_err(self.token.span, msg);
        for &(_, sp) in &self.open_braces {
            err.span_label(sp, "unclosed delimiter");
            self.unmatched_braces.push(UnmatchedBrace {
                expected_delim: Delimiter::Brace,
                found_delim: None,
                found_span: self.token.span,
                unclosed_span: Some(sp),
                candidate_span: None,
            });
        }

        if let Some((delim, _)) = self.open_braces.last() {
            self.report_error_prone_delim_block(*delim, &mut err);
        }
        err
    }

    fn parse_token_tree_open_delim(&mut self, open_delim: Delimiter) -> TokenTree {
        // The span for beginning of the delimited section
        let pre_span = self.token.span;

        self.open_braces.push((open_delim, self.token.span));

        // Parse the token trees within the delimiters.
        // We stop at any delimiter so we can try to recover if the user
        // uses an incorrect delimiter.
        let tts = self.parse_token_trees(/* is_delimited */ true).unwrap();

        // Expand to cover the entire delimited token tree
        let delim_span = DelimSpan::from_pair(pre_span, self.token.span);

        match self.token.kind {
            // Correct delimiter.
            token::CloseDelim(close_delim) if close_delim == open_delim => {
                let (open_brace, open_brace_span) = self.open_braces.pop().unwrap();
                let close_brace_span = self.token.span;

                if tts.is_empty() {
                    let empty_block_span = open_brace_span.to(close_brace_span);
                    let sm = self.string_reader.sess.source_map();
                    if !sm.is_multiline(empty_block_span) {
                        // Only track if the block is in the form of `{}`, otherwise it is
                        // likely that it was written on purpose.
                        self.empty_block_spans.insert(empty_block_span, open_delim);
                    }
                }

                // only add braces
                if let (Delimiter::Brace, Delimiter::Brace) = (open_brace, open_delim) {
                    self.matching_block_spans.push((open_brace_span, close_brace_span));

                    // Add all the matching spans, we will sort by span later
                    self.matching_delim_spans.push((open_brace, open_brace_span, close_brace_span));
                }

                // Move past the closing delimiter.
                self.token = self.string_reader.next_token().0;
            }
            // Incorrect delimiter.
            token::CloseDelim(close_delim) => {
                let mut unclosed_delimiter = None;
                let mut candidate = None;

                if self.last_unclosed_found_span != Some(self.token.span) {
                    // do not complain about the same unclosed delimiter multiple times
                    self.last_unclosed_found_span = Some(self.token.span);
                    // This is a conservative error: only report the last unclosed
                    // delimiter. The previous unclosed delimiters could actually be
                    // closed! The parser just hasn't gotten to them yet.
                    if let Some(&(_, sp)) = self.open_braces.last() {
                        unclosed_delimiter = Some(sp);
                    };
                    for (brace, brace_span) in &self.open_braces {
                        if self.same_identation_level(self.token.span, *brace_span)
                            && brace == &close_delim
                        {
                            // high likelihood of these two corresponding
                            candidate = Some(*brace_span);
                        }
                    }
                    let (tok, _) = self.open_braces.pop().unwrap();
                    self.unmatched_braces.push(UnmatchedBrace {
                        expected_delim: tok,
                        found_delim: Some(close_delim),
                        found_span: self.token.span,
                        unclosed_span: unclosed_delimiter,
                        candidate_span: candidate,
                    });
                } else {
                    self.open_braces.pop();
                }

                // If the incorrect delimiter matches an earlier opening
                // delimiter, then don't consume it (it can be used to
                // close the earlier one). Otherwise, consume it.
                // E.g., we try to recover from:
                // fn foo() {
                //     bar(baz(
                // }  // Incorrect delimiter but matches the earlier `{`
                if !self.open_braces.iter().any(|&(b, _)| b == close_delim) {
                    self.token = self.string_reader.next_token().0;
                }
            }
            token::Eof => {
                // Silently recover, the EOF token will be seen again
                // and an error emitted then. Thus we don't pop from
                // self.open_braces here.
            }
            _ => unreachable!(),
        }

        TokenTree::Delimited(delim_span, open_delim, tts)
    }

    fn close_delim_err(&mut self, delim: Delimiter) -> PErr<'a> {
        // An unexpected closing delimiter (i.e., there is no
        // matching opening delimiter).
        let token_str = token_to_string(&self.token);
        let msg = format!("unexpected closing delimiter: `{}`", token_str);
        let mut err =
            self.string_reader.sess.span_diagnostic.struct_span_err(self.token.span, &msg);

        self.report_error_prone_delim_block(delim, &mut err);
        err.span_label(self.token.span, "unexpected closing delimiter");
        err
    }

    fn same_identation_level(&self, open_sp: Span, close_sp: Span) -> bool {
        let sm = self.string_reader.sess.source_map();
        if let (Some(open_padding), Some(close_padding)) =
            (sm.span_to_margin(open_sp), sm.span_to_margin(close_sp))
        {
            open_padding == close_padding
        } else {
            false
        }
    }

    fn report_error_prone_delim_block(&mut self, delim: Delimiter, err: &mut Diagnostic) {
        let mut matched_spans = vec![];

        for &(d, open_sp, close_sp) in &self.matching_delim_spans {
            if d == delim {
                let block_span = open_sp.with_hi(close_sp.lo());
                let same_ident = self.same_identation_level(open_sp, close_sp);
                matched_spans.push((block_span, same_ident));
            }
        }

        // sort by `lo`, so the large block spans in the front
        matched_spans.sort_by(|a, b| a.0.lo().cmp(&b.0.lo()));

        // We use larger block whose identation is well to cover those innert blocks
        // O(N^2) here, but we are on error reporting path, so it is fine
        for i in 0..matched_spans.len() {
            let (block_span, same_ident) = matched_spans[i];
            if same_ident {
                for j in i + 1..matched_spans.len() {
                    let (inner_block, innert_same_ident) = matched_spans[j];
                    if block_span.contains(inner_block) && !innert_same_ident {
                        matched_spans[j] = (inner_block, true);
                    }
                }
            }
        }

        let mut candidate_span = None;
        // Find the innermost span candidate for final report
        for (block_span, same_ident) in matched_spans.into_iter().rev() {
            if !same_ident {
                candidate_span = Some(block_span);
                break;
            }
        }

        if let Some(block_span) = candidate_span {
            err.span_label(
                block_span.shrink_to_lo(),
                "this delimiter might not be properly closed...",
            );
            err.span_label(
                block_span.shrink_to_hi(),
                "...as it matches this but it has different indentation",
            );

             // If there is a empty block in the mismatched span, note it
            for span in self.empty_block_spans.keys() {
                if let Some(d) = self.empty_block_spans.get(span) &&
                        *d == delim && block_span.contains(*span) {
                    err.span_label(*span, "block is empty, you might have not meant to close it");
                    break;
                }
            }
        }
    }
}
