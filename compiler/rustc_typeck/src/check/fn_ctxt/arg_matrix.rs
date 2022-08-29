use std::cmp;

use rustc_index::vec::IndexVec;
use rustc_middle::ty::error::TypeError;

rustc_index::newtype_index! {
    pub(crate) struct ExpectedIdx {
        DEBUG_FORMAT = "ExpectedIdx({})",
    }
}

rustc_index::newtype_index! {
    pub(crate) struct ProvidedIdx {
        DEBUG_FORMAT = "ProvidedIdx({})",
    }
}

impl ExpectedIdx {
    pub fn to_provided_idx(self) -> ProvidedIdx {
        ProvidedIdx::from_usize(self.as_usize())
    }
}

// An issue that might be found in the compatibility matrix
#[derive(Debug)]
enum Issue {
    /// The given argument is the invalid type for the input
    Invalid(usize),
    /// There is a missing input
    Missing(usize),
    /// There's a superfluous argument
    Extra(usize),
    /// Two arguments should be swapped
    Swap(usize, usize),
    /// Several arguments should be reordered
    Permutation(Vec<Option<usize>>),
}

#[derive(Clone, Debug)]
pub(crate) enum Compatibility<'tcx> {
    Compatible,
    Incompatible(Option<TypeError<'tcx>>),
}

/// Similar to `Issue`, but contains some extra information
#[derive(Debug)]
pub(crate) enum Error<'tcx> {
    /// The provided argument is the invalid type for the expected input
    Invalid(ProvidedIdx, ExpectedIdx, Compatibility<'tcx>),
    /// There is a missing input
    Missing(ExpectedIdx),
    /// There's a superfluous argument
    Extra(ProvidedIdx),
    /// Two arguments should be swapped
    Swap(ProvidedIdx, ProvidedIdx, ExpectedIdx, ExpectedIdx),
    /// Several arguments should be reordered
    Permutation(Vec<(ExpectedIdx, ProvidedIdx)>),
}

pub(crate) struct ArgMatrix<'tcx> {
    /// Maps the indices in the `compatibility_matrix` rows to the indices of
    /// the *user provided* inputs
    provided_indices: Vec<ProvidedIdx>,
    /// Maps the indices in the `compatibility_matrix` columns to the indices
    /// of the *expected* args
    expected_indices: Vec<ExpectedIdx>,
    /// The first dimension (rows) are the remaining user provided inputs to
    /// match and the second dimension (cols) are the remaining expected args
    /// to match
    compatibility_matrix: Vec<Vec<Compatibility<'tcx>>>,
}

