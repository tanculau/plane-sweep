use web_time::Instant;

use auto_enums::auto_enum;
use common::{
    PushStep,
    intersection::{IntersectionIdx, Intersections},
    segment::{Segment, SegmentIdx, Segments},
};
use tracing::{info, instrument};

pub mod ui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AlgorithmStep {
    Init,
    Running {
        step: usize,
        i: usize,
        j: usize,
        segment_i: SegmentIdx,
        segment_j: SegmentIdx,
        intersection: Option<IntersectionIdx>,
    },
    End,
}

impl AlgorithmStep {
    /// Returns `true` if the algorithm step is [`Init`].
    ///
    /// [`Init`]: AlgorithmStep::Init
    #[must_use]
    pub const fn is_init(&self) -> bool {
        matches!(self, Self::Init)
    }

    /// Returns `true` if the algorithm step is [`End`].
    ///
    /// [`End`]: AlgorithmStep::End
    #[must_use]
    pub const fn is_end(&self) -> bool {
        matches!(self, Self::End)
    }
}

impl common::AlgrorithmStep for AlgorithmStep {
    #[auto_enum(Iterator)]
    fn segments(&self) -> impl Iterator<Item = SegmentIdx> {
        match self {
            Self::Running {
                segment_i,
                segment_j,
                ..
            } => [*segment_i, *segment_j].into_iter(),
            _ => std::iter::empty(),
        }
    }

    #[auto_enum(Iterator)]
    fn intersections(&self) -> impl Iterator<Item = IntersectionIdx> {
        match self {
            Self::Running { intersection, .. } => intersection.iter().copied(),
            _ => std::iter::empty(),
        }
    }
}

#[instrument(name = "brute_force", skip_all)]
pub fn calculate_steps<T: PushStep<AlgorithmStep>>(
    segments: &Segments,
    intersections: &mut Intersections,
    steps: &mut T,
) {
    let time = Instant::now();
    intersections.clear();
    steps.clear();
    steps.push(AlgorithmStep::Init);

    let len = segments.len();
    let mut step = 1;

    for i in 0..len {
        for j in i + 1..len {
            let segment_i = i.into();
            let segment_j = j.into();

            let found_intersections = Segment::intersect([segment_i, segment_j], segments, step);
            let mut key = None;
            if let Some(intersection) = found_intersections {
                key = Some(intersections.push_and_get_key(intersection));
            }
            steps.push(AlgorithmStep::Running {
                step,
                i,
                j,
                segment_i,
                segment_j,
                intersection: key,
            });
            step += 1;
        }
    }

    steps.push(AlgorithmStep::End);
    let finished = time.elapsed();
    info!(
        "Brute force algorithm finished in {} ms in {} steps and found {} intersections",
        finished.as_millis(),
        step - 1,
        intersections.len()
    );
}
