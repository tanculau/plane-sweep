use common::{
    AlgoStepIdx, AlgoSteps, MyWidget, WidgetName,
    intersection::Intersections,
    segment::{SegmentIdx, Segments},
};
use eframe::egui::RichText;

use crate::{Step, StepType};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CodeView;

impl WidgetName for CodeView {
    const NAME: &'static str = "Code";
    const NAME_LONG: &'static str = "Code Viewer";
}
#[derive(Debug, Clone)]
pub struct CodeViewState<'a, 'b, 'c> {
    pub step: AlgoStepIdx,
    pub steps: &'a AlgoSteps<Step>,
    pub segments: &'b Segments,
    pub intersections: &'c Intersections,
}

impl<'a, 'b, 'c> MyWidget<CodeViewState<'a, 'b, 'c>> for CodeView {
    #[allow(clippy::too_many_lines)]
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: impl Into<CodeViewState<'a, 'b, 'c>>) {
        let CodeViewState {
            step,
            steps,
            segments,
            intersections,
        } = state.into();
        let s = &steps[step];

        if s.typ.is_init() {
            ui.label(RichText::new("Not yet started").heading().underline());
        }

        let text = RichText::new("Find Intersections").heading();
        ui.label(if s.typ.is_find_intersections() {
            text.underline()
        } else {
            text
        });

        let text = RichText::new("1. Initialize an empty event queue Q.");
        ui.label(if matches!(s.typ, crate::StepType::StartInitQ) {
            text.underline()
        } else {
            text
        });
        if let StepType::InitQ { segment } = s.typ {
            let seg = &segments[segment];

            let text = RichText::new(format!(
                "2. Insterting segment s{} with events (y,x,segment) ({:.2}, {:.2}, s{}) and ({:.2}, {:.2}, ()",
                seg.id, seg.upper.y, seg.upper.x, seg.id, seg.lower.y, seg.lower.x
            )).underline();
            ui.label(text);
        } else {
            ui.label("2. Insert segment endpoints into Q.");
        }

        let text = RichText::new("3. Initialize an empty status queue T.");
        ui.label(if matches!(s.typ, crate::StepType::InitT) {
            text.underline()
        } else {
            text
        });

        if s.typ == StepType::PopQ {
            let event = s.event.as_ref().unwrap();
            let y = event.y;
            let x = event.x;
            let seg = format_segment(event.segments.iter(), segments);

            let text = RichText::new(format!("4. while Q is not empty, pop the next event point. The next event is ({y}, {x}, ({seg}))")).underline();
            ui.label(text);
        } else {
            ui.label("4. while Q is not empty, pop the next event point");
        }
        ui.separator();

        let text = RichText::new("Handle Event Point").heading();
        ui.label(if s.typ.is_find_new_event() {
            text.underline()
        } else {
            text
        });
        let text = RichText::new("1. Calculate the Set U(p), C(p), L(p)");
        ui.label(if s.typ == StepType::CalculateSets {
            text.underline()
        } else {
            text
        });
        if let StepType::CalculateUpCpLp { up_cp_lp } = &s.typ {
            let seg = format_segment(up_cp_lp.iter(), segments);
            let text = RichText::new(format!("2. Calculate the set U(p) and C(p) and L(p): {seg}")).family(eframe::egui::FontFamily::Name("phosphor".into()))
                .underline();
            ui.label(text);
        } else {
            ui.label("2. Calculate the set U(p) and C(p) and L(p)");
        }
        if s.typ == StepType::ReportIntersections {
            let intersection = intersections[s.intersection.unwrap()].step.unwrap();
            ui.label(RichText::new(format!("4. If U(p) and C(p) and L(p) >= 2, report an intersection. Adding intersection {intersection}")).underline());
        } else {
            ui.label("4. If U(p) and C(p) and L(p) >= 2, report an intersection.");
        }
        let text = RichText::new("5. Delete C(p) and L(p) from the status queue");
        ui.label(if s.typ == StepType::DeleteLpCp {
            text.underline()
        } else {
            text
        });
        let text = RichText::new("6. Insert U(p) into the status queue");
        ui.label(if s.typ == StepType::InsertUpCp {
            text.underline()
        } else {
            text
        });
        ui.label("7. if U(p) and C(p) = empty");

