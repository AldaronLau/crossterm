//! This module contains the `object style` that can be applied to an `styled object`.

use super::{ScreenManager, Color, StyledObject};

use std::fmt::Display;

#[cfg(unix)]
use super::Attribute;

/// Struct that contains the style properties that can be applied to an displayable object.
#[derive(Clone)]
pub struct ObjectStyle {
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,

    #[cfg(unix)]
    pub attrs: Vec<Attribute>,
}

impl Default for ObjectStyle {
    fn default() -> ObjectStyle {
        ObjectStyle {
            fg_color: Some(Color::White),
            bg_color: Some(Color::Black),
            #[cfg(unix)]
            attrs: Vec::new(),
        }
    }
}

impl ObjectStyle {
    /// Apply an `StyledObject` to the passed displayable object.
    pub fn apply_to<'style, D: Display>(&self, val: D, screen_manager: &'style ScreenManager) -> StyledObject<'style,D> {
        StyledObject {
            object_style: self.clone(),
            screen_manager: screen_manager,
            content: val,
        }
    }

    /// Get an new instance of `ObjectStyle`
    pub fn new() -> ObjectStyle {
        return ObjectStyle {
            fg_color: None,
            bg_color: None,
            #[cfg(unix)]
            attrs: Vec::new(),
        };
    }

    /// Set the background color of `ObjectStyle` to the passed color.
    pub fn bg(mut self, color: Color) -> ObjectStyle {
        self.bg_color = Some(color);
        self
    }

    /// Set the foreground color of `ObjectStyle` to the passed color.
    pub fn fg(mut self, color: Color) -> ObjectStyle {
        self.fg_color = Some(color);
        self
    }

    #[cfg(unix)]
    pub fn add_attr(&mut self, attr: Attribute) {
        self.attrs.push(attr);
    }
}
