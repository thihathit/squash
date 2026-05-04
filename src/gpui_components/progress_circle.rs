// Standalone ProgressCircle component extracted from gpui-component
//
// External dependencies needed (add to your Cargo.toml):
// - gpui (with gpui_macros feature for IntoElement derive)
// - instant
//
// Usage: Copy this file to your project and import the types you need.

use gpui::prelude::FluentBuilder as _;
use gpui::{
    Animation, AnimationExt as _, AnyElement, App, ElementId, Hsla, InteractiveElement as _,
    IntoElement, ParentElement, Pixels, RenderOnce, StyleRefinement, Styled, Window, canvas,
    ease_in_out, px, relative,
};
use gpui::{Bounds, Path, PathBuilder, div, point};
use instant::Duration;
use std::f32::consts::TAU;

// ============================================================================
// Size enum - defines the size variants for UI elements
// ============================================================================

#[derive(Clone, Default, Copy, PartialEq, Eq, Debug)]
pub enum Size {
    Size(Pixels),
    XSmall,
    Small,
    #[default]
    Medium,
    Large,
}

impl From<Pixels> for Size {
    fn from(size: Pixels) -> Self {
        Size::Size(size)
    }
}

// ============================================================================
// Sizable trait - allows setting the size of an element
// ============================================================================

pub trait Sizable: Sized {
    fn with_size(mut self, size: impl Into<Size>) -> Self;

    fn xsmall(self) -> Self {
        self.with_size(Size::XSmall)
    }

    fn small(self) -> Self {
        self.with_size(Size::Small)
    }

    fn large(self) -> Self {
        self.with_size(Size::Large)
    }
}

// ============================================================================
// StyledExt trait (minimal version) - extends gpui::Styled with additional methods
// ============================================================================

pub trait StyledExt: Styled + Sized {
    fn refine_style(mut self, style: &StyleRefinement) -> Self {
        self.style().refine(style);
        self
    }
}

impl<E: Styled> StyledExt for E {}

// ============================================================================
// ProgressState - shared state for progress components with animation support
// ============================================================================

use std::cell::Cell;

pub struct ProgressState {
    pub(crate) value: f32,
    target: Cell<f32>,
}

impl ProgressState {
    pub(crate) fn new(value: f32) -> Self {
        Self {
            value,
            target: Cell::new(value),
        }
    }

    pub(crate) fn target(&self) -> f32 {
        self.target.get()
    }

    pub(crate) fn set_target(&self, value: f32) {
        self.target.set(value);
    }
}

// ============================================================================
// Arc and ArcData - for rendering circular/progress arcs
// Reference: https://d3js.org/d3-shape/arc
// ============================================================================

const EPSILON: f32 = 1e-12;
const HALF_PI: f32 = std::f32::consts::PI / 2.;

pub struct ArcData<'a, T> {
    pub data: &'a T,
    pub index: usize,
    pub value: f32,
    pub start_angle: f32,
    pub end_angle: f32,
    pub pad_angle: f32,
}

impl<T> std::fmt::Debug for ArcData<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ArcData {{ index: {}, value: {}, start_angle: {}, end_angle: {}, pad_angle: {} }}",
            self.index, self.value, self.start_angle, self.end_angle, self.pad_angle
        )
    }
}

pub struct Arc {
    inner_radius: f32,
    outer_radius: f32,
}

impl Default for Arc {
    fn default() -> Self {
        Self {
            inner_radius: 0.,
            outer_radius: 0.,
        }
    }
}

impl Arc {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inner_radius(mut self, inner_radius: f32) -> Self {
        self.inner_radius = inner_radius;
        self
    }

    pub fn outer_radius(mut self, outer_radius: f32) -> Self {
        self.outer_radius = outer_radius;
        self
    }

    pub fn centroid<T>(&self, arc: &ArcData<T>) -> gpui::Point<f32> {
        let start_angle = arc.start_angle - HALF_PI;
        let end_angle = arc.end_angle - HALF_PI;
        let r = (self.inner_radius + self.outer_radius) / 2.;
        let a = (start_angle + end_angle) / 2.;

        point(r * a.cos(), r * a.sin())
    }

