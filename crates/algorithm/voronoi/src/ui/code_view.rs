use common::{
    AlgoStepIdx, AlgoSteps,
    math::B,
    ui::{MyWidget, WidgetName},
};
use eframe::egui::RichText;
use slotmap::Key;

use crate::{Circle, Step, StepType, beachline::SQKey, event_queue::Event};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CodeView;

impl WidgetName for CodeView {
    const NAME: &'static str = "Code";
    const NAME_LONG: &'static str = "Code Viewer";
}
#[derive(Debug, Clone)]
pub struct CodeViewState<'a, T: B> {
    pub step: AlgoStepIdx,
    pub steps: &'a AlgoSteps<Step<T>>,
}

impl<'a, T: B> MyWidget<CodeViewState<'a, T>> for CodeView {
    #[allow(clippy::too_many_lines)]
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: impl Into<CodeViewState<'a, T>>) {
        let CodeViewState { step, steps } = state.into();
        let s = &steps[step];

        if matches!(s.typ, StepType::Init) {
            ui.label(RichText::new("Not yet started").heading().underline());
        }
        if matches!(s.typ, StepType::Stopped) {
            ui.label(RichText::new("'Stopped'").heading().underline());
        }

        let text = "Initialize all data structures. Insert the first event into the Beachline";
        if matches!(s.typ, StepType::InitQ) {
            ui.label(RichText::new(text).heading().underline());
        } else {
            ui.label(RichText::new(text));
        }

        if matches!(s.typ, StepType::PopQ) {
            ui.label(
                RichText::new(format!(
                    "Get the next event. It is: {}",
                    s.event
                        .unwrap_or_else(|| Event::Circle(SQKey::null(), Circle::new(0, 0, 0)))
                ))
                .heading()
                .underline(),
            );
        } else {
            ui.label(RichText::new("Get the next event.").heading().underline());
        }

        ui.separator();
        let text = "Handling Site Event";
        if s.typ.is_site_event() {
            ui.label(RichText::new(text).heading().underline());
        } else {
            ui.label(RichText::new(text));
        }

        if let StepType::SearchT(key) = s.typ {
            ui.label(
                RichText::new(format!("Find arc to insert. Found {key:?}"))
                    .heading()
                    .underline(),
            );
        } else {
            ui.label(RichText::new("Find arc to insert").heading().underline());
        }

        if let StepType::DeleteVertexEvent(key) = s.typ {
            if let Some(event) = key {
                ui.label(
                    RichText::new(format!(
                        "Remove event associated with the arc. Removed {event}"
                    ))
                    .heading()
                    .underline(),
                );
            } else {
                ui.label(
                    RichText::new(
                        "Remove event associated with the arc. No event associated".to_string(),
                    )
                    .heading()
                    .underline(),
                );
            }
        } else {
            ui.label(
                RichText::new("Remove event associated with the arc.")
                    .heading()
                    .underline(),
            );
        }

        ui.separator();
        let text = "Insert new Arc";
        if s.typ.is_replacing() {
            ui.label(RichText::new(text).heading().underline());
        } else {
            ui.label(RichText::new(text));
        }
        let text = "New Arc is left from the Beachline. We insert the Site before the first leaf";
        if matches!(s.typ, StepType::ReplaceLeft) {
            ui.label(RichText::new(text).heading().underline());
        } else {
            ui.label(RichText::new(text));
        }

        let text = "New Arc is right from the Beachline. We insert the Site after the last leaf";
        if matches!(s.typ, StepType::ReplaceLeft) {
            ui.label(RichText::new(text).heading().underline());
        } else {
            ui.label(RichText::new(text));
        }

        let text = "We are breaking an existing arc. The found node is left. We insert a node with the new site after it and call this node middle. We then insert the site of left after the middle node.";
        if matches!(s.typ, StepType::ReplaceLeft) {
            ui.label(RichText::new(text).heading().underline());
        } else {
            ui.label(RichText::new(text));
        }

        if let StepType::CreateHalfEdges { e1, e2 } = s.typ {
            ui.label(
                RichText::new(format!("Insert new Half Edges between {e1} and {e2}"))
                    .heading()
                    .underline(),
            );
        } else {
            ui.label(RichText::new(
                "Insert new Half Edges between the old site and new site",
            ));
        }

        if let StepType::CreateVertexEvent(k, e) = s.typ {
            if let Some(event) = e {
                ui.label(
                    RichText::new(format!("Check {k:?} for a new circle event. Found {event}"))
                        .heading()
                        .underline(),
                );
            } else {
                ui.label(
                    RichText::new(format!("Check {k:?} for a new circle event. Found none"))
                        .heading()
                        .underline(),
                );
            }
        } else {
            ui.label(RichText::new(
                "Check the node before and after insertion for a new cycle event.",
            ));
        }