impl<'tcx> ArgMatrix<'tcx> {
    pub(crate) fn new<F: FnMut(ProvidedIdx, ExpectedIdx) -> Compatibility<'tcx>>(
        provided_count: usize,
        expected_input_count: usize,
        mut is_compatible: F,
    ) -> Self {
        let compatibility_matrix = (0..provided_count)
            .map(|i| {
                (0..expected_input_count)
                    .map(|j| is_compatible(ProvidedIdx::from_usize(i), ExpectedIdx::from_usize(j)))
                    .collect()
            })
            .collect();
        ArgMatrix {
            provided_indices: (0..provided_count).map(ProvidedIdx::from_usize).collect(),
            expected_indices: (0..expected_input_count).map(ExpectedIdx::from_usize).collect(),
            compatibility_matrix,
        }
    }

    /// Remove a given input from consideration
    fn eliminate_provided(&mut self, idx: usize) {
        self.provided_indices.remove(idx);
        self.compatibility_matrix.remove(idx);
    }

    /// Remove a given argument from consideration
    fn eliminate_expected(&mut self, idx: usize) {
        self.expected_indices.remove(idx);
        for row in &mut self.compatibility_matrix {
            row.remove(idx);
        }
    }

    /// "satisfy" an input with a given arg, removing both from consideration
    fn satisfy_input(&mut self, provided_idx: usize, expected_idx: usize) {
        self.eliminate_provided(provided_idx);
        self.eliminate_expected(expected_idx);
    }

    fn print_mat(&self, msg: &str) {
        println!("================== {} ==================", msg);
        let mat = &self.compatibility_matrix;
        let mut head = false;
        for (i, row) in mat.iter().enumerate() {
            if !head {
                print!("x| ");
                for (j, _) in row.iter().enumerate() {
                    print!(" {} ", j);
                }
                print!("\n-| ");
                for (_, _) in row.iter().enumerate() {
                    print!(" - ");
                }
                head = true;
                println!();
            }
            print!("{}| ", i);
            for (_j, cell) in row.iter().enumerate() {
                match cell {
                    Compatibility::Compatible => print!(" 1 "),
                    Compatibility::Incompatible(_err) => {
                        print!(" 0 ");
                    }
                }
            }
            println!();
        }
    }

    // Returns a `Vec` of (user input, expected arg) of matched arguments. These
    // are inputs on the remaining diagonal that match.
    fn eliminate_satisfied(&mut self) -> Vec<(ProvidedIdx, ExpectedIdx)> {
        let num_args = cmp::min(self.provided_indices.len(), self.expected_indices.len());
        let mut eliminated = vec![];
        for i in (0..num_args).rev() {
            if matches!(self.compatibility_matrix[i][i], Compatibility::Compatible) {
                eliminated.push((self.provided_indices[i], self.expected_indices[i]));
                self.satisfy_input(i, i);
            }
        }
        eliminated
    }

    // Find some issue in the compatibility matrix
    fn find_issue(&self) -> Option<Issue> {
        let mat = &self.compatibility_matrix;
        let ai = &self.expected_indices;
        let ii = &self.provided_indices;

        println!("ai: {:?}", ai);
        println!("ii: {:?}", ii);
        self.print_mat("find_issue:"); 
        let mut cur_matched_idx = 0;
        for i in 0..cmp::max(ai.len(), ii.len()) {
            // If we eliminate the last row, any left-over arguments are considered missing
            if i >= mat.len() {
                return Some(Issue::Missing(cur_matched_idx));
            }
            // If we eliminate the last column, any left-over inputs are extra
            if mat[i].len() == 0 {
                return Some(Issue::Extra(cur_matched_idx));
            }

            // Make sure we don't pass the bounds of our matrix
            let is_arg = i < ai.len();
            let is_input = i < ii.len();
            if is_arg && is_input && matches!(mat[i][i], Compatibility::Compatible) {
                // This is a satisfied input, so move along
                cur_matched_idx += 1;
                continue;
            }

            let mut useless = true;
            let mut unsatisfiable = true;
            if is_arg {
                for j in 0..ii.len() {
                    // If we find at least one input this argument could satisfy
                    // this argument isn't unsatisfiable
                    if matches!(mat[j][i], Compatibility::Compatible) {
                        unsatisfiable = false;
                        break;
                    }
                }
            }
            if is_input {
                for j in 0..ai.len() {
                    // If we find at least one argument that could satisfy this input
                    // this input isn't useless
                    if matches!(mat[i][j], Compatibility::Compatible) {
                        useless = false;
                        break;
                    }
                }
            }

            println!(
                "i: {}, is_input: {}, is_arg: {}, useless: {}, unsatisfiable: {}",
                i, is_input, is_arg, useless, unsatisfiable
            ); 
            match (is_input, is_arg, useless, unsatisfiable) {
                // If an argument is unsatisfied, and the input in its position is useless
                // then the most likely explanation is that we just got the types wrong
                (true, true, true, true) => return Some(Issue::Invalid(i)),
                // Otherwise, if an input is useless, then indicate that this is an extra argument
                (true, _, true, _) => return Some(Issue::Extra(i)),
                // Otherwise, if an argument is unsatisfiable, indicate that it's missing
                (_, true, _, true) => return Some(Issue::Missing(i)),
                (true, true, _, _) => {
                    // The argument isn't useless, and the input isn't unsatisfied,
                    // so look for a parameter we might swap it with
                    // We look for swaps explicitly, instead of just falling back on permutations
                    // so that cases like (A,B,C,D) given (B,A,D,C) show up as two swaps,
                    // instead of a large permutation of 4 elements.
                    for j in 0..cmp::min(ai.len(), ii.len()) {
                        if i == j || matches!(mat[j][j], Compatibility::Compatible) {
                            continue;
                        }
                        if matches!(mat[i][j], Compatibility::Compatible)
                            && matches!(mat[j][i], Compatibility::Compatible)
                        {
                            return Some(Issue::Swap(i, j));
                        }
                    }
                }
                _ => {
                    continue;
                }
            }
        }

        // We didn't find any of the individual issues above, but
        // there might be a larger permutation of parameters, so we now check for that
        // by checking for cycles
        // We use a double option at position i in this vec to represent:
        // - None: We haven't computed anything about this argument yet
        // - Some(None): This argument definitely doesn't participate in a cycle
        // - Some(Some(x)): the i-th argument could permute to the x-th position
        let mut permutation: Vec<Option<Option<usize>>> = vec![None; mat.len()];
        let mut permutation_found = false;
        for i in 0..mat.len() {
            if permutation[i].is_some() {
                // We've already decided whether this argument is or is not in a loop
                continue;
            }

            let mut stack = vec![];
            let mut j = i;
            let mut last = i;
            let mut is_cycle = true;
            loop {
                stack.push(j);
                // Look for params this one could slot into
                let compat: Vec<_> =
                    mat[j]
                        .iter()
                        .enumerate()
                        .filter_map(|(i, c)| {
                            if matches!(c, Compatibility::Compatible) { Some(i) } else { None }
                        })
                        .collect();
                println!("loop i: {} j: {} compat: {:?}", i, j, compat);
                if compat.len() != 1 {
                    // this could go into multiple slots, don't bother exploring both
                    is_cycle = false;
                    break;
                }
                j = compat[0];
                if stack.contains(&j) {
                    last = j;
                    break;
                }
            }
            if stack.len() <= 2 {
                // If we encounter a cycle of 1 or 2 elements, we'll let the
                // "satisfy" and "swap" code above handle those
                is_cycle = false;
            }
            // We've built up some chain, some of which might be a cycle
            // ex: [1,2,3,4]; last = 2; j = 2;
            // So, we want to mark 4, 3, and 2 as part of a permutation
            permutation_found = is_cycle;
            println!("permutation_found: {}", permutation_found);
            while let Some(x) = stack.pop() {
                if is_cycle {
                    permutation[x] = Some(Some(j));
                    j = x;
                    if j == last {
                        // From here on out, we're a tail leading into a cycle,
                        // not the cycle itself
                        is_cycle = false;
                    }
                } else {
                    // Some(None) ensures we save time by skipping this argument again
                    permutation[x] = Some(None);
                    println!("now permutation: {:?}", permutation);
                }
            }
        }

        if permutation_found {
            // Map unwrap to remove the first layer of Some
            let final_permutation: Vec<Option<usize>> =
                permutation.into_iter().map(|x| x.unwrap()).collect();
            return Some(Issue::Permutation(final_permutation));
        }
        return None;
    }

    // Obviously, detecting exact user intention is impossible, so the goal here is to
    // come up with as likely of a story as we can to be helpful.
    //
    // We'll iteratively removed "satisfied" input/argument pairs,
    // then check for the cases above, until we've eliminated the entire grid
    //
    // We'll want to know which arguments and inputs these rows and columns correspond to
    // even after we delete them.
    pub(crate) fn find_errors(
        mut self,
    ) -> (Vec<Error<'tcx>>, IndexVec<ExpectedIdx, Option<ProvidedIdx>>) {
        let provided_arg_count = self.provided_indices.len();

        let mut errors: Vec<Error<'tcx>> = vec![];
        // For each expected argument, the matched *actual* input
        let mut matched_inputs: IndexVec<ExpectedIdx, Option<ProvidedIdx>> =
            IndexVec::from_elem_n(None, self.expected_indices.len());

        // Before we start looking for issues, eliminate any arguments that are already satisfied,
        // so that an argument which is already spoken for by the input it's in doesn't
        // spill over into another similarly typed input
        // ex:
        //   fn some_func(_a: i32, _b: i32) {}
        //   some_func(1, "");
        // Without this elimination, the first argument causes the second argument
        // to show up as both a missing input and extra argument, rather than
        // just an invalid type.
        for (provided, expected) in self.eliminate_satisfied() {
            matched_inputs[expected] = Some(provided);
        }

        while !self.provided_indices.is_empty() || !self.expected_indices.is_empty() {
            let res = self.find_issue();
            println!("res: {:?}", res);
            match res {
                Some(Issue::Invalid(idx)) => {
                    let compatibility = self.compatibility_matrix[idx][idx].clone();
                    let input_idx = self.provided_indices[idx];
                    let arg_idx = self.expected_indices[idx];
                    self.satisfy_input(idx, idx);
                    errors.push(Error::Invalid(input_idx, arg_idx, compatibility));
                }
                Some(Issue::Extra(idx)) => {
                    let input_idx = self.provided_indices[idx];
                    self.eliminate_provided(idx);
                    errors.push(Error::Extra(input_idx));
                }
                Some(Issue::Missing(idx)) => {
                    let arg_idx = self.expected_indices[idx];
                    self.eliminate_expected(idx);
                    errors.push(Error::Missing(arg_idx));
                }
                Some(Issue::Swap(idx, other)) => {
                    let input_idx = self.provided_indices[idx];
                    let other_input_idx = self.provided_indices[other];
                    let arg_idx = self.expected_indices[idx];
                    let other_arg_idx = self.expected_indices[other];
                    let (min, max) = (cmp::min(idx, other), cmp::max(idx, other));
                    self.satisfy_input(min, max);
                    // Subtract 1 because we already removed the "min" row
                    self.satisfy_input(max - 1, min);
                    errors.push(Error::Swap(input_idx, other_input_idx, arg_idx, other_arg_idx));
                    matched_inputs[other_arg_idx] = Some(input_idx);
                    matched_inputs[arg_idx] = Some(other_input_idx);
                }
                Some(Issue::Permutation(args)) => {
                    let mut idxs: Vec<usize> = args.iter().filter_map(|&a| a).collect();

                    let mut real_idxs: IndexVec<ProvidedIdx, Option<(ExpectedIdx, ProvidedIdx)>> =
                        IndexVec::from_elem_n(None, provided_arg_count);
                    for (src, dst) in
                        args.iter().enumerate().filter_map(|(src, dst)| dst.map(|dst| (src, dst)))
                    {
                        let src_input_idx = self.provided_indices[src];
                        let dst_input_idx = self.provided_indices[dst];
                        let dest_arg_idx = self.expected_indices[dst];
                        real_idxs[src_input_idx] = Some((dest_arg_idx, dst_input_idx));
                        matched_inputs[dest_arg_idx] = Some(src_input_idx);
                    }
                    idxs.sort();
                    idxs.reverse();
                    for i in idxs {
                        self.satisfy_input(i, i);
                    }
                    errors.push(Error::Permutation(real_idxs.into_iter().flatten().collect()));
                }
                None => {
                    // We didn't find any issues, so we need to push the algorithm forward
                    // First, eliminate any arguments that currently satisfy their inputs
                    let eliminated = self.eliminate_satisfied();
                    //assert!(!eliminated.is_empty(), "didn't eliminated any indice in this round");
                    if eliminated.is_empty() {
                        println!("not found anything ....");
                        return (errors, matched_inputs);
                    }
                    for (inp, arg) in eliminated {
                        matched_inputs[arg] = Some(inp);
                    }
                }
            };
        }

        return (errors, matched_inputs);
    }
}
