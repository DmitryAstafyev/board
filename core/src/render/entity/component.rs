use crate::{
    entity::{Component, Ports, Signature, SignatureGetter},
    error::E,
    render::{
        elements, form::GridRectangle, grid::ElementType, options::Options, Container, Form,
        Relative, Render, Representation, View,
    },
    state::State,
};
use wasm_bindgen::JsValue;

const MIN_HEIGHT: i32 = 64;
const MIN_WIDTH: i32 = 64;

impl<'a, 'b: 'a> SignatureGetter<'a, 'b> for Render<Component> {
    fn sig(&'b self) -> &'a Signature {
        &self.origin().sig
    }
}

impl Render<Component> {
    pub fn new(mut entity: Component, options: &Options, mut ty: Option<ElementType>) -> Self {
        entity.ports = if let Representation::Origin(ports) = entity.ports {
            Representation::Render(Render::<Ports>::new(
                ports,
                options,
                ty.as_ref()
                    .map(|ty| matches!(ty, ElementType::Composition))
                    .unwrap_or(false),
            ))
        } else {
            entity.ports
        };
        let id = entity.sig.id;
        let composition = entity.composition;
        Self {
            entity,
            view: View {
                container: Container {
                    form: Form::GridRectangle(
                        if let Some(ty) = ty.take() {
                            ty
                        } else {
                            ElementType::Component
                        },
                        GridRectangle::new(
                            id.to_string(),
                            0,
                            0,
                            options.ratio().get(MIN_WIDTH),
                            options.ratio().get(MIN_HEIGHT),
                            &options.ratio(),
                        ),
                    ),
                    style: if composition {
                        (&options.scheme.composition_as_component_rect).into()
                    } else {
                        (&options.scheme.component_rect).into()
                    },
                },
                elements: Vec::new(),
            },
            hidden: false,
        }
    }

    pub fn is_composition(&self) -> bool {
        matches!(
            self.view.container.form.get_el_ty(),
            ElementType::Composition
        )
    }

    pub fn calc(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
        options: &Options,
        state: &State,
        root: usize,
    ) -> Result<(), E> {
        // Calc ports
        let self_relative = self.relative(relative);
        self.entity.ports.render_mut()?.calc(
            context,
            self.view.container.get_box_size().0,
            &self_relative,
            options,
            state,
            root,
        )?;
        let min_height = options.ratio().get(MIN_HEIGHT);
        // Set self size
        self.view.container.set_box_size(
            None,
            Some(elements::max(
                &[
                    min_height,
                    self.entity.ports.render_mut()?.height(state, options),
                ],
                min_height,
            )),
        );
        Ok(())
    }

    pub fn draw(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
        options: &Options,
        state: &State,
        root: usize,
    ) -> Result<(), E> {
        if state.is_hovered(&self.entity.sig.id) {
            self.view.container.style = (&options.scheme.hovered_rect).into();
        } else if matches!(
            self.view.container.form.get_el_ty(),
            ElementType::Composition
        ) {
            self.view.container.style = (&options.scheme.composition_as_component_rect).into();
        } else if state.is_component_selected(&self.entity.sig.id) {
            self.view.container.style = (&options.scheme.selected_rect).into();
        } else {
            self.view.container.style = (&options.scheme.component_rect).into();
        }
        if state.is_match(&self.entity.sig.id) {
            self.view.container.style = (&options.scheme.matched_rect).into();
        }
        if state.is_highlighted(&self.entity.sig.id) {
            self.view.container.style = (&options.scheme.highlighted_rect).into();
        }
        self.view.render(context, relative, options);
        let self_relative = self.relative(relative);
        let ratio = options.ratio();
        self.entity
            .ports
            .render_mut()?
            .draw(context, &self_relative, options, state, root)?;
        context.set_text_baseline("bottom");
        context.set_stroke_style(&JsValue::from_str(&options.scheme.composition_label.stroke));
        context.set_font(&format!(
            "{}px {}",
            ratio.get(relative.zoom(12)),
            options.font
        ));
        context.set_fill_style(&JsValue::from_str(&options.scheme.composition_label.fill));
        let _ = context.fill_text(
            &self.origin().get_label(options),
            relative.x(self.view.container.get_coors().0) as f64,
            relative.y(self.view.container.get_coors().1 - ratio.get(3)) as f64,
        );
        Ok(())
    }
}