        ui.separator();
        let text = "Handling Circle Event";
        if s.typ.is_circle_event() {
            ui.label(RichText::new(text).heading().underline());
        } else {
            ui.label(RichText::new(text));
        }
        if s.typ.is_circle_event()
            && let Some(event @ Event::Circle(k, _)) = s.event
        {
            ui.label(format!("Handling Circle: {event} at {k:?}"));
        }

        let text = "Delete the leaf that disappears from the Beachline.";
        if matches!(s.typ, StepType::ReplaceLeft) {
            ui.label(RichText::new(text).heading().underline());
        } else {
            ui.label(RichText::new(text));
        }

        if let StepType::DeleteVertexEvent2(key, event) = s.typ {
            if let Some(event) = event {
                ui.label(
                    RichText::new(format!(
                        "Remove possible circle event involving the arc. Checking: {key:?}. Removed {event}"
                    ))
                    .heading()
                    .underline(),
                );
            } else {
                ui.label(
                    RichText::new(
                        format!("Remove possible circle event involving the arc. Checking: {key:?}. No event associated"),
                    )
                    .heading()
                    .underline(),
                );
            }
        } else {
            ui.label(
                RichText::new("Remove possible circle event involving the arc.")
                    .heading()
                    .underline(),
            );
        }

        if let StepType::AddVertexToEdges(v) = s.typ {
            ui.label(
                RichText::new(format!("Add Vertex to the DCEL. Added Vertex {v}"))
                    .heading()
                    .underline(),
            );
        } else {
            ui.label(
                RichText::new("Add Vertex to the DCEL.")
                    .heading()
                    .underline(),
            );
        }

        if let StepType::CreateVertexCircle(k, e) = s.typ {
            if let Some(event) = e {
                ui.label(
                    RichText::new(format!("Check {k:?} for a new circle event. Found {event}"))
                        .heading()
                        .underline(),
                );
            } else {
                ui.label(
                    RichText::new(format!("Check {k:?} for a new circle event. Found none"))
                        .heading()
                        .underline(),
                );
            }
        } else {
            ui.label(RichText::new(
                "Check the node before and after the removed one for a new cycle event.",
            ));
        }

        ui.separator();
        let text = "Bound Diagram";
        if s.typ.is_bound() {
            ui.label(RichText::new(text).heading().underline());
        } else {
            ui.label(RichText::new(text));
        }

        if let StepType::FullInfinite(hl, hr, l, r, v1, v2) = s.typ {
            ui.label(
                RichText::new("For all halfedges, that are on neither site bounded:")
                    .heading()
                    .underline(),
            );
            ui.label(
                RichText::new(format!("Inspecting half edges {hl} and {hr}"))
                    .heading()
                    .underline(),
            );
            ui.label(
                RichText::new(format!("Calculate the Bisector between {l} and {r}"))
                    .heading()
                    .underline(),
            );
            ui.label(
                RichText::new(format!("Intersect the Bisector with the boundary segments. Found new vertices Vertex {v1} and Vertex {v2}"))
                    .heading()
                    .underline(),
            );
            ui.label(RichText::new("Add Vertices to DCEL").heading().underline());
        } else {
            ui.label("Calculate the Bisector between left site and right site");
            ui.label("Intersect the Bisector with the boundary segments");
            ui.label("Add Vertices to DCEL");
        }

        if let StepType::HalfInfinite(hl, hr, l, r, _, v1, v2) = s.typ {
            ui.label(
                RichText::new("For all halfedges, that are bounded on site:")
                    .heading()
                    .underline(),
            );
            ui.label(
                RichText::new(format!(
                    "Inspecting half edges {hl} and {hr} with Vertex {v1}"
                ))
                .heading()
                .underline(),
            );
            ui.label(
                RichText::new(format!("Calculate the Bisector between {l} and {r}"))
                    .heading()
                    .underline(),
            );
            ui.label(
                RichText::new(format!("Determine the direction, to {l} and {r}. The direction is seperated in UpLeft, DownLeft, DownRight and UpRight."))
                    .heading()
                    .underline(),
            );
            ui.label(
                RichText::new(format!("Check if no segment exists, that lies between {l} and {r}. If it exists Reverse the direction."))
                    .heading()
                    .underline(),
            );
            ui.label(
                RichText::new("Intersect the Bisector with the boundary segments that lie in the calculated direction.")
                    .heading()
                    .underline(),
            );
            ui.label(
                RichText::new(format!("Add Vertex {v2} to DCEL"))
                    .heading()
                    .underline(),
            );
        } else {
            ui.label("Calculate the Bisector between left site and right site");
            ui.label("Intersect the Bisector with the boundary segments");
            ui.label("Add Vertices to DCEL");
        }
    }
}
