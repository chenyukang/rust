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
    last_delim_empty_block_spans: FxHashMap<Delimiter, Span>,

    /// Delimiters which have been reported as mismatching error already.
    reported_mismatched_delims: FxHashMap<Span, Delimiter>,

    /// Delimiter number to record the number of unmatched open delimiters.
    /// If the number is > 0, then we will report unclosed_delimiter error.
    unclosed_open_delims: u32,

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
            last_delim_empty_block_spans: FxHashMap::default(),
            reported_mismatched_delims: FxHashMap::default(),
            unclosed_open_delims: 0,
            matching_block_spans: Vec::new(),
        };
        let res = tt_reader.parse_token_trees(/* open_delimit */ None, None);
        (res, tt_reader.unmatched_braces)
    }

    // Parse a stream of tokens into a list of `TokenTree`s.
    fn parse_token_trees(
        &mut self,
        open_delimit: Option<Delimiter>,
        open_span: Option<Span>,
    ) -> PResult<'a, TokenStream> {
        self.token = self.string_reader.next_token().0;
        let mut buf = Vec::new();
        loop {
            match self.token.kind {
                token::OpenDelim(delim) => buf.push(self.parse_token_tree_open_delim(delim)?),
                token::CloseDelim(delim) => {
                    debug!(
                        "yukang now return open = {:?} delim = {:?}, token: {:?}",
                        open_delimit, delim, self.token
                    );
                    self.unclosed_open_delims -= 1;
                    if open_delimit == Some(delim) {
                        // correct close delimiter
                        return Ok(TokenStream::new(buf));
                    } else if open_delimit.is_none() {
                        // unexpected close delimiter
                        return Err(self.report_delim_err(delim, open_span));
                    } else {
                        // mismatch delimiter
                        if self.last_unclosed_found_span.is_none()
                            || self.last_unclosed_found_span != open_span
                        {
                            self.last_unclosed_found_span = open_span;
                            self.report_delim_err(delim, open_span).emit();
                            if let Some(span) = open_span {
                                self.reported_mismatched_delims.insert(span, delim);
                            }
                        }

                        if self.open_braces.iter().any(|&(b, _)| b == delim) {
                            self.open_braces.pop();
                            return Ok(TokenStream::new(buf));
                        }

                        let close_span = self.token.span;
                        let same_ident = open_span.is_some()
                            && self.same_identation_level(open_span.unwrap(), close_span);
                        debug!(
                            "yukang now same_ident = {:?} open: {:?}, close: {:?}",
                            same_ident,
                            open_delimit.unwrap(),
                            delim
                        );
                        debug!("yukang before move next: {:?}", self.token);
                        debug!("yukang after move next: {:?}", self.token);
                        if same_ident {
                            self.open_braces.pop();
                            debug!("yukang now open_braces = {:?}", self.open_braces);
                            return Ok(TokenStream::new(buf));
                        }
                        self.token = self.string_reader.next_token().0;
                    };
                }
                token::Eof => {
                    if open_delimit.is_some() && self.unclosed_open_delims > 0 {
                        self.report_eof_err();
                    }
                    for t in buf.iter_mut() {
                        debug!("yukang final return tree t = {:?}", t);
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

    fn report_eof_err(&mut self) {
        let mut need_report = vec![];
        for &(_, sp) in &self.open_braces {
            if !(self.reported_mismatched_delims.contains_key(&sp)
                || self.reported_mismatched_delims.keys().any(|&k| k.gt(&sp)))
            {
                need_report.push(sp);
            }
        }
        if need_report.is_empty() {
            return;
        }
        let msg = "this file contains an unclosed delimiter";
        let mut err = self.string_reader.sess.span_diagnostic.struct_span_err(self.token.span, msg);

        for sp in need_report {
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
            self.error_prone_delim_block(*delim, &mut err);
        }
        err.emit();
    }

    fn parse_token_tree_open_delim(&mut self, open_delim: Delimiter) -> PResult<'a, TokenTree> {
        // The span for beginning of the delimited section
        let pre_span = self.token.span;

        self.open_braces.push((open_delim, self.token.span));
        self.unclosed_open_delims += 1;
        debug!("yukang begin parse_token_tree_open_delim: {:?}", self.open_braces);

        // Parse the token trees within the delimiters.
        // We stop at any delimiter so we can try to recover if the user
        // uses an incorrect delimiter.
        let tts = self
            .parse_token_trees(/* open_delimit */ Some(open_delim), Some(self.token.span))?;

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
                        self.last_delim_empty_block_spans.insert(open_delim, empty_block_span);
                    }
                }

                //only add braces
                if let (Delimiter::Brace, Delimiter::Brace) = (open_brace, open_delim) {
                    self.matching_block_spans.push((open_brace_span, close_brace_span));
                }

                // Add all the matching spans, we will sort by span later
                self.matching_delim_spans.push((open_brace, open_brace_span, close_brace_span));

                // Move past the closing delimiter.
                self.token = self.string_reader.next_token().0;
                debug!("yukang correct close_delim: {:?}", self.open_braces);
            }
            token::CloseDelim(close_delim) => {
                // Move past the closing delimiter.
                let match_last =
                    if let Some(&(last, _)) = self.open_braces.last() &&
                        last == close_delim {
                        true
                    } else { false };

                if !match_last {
                    self.token = self.string_reader.next_token().0;
                }
            }
            token::Eof => {
                // Silently recover, the EOF token will be seen again
                // and an error emitted then. Thus we don't pop from
                // self.open_braces here.
            }
            _ => {}
        }

        debug!(
            "yukang finished parse_token_tree_open_delim : open: {:?} # curr_token = {:?}",
            open_delim, self.token
        );

        let sm = self.string_reader.sess.source_map();
        if let Ok(code) = sm.span_to_snippet(pre_span.with_hi(self.token.span.lo())) {
            debug!("yukang code: {:?}", code);
        }

        Ok(TokenTree::Delimited(delim_span, open_delim, tts))
    }

    fn report_delim_err(&mut self, delim: Delimiter, open_span: Option<Span>) -> PErr<'a> {
        // An unexpected closing delimiter (i.e., there is no
        // matching opening delimiter).
        let token_str = token_to_string(&self.token);
        let err_type = if open_span.is_some() { "mismatched" } else { "unexpected" };
        let msg = format!("{} closing delimiter: `{}`", err_type, token_str);
        let mut err =
            self.string_reader.sess.span_diagnostic.struct_span_err(self.token.span, &msg);

        self.error_prone_delim_block(delim, &mut err);
        if let Some(span) = open_span {
            err.span_label(span.shrink_to_lo(), "unmatched opening delimiter begins here");
        }
        err.span_label(self.token.span, format!("{} closing delimiter", err_type));
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

    fn error_prone_delim_block(&self, delim: Delimiter, err: &mut Diagnostic) {
        let mut matched_spans = vec![];
        let mut candidate_span = None;

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
        }
    }
}