    fn path<T>(
        &self,
        arc: &ArcData<T>,
        inner_radius: Option<f32>,
        outer_radius: Option<f32>,
        bounds: &Bounds<Pixels>,
    ) -> Option<Path<Pixels>> {
        let start_angle = arc.start_angle - HALF_PI;
        let end_angle = arc.end_angle - HALF_PI;
        let da = end_angle - start_angle;
        let pad_angle = if da >= std::f32::consts::PI {
            // Leave some pad angle for full circle.
            // If not, the path start and end will be the same point.
            0.0001
        } else {
            arc.pad_angle
        };
        let r0 = inner_radius.unwrap_or(self.inner_radius).max(0.);
        let r1 = outer_radius.unwrap_or(self.outer_radius).max(0.);

        // Calculate the center point.
        let center_x = bounds.origin.x.as_f32() + bounds.size.width.as_f32() / 2.;
        let center_y = bounds.origin.y.as_f32() + bounds.size.height.as_f32() / 2.;

        // Angle difference.
        if r1 < EPSILON || da.abs() < EPSILON {
            return None;
        }

        // Handle pad angle.
        let (a0_outer, a1_outer, a0_inner, a1_inner) = if r0 > EPSILON && pad_angle > 0.0 {
            let pad_width = r1 * pad_angle;
            let pad_angle_outer = pad_width / r1;
            let mut pad_angle_inner = pad_width / r0;
            let max_inner_pad = da * 0.8;
            if pad_angle_inner > max_inner_pad {
                pad_angle_inner = max_inner_pad;
            }
            (
                start_angle + pad_angle_outer * 0.5,
                end_angle - pad_angle_outer * 0.5,
                start_angle + pad_angle_inner * 0.5,
                end_angle - pad_angle_inner * 0.5,
            )
        } else {
            let pad = pad_angle * 0.5;
            (
                start_angle + pad,
                end_angle - pad,
                start_angle + pad,
                end_angle - pad,
            )
        };

        let da_outer = a1_outer - a0_outer;
        if da_outer <= 0. {
            return None;
        }

        // Calculate the start and end points of the outer arc.
        let x01 = center_x + r1 * a0_outer.cos();
        let y01 = center_y + r1 * a0_outer.sin();
        let x11 = center_x + r1 * a1_outer.cos();
        let y11 = center_y + r1 * a1_outer.sin();

        let mut builder = PathBuilder::fill();

        // Move to the start point of the outer arc.
        builder.move_to(point(px(x01), px(y01)));

        // Draw the outer arc.
        let large_arc = (a1_outer - a0_outer).abs() > std::f32::consts::PI;
        builder.arc_to(
            point(px(r1), px(r1)),
            px(0.),
            large_arc,
            true,
            point(px(x11), px(y11)),
        );

        if r0 > EPSILON {
            // End point of the inner arc.
            let x10 = center_x + r0 * a1_inner.cos();
            let y10 = center_y + r0 * a1_inner.sin();
            builder.line_to(point(px(x10), px(y10)));

            // Draw the inner arc.
            let x00 = center_x + r0 * a0_inner.cos();
            let y00 = center_y + r0 * a0_inner.sin();
            let large_arc_inner = (a1_inner - a0_inner).abs() > std::f32::consts::PI;
            builder.arc_to(
                point(px(r0), px(r0)),
                px(0.),
                large_arc_inner,
                false,
                point(px(x00), px(y00)),
            );
        } else {
            // If there is no inner radius, draw a line to the center.
            builder.line_to(point(px(center_x), px(center_y)));
        }

        builder.build().ok()
    }

    pub fn paint<T>(
        &self,
        arc: &ArcData<T>,
        color: impl Into<Hsla>,
        inner_radius: Option<f32>,
        outer_radius: Option<f32>,
        bounds: &Bounds<Pixels>,
        window: &mut Window,
    ) {
        let path = self.path(arc, inner_radius, outer_radius, bounds);
        if let Some(path) = path {
            window.paint_path(path, color.into());
        }
    }
}

// ============================================================================
// ProgressCircle - the main circular progress indicator component
// ============================================================================

/// Default color for the progress circle when no color is specified.
/// This is a neutral blue color similar to common UI frameworks.
pub const DEFAULT_PROGRESS_COLOR: Hsla = Hsla {
    h: 210.0 / 360.0, // Blue hue
    s: 0.8,           // High saturation
    l: 0.5,           // Medium lightness
    a: 1.0,           // Full opacity
};

/// A circular progress indicator element.
#[derive(IntoElement)]
pub struct ProgressCircle {
    id: ElementId,
    style: StyleRefinement,
    color: Option<Hsla>,
    value: f32,
    size: Size,
    children: Vec<AnyElement>,
    loading: bool,
}

