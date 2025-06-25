use core::fmt::Debug;

use float_cmp::ApproxEq;
use googletest::{
    matcher::MatcherResult,
    prelude::{Matcher, MatcherBase},
};

pub fn approx_eq<T: ApproxEq + Debug + Copy>(expected: T) -> impl Matcher<T> {
    MyApproxEqMatcher { expected }
}

#[derive(MatcherBase)]
struct MyApproxEqMatcher<T> {
    expected: T,
}

impl<T: ApproxEq + Debug + Copy> Matcher<T> for MyApproxEqMatcher<T> {
    fn matches(&self, actual: T) -> googletest::matcher::MatcherResult {
        if float_cmp::approx_eq!(T, self.expected, actual) {
            MatcherResult::Match
        } else {
            MatcherResult::NoMatch
        }
    }

    fn describe(
        &self,
        matcher_result: googletest::matcher::MatcherResult,
    ) -> googletest::description::Description {
        match matcher_result {
            MatcherResult::Match => format!("is equal to {:?}", self.expected).into(),
            MatcherResult::NoMatch => format!("isn't equal to {:?}", self.expected).into(),
        }
    }
}
