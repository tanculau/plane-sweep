use eframe::egui::Window;
use typed_index_collections::TiVec;

use crate::{intersection::IntersectionIdx, math::cartesian::CartesianCoord, segment::SegmentIdx};

pub mod intersection;
pub mod math;
pub mod segment;
#[cfg(feature = "test")]
pub mod test;

pub trait WidgetName {
    /// Name of the Widget.
    const NAME: &'static str;

    /// Long Name of the Widget.
    const NAME_LONG: &'static str = Self::NAME;

    /// Name of the Widget.
    fn name(&self) -> &'static str {
        Self::NAME
    }
    /// Long Name of the Widget.
    fn name_long(&self) -> &'static str {
        Self::NAME_LONG
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ToggleAbleWidget<T: MyWidget<State>, State> {
    view: T,
    is_open: bool,
    phantom: std::marker::PhantomData<State>,
}

impl<T: MyWidget<State>, State> ToggleAbleWidget<T, State> {
    pub const fn new(view: T, is_open: bool) -> Self {
        Self {
            view,
            is_open,
            phantom: core::marker::PhantomData,
        }
    }

    pub const fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }
    pub fn view(&mut self, ctx: &eframe::egui::Context, state: impl Into<State>) {
        self.view.show(ctx, &mut self.is_open, state);
    }

    pub fn add_toggle_value(&mut self, ui: &mut eframe::egui::Ui) {
        ui.toggle_value(&mut self.is_open, self.view.name());
    }
    pub const fn inner(&mut self) -> &mut T {
        &mut self.view
    }
}

impl<T: MyWidget<State>, State> WidgetName for ToggleAbleWidget<T, State> {
    const NAME: &'static str = T::NAME;
    const NAME_LONG: &'static str = T::NAME_LONG;
}

/// Display a widget in the UI.
pub trait MyWidget<State>: WidgetName {
    /// Draws the widget in the given UI context.
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: impl Into<State>);

    /// Draws the widget in a window.
    fn show(&mut self, ctx: &eframe::egui::Context, open: &mut bool, state: impl Into<State>) {
        Window::new(Self::NAME_LONG)
            .open(open)
            .resizable([true; 2])
            .scroll(true)
            .show(ctx, |ui| {
                self.ui(ui, state);
            });
    }
}

/// Common trait to generalize algorithm.
/// It represents a step in an algorithm that can be iterated over.
pub trait AlgrorithmStep {
    /// Returns the segment that are currently looked at by the algorithm.
    fn segments(&self) -> impl Iterator<Item = SegmentIdx>;
    /// Returns the intersections that are currently looked at by the algorithm.
    fn intersections(&self) -> impl Iterator<Item = IntersectionIdx>;

    fn sweep_line(&self) -> Option<CartesianCoord> {
        None
    }
}

pub type AlgoSteps<T> = TiVec<AlgoStepIdx, T>;

impl_idx!(AlgoStepIdx);
pub trait PushStep<T> {
    fn push(&mut self, step: T);
    fn clear(&mut self);
}

impl<T> PushStep<T> for AlgoSteps<T> {
    fn push(&mut self, step: T) {
        self.push(step);
    }
    fn clear(&mut self) {
        self.clear();
    }
}

impl<T> PushStep<T> for () {
    #[inline(always)]
    fn push(&mut self, _: T) {}
    #[inline(always)]
    fn clear(&mut self) {}
}

#[macro_export]
macro_rules! impl_idx {
    ($t : tt) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[repr(transparent)]
        pub struct $t(usize);

        impl From<usize> for $t {
            fn from(value: usize) -> Self {
                Self(value)
            }
        }
        impl From<&usize> for $t {
            fn from(value: &usize) -> Self {
                Self(*value)
            }
        }
        impl From<&mut usize> for $t {
            fn from(value: &mut usize) -> Self {
                Self(*value)
            }
        }
        impl From<$t> for usize {
            fn from(value: $t) -> Self {
                value.0
            }
        }
        impl From<&$t> for usize {
            fn from(value: &$t) -> Self {
                value.0
            }
        }
        impl From<&mut $t> for usize {
            fn from(value: &mut $t) -> Self {
                value.0
            }
        }
        impl From<&$t> for $t {
            fn from(value: &$t) -> Self {
                *value
            }
        }
        impl From<&mut $t> for $t {
            fn from(value: &mut $t) -> Self {
                *value
            }
        }
    };
}