        if let StepType::UpCpEmpty { s_l, s_r } = &s.typ {
            let s_l = format_segment(s_l.iter(), segments);
            let s_r = format_segment(s_r.iter(), segments);

            let text= RichText::new(format!("8. then Let s_l and s_r be the left and right neighbors of event p in our StatusQueue. s_l = ({s_l}), s_r = ({s_r})")).underline();
            ui.label(text);
            ui.label(RichText::new("9. FindNewEvent(s_l, s_r, p)").underline());
        } else {
            ui.label("8. then Let s_l and s_r be the left and right neighbors of event p in our StatusQueue.");
            ui.label("9. FindNewEvent(s_l, s_r, p)");
        }
        if let StepType::UpCpNotEmpty {
            s_r,
            s_dash,
            s_dash_dash,
            s_l,
        } = &s.typ
        {
            let s_l = format_segment(s_l.iter(), segments);
            let s_r = format_segment(s_r.iter(), segments);
            let s_dash = segments[*s_dash].id;
            let s_dash_dash = segments[*s_dash_dash].id;
            let text= RichText::new(format!("10. else Let s' be the leftmost segment of U(p) ∪ C(p) in the StatusQueue. s' = s{s_dash:?}")).underline();
            ui.label(text);
            let text = RichText::new(format!(
                "10. Let s_l be the left neighbor of s' in the StatusQueue. s_l = ({s_l})"
            ))
            .underline();
            ui.label(text);
            ui.label(RichText::new("11. FindNewEvent(s_l, s', p)").underline());
            let text= RichText::new(format!("12. Let s'' be the rightmost segment of U(p) ∪ C(p) in the StatusQueue. s'' = s{s_dash_dash:?}")).underline();
            ui.label(text);
            let text = RichText::new(format!(
                "12. Let s_r be the right neighbor of s'' in the StatusQueue. s_r = ({s_r})"
            ))
            .underline();
            ui.label(text);
            ui.label(RichText::new("12. FindNewEvent(s'', s_r, p)").underline());
        } else {
            ui.label("9. else Let s' be the leftmost segment of U(p) and C(p) in the StatusQueue.");
            ui.label("9. Let s_l be the left neighbor of s' in the StatusQueue.");
            ui.label("10. FindNewEvent(s_l, s', p)");
            ui.label("11. Let s'' be the rightmost segment of U(p) and C(p) in the StatusQueue.");
            ui.label("11. Let s_r be the right neighbor of s'' in the StatusQueue.");
            ui.label("12. FindNewEvent(s'', s_r, p)");
        }

        ui.separator();

        let text = RichText::new("Find New Event").heading();
        ui.label(if s.typ.is_find_new_event() {
            text.underline()
        } else {
            text
        });

        if let StepType::FindNewEvent { s_l, s_r } = &s.typ {
            let s_l = segments[*s_l].id;
            let s_r = segments[*s_r].id;
            let text = RichText::new(format!("1. if s_l (s{s_l}) and s_r (s{s_r}) intersect below the sweep line, or on it and to the right of the current event point p, and the intersection is not yet present as an event in the StatusQueue")).underline();
            ui.label(text);
        } else {
            ui.label("1. if s_l and s_r intersect below the sweep line, or on it and to the right of the current event point p, and the intersection is not yet present as an event in the StatusQueue");
        }

        if let StepType::InsertIntersectionEvent {
            s_l,
            s_r,
            intersection: (x, y),
        } = &s.typ
        {
            let x = x.0;
            let y = y.0;
            let s_l = segments[*s_l].id;
            let s_r = segments[*s_r].id;
            let text = RichText::new(format!("2. then insert the intersection point as an event into StatusQueue. Inserting ({y:.2}, {x:.2}, () as insection from s{s_l} and s{s_r}.")).underline();
            ui.label(text);
        } else {
            ui.label("2. then Insert the intersection point as an event in StatusQueue");
        }
    }
}

fn format_segment<'a>(a: impl Iterator<Item = &'a SegmentIdx>, segments: &Segments) -> String {
    use std::fmt::Write;
    let mut buf = String::new();
    let mut s = a;

    if let Some(s) = s.next() {
        let _ = write!(&mut buf, "s{}", segments[*s].id);
    }
    for s in s {
        let _ = write!(&mut buf, ", s{}", segments[*s].id);
    }

    buf
}