impl ProgressCircle {
    /// Create a new circular progress indicator.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: Default::default(),
            color: None,
            style: StyleRefinement::default(),
            size: Size::default(),
            children: Vec::new(),
            loading: false,
        }
    }

    /// Enable indeterminate loading animation.
    ///
    /// When `loading` is `true`, the `value` is ignored and an infinite
    /// rotating arc animation is shown instead.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Set the color of the progress circle.
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Set the percentage value of the progress circle.
    ///
    /// The value should be between 0.0 and 100.0.
    pub fn value(mut self, value: f32) -> Self {
        self.value = value.clamp(0., 100.);
        self
    }

    /// Render the arc canvas. `start_value` and `end_value` are in 0.0–100.0 percentage.
    /// The progress arc is skipped when `end_value <= 0`.
    fn render_circle(start_value: f32, end_value: f32, color: Hsla) -> impl IntoElement {
        struct PrepaintState {
            start_value: f32,
            end_value: f32,
            actual_inner_radius: f32,
            actual_outer_radius: f32,
            bounds: Bounds<Pixels>,
        }

        canvas(
            move |bounds: Bounds<Pixels>, _window: &mut Window, _cx: &mut App| {
                let stroke_width = (bounds.size.width * 0.15).min(px(5.));
                let actual_size = bounds.size.width.min(bounds.size.height);
                let actual_radius = (actual_size.as_f32() - stroke_width.as_f32()) / 2.;
                PrepaintState {
                    start_value,
                    end_value,
                    actual_inner_radius: actual_radius - stroke_width.as_f32() / 2.,
                    actual_outer_radius: actual_radius + stroke_width.as_f32() / 2.,
                    bounds,
                }
            },
            move |_bounds, prepaint, window: &mut Window, _cx: &mut App| {
                let arc = Arc::new()
                    .inner_radius(prepaint.actual_inner_radius)
                    .outer_radius(prepaint.actual_outer_radius);

                arc.paint(
                    &ArcData {
                        data: &(),
                        index: 0,
                        value: 100.,
                        start_angle: 0.,
                        end_angle: TAU,
                        pad_angle: 0.,
                    },
                    color.opacity(0.2),
                    None,
                    None,
                    &prepaint.bounds,
                    window,
                );

                if prepaint.end_value > 0. {
                    let start_angle = (prepaint.start_value / 100.) * TAU;
                    let end_angle = (prepaint.end_value / 100.) * TAU;
                    arc.paint(
                        &ArcData {
                            data: &(),
                            index: 1,
                            value: prepaint.end_value,
                            start_angle,
                            end_angle,
                            pad_angle: 0.,
                        },
                        color,
                        None,
                        None,
                        &prepaint.bounds,
                        window,
                    );
                }
            },
        )
        .absolute()
        .size_full()
    }
}

impl Styled for ProgressCircle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Sizable for ProgressCircle {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}

impl ParentElement for ProgressCircle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ProgressCircle {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = self.value;
        let loading = self.loading;
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| ProgressState::new(value));
        let prev_target = state.read(cx).target();
        let has_changed = prev_target != value;

        // Use default color or the color specified by the user
        let color = self.color.unwrap_or(DEFAULT_PROGRESS_COLOR);

        div()
            .id(self.id.clone())
            .flex()
            .items_center()
            .justify_center()
            .line_height(relative(1.))
            .map(|this| match self.size {
                Size::XSmall => this.size_2(),
                Size::Small => this.size_3(),
                Size::Medium => this.size_4(),
                Size::Large => this.size_5(),
                Size::Size(s) => this.size(s * 0.75),
            })
            .refine_style(&self.style)
            .children(self.children)
            .map(|this| {
                if has_changed {
                    let from = prev_target;
                    state.read(cx).set_target(value);

                    let duration = Duration::from_secs_f64(0.15);
                    cx.spawn({
                        let state = state.clone();
                        async move |cx| {
                            cx.background_executor().timer(duration).await;
                            _ = state.update(cx, |this, _| {
                                this.value = this.target();
                            });
                        }
                    })
                    .detach();

                    this.with_animation(
                        format!("progress-circle-{}", from),
                        Animation::new(duration),
                        move |this, delta| {
                            let v = from + (value - from) * delta;
                            this.child(Self::render_circle(0., v, color))
                        },
                    )
                    .into_any_element()
                } else if loading {
                    this.with_animation(
                        "progress-circle-loading",
                        Animation::new(Duration::from_secs(1)).repeat(),
                        move |this, delta| {
                            let end = ease_in_out(delta) * 100.;
                            let start = ease_in_out(((delta - 0.5) / 0.5).clamp(0., 1.)) * 100.;
                            this.child(Self::render_circle(start, end, color))
                        },
                    )
                    .into_any_element()
                } else {
                    this.child(Self::render_circle(0., value, color))
                        .into_any_element()
                }
            })
    }
}

// ============================================================================
// Example usage (commented out):
// ============================================================================
//
// use gpui::{App, Window, div, px};
//
// fn example_usage(window: &mut Window, cx: &mut App) {
//     // Basic usage with default color
//     let progress = ProgressCircle::new("my-progress")
//         .value(75.0)
//         .size(px(100.));
//
//     // With custom color
//     let progress2 = ProgressCircle::new("my-progress-2")
//         .value(50.0)
//         .color(gpui::Hsla::new(0.0, 0.8, 0.5, 1.0)) // Red color
//         .small();
//
//     // Loading animation
//     let loading = ProgressCircle::new("loading")
//         .loading(true)
//         .large();
// }
