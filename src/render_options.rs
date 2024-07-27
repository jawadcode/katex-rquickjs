use rquickjs::Object;
use rquickjs::{Ctx, IntoJs, Value};
use std::collections::HashMap;

use crate::QJSResult;

pub struct RenderOptions {
    /// If `true`, then math[s] will be rendered in display mode, else it will be rendered in inline mode
    pub display_mode: bool,
    /// The markup language of the output
    pub output: OutputType,
    /// Render `\tag` on the left instead of the right (as in `\usepackage[leqno]{amsmath}`)
    pub leqno: bool,
    /// Whether display math[s] should render flush left (as in `\documentclass[fleqn]`)
    pub fleqn: bool,
    /// Whether to throw a `ParseError` exception upon encountering an unsupported command or invalid LaTeX, if this is `false` then KaTeX will render unsupported commands as text, and render invalid LaTeX as its source code with hover text giving the error, in the color given by [[RenderOptions::error_color]].
    throw_on_error: bool,
    pub error_color: String,
    /// A map of macro names to their expansions.
    ///
    /// # Note
    ///
    /// KaTeX does not just accept `String` values in the `macros` object, the other possibilities are:
    ///
    ///  * Functions taking [`MacroExpander`](https://github.com/KaTeX/KaTeX/blob/main/src/MacroExpander.js#L29) as the first argument, returning a string expansion
    ///  * An expansion object matching the [`MacroExpansion`](https://github.com/KaTeX/KaTeX/blob/main/src/defineMacro.js) specification
    ///
    /// However, the `MacroExpander` class, all its properties & methods, and all types exposed in its interface would need a Rust interface, and the `MacroExpansion` specification would also need a corresponding Rust construct (perhaps a trait). This is all work I am currently uninterested in taking on.
    pub macros: HashMap<String, String>,
    pub min_rule_thickness: Option<usize>,
    pub color_is_text_color: bool,
    pub strict: Strictness,
    /// Whether to allow commands that could enable adverse behaviour, such as `\includegraphics` or `\href`.
    ///
    /// # Note
    ///
    /// KaTeX also allows a handler function for this option, however this library currently only supports a boolean.
    pub trust: bool,
    pub max_size: f64,
    pub max_expand: f64,
    pub global_group: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            display_mode: false,
            output: OutputType::HTMLAndMathML,
            leqno: false,
            fleqn: false,
            throw_on_error: false,
            error_color: "#cc0000".to_string(),
            macros: HashMap::new(),
            min_rule_thickness: None,
            color_is_text_color: false,
            strict: Strictness::Warn,
            trust: true,
            max_size: f64::INFINITY,
            max_expand: 1000.0,
            global_group: false,
        }
    }
}

impl<'js> IntoJs<'js> for RenderOptions {
    #[rustfmt::skip]
    fn into_js(self, ctx: &Ctx<'js>) -> QJSResult<Value<'js>> {
        let obj = Object::new(ctx.clone())?;
        obj.set("displayMode", self.display_mode.into_js(ctx)?)?;
        obj.set("output", self.output.into_js(ctx)?)?;
        obj.set("leqno", self.leqno.into_js(ctx)?)?;
        obj.set("fleqn", self.fleqn.into_js(ctx)?)?;
        obj.set("throwOnError", self.throw_on_error.into_js(ctx)?)?;
        obj.set("errorColor", self.error_color.into_js(ctx)?)?;
        obj.set("macros", self.macros.into_js(ctx)?)?;
        obj.set("minRuleThickness", self.min_rule_thickness.into_js(ctx)?)?;
        obj.set("colorIsTextColor", self.color_is_text_color.into_js(ctx)?)?;
        obj.set("strict", self.strict.into_js(ctx)?)?;
        obj.set("trust", self.trust.into_js(ctx)?)?;
        obj.set("maxSize", self.max_size.into_js(ctx)?)?;
        obj.set("maxExpand", self.max_expand.into_js(ctx)?)?;
        obj.set("globalGroup", self.global_group.into_js(ctx)?)?;
        Ok(Value::from_object(obj))
    }
}

/// The markup language of the rendered output
pub enum OutputType {
    /// Uses both HTML for visual rendering and MathML for accessibility (the default).
    HTMLAndMathML,
    /// Use exclusively HTML
    HTML,
    /// Use exclusively MathML
    MathML,
}

impl<'js> IntoJs<'js> for OutputType {
    fn into_js(self, ctx: &Ctx<'js>) -> QJSResult<Value<'js>> {
        match self {
            Self::HTMLAndMathML => "htmlAndMathml",
            Self::HTML => "html",
            Self::MathML => "mathml",
        }
        .into_js(ctx)
    }
}

/// Strictness with regards to the usage of non-standard LaTeX convenience features provided by KaTeX.
///
/// # Note
///
/// Custom handler functions are not included as I have not found an elegant way to represent/include them.
pub enum Strictness {
    /// Warn about such behaviour through `console.warn()`, currently this has no effect as neither QuickJS nor this library provide a `console` object to KaTeX.
    Warn,
    /// Allow convenient but non-standard LaTeX features (subsumes `false`)
    Ignore,
    /// Throw an error when non-standard LaTeX features are used (subsumes `true`)
    Error,
}

impl<'js> IntoJs<'js> for Strictness {
    fn into_js(self, ctx: &Ctx<'js>) -> QJSResult<Value<'js>> {
        match self {
            Strictness::Warn => "warn",
            Strictness::Ignore => "ignore",
            Strictness::Error => "error",
        }
        .into_js(ctx)
    }
}

pub enum ErrorCode {
    UnknownSymbol,
    UnicodeTextInMathMode,
    MathVsTextUnits,
    CommentAtEnd,
    HtmlExtension,
    NewLineInDisplayMode,
}

impl<'js> IntoJs<'js> for ErrorCode {
    fn into_js(self, ctx: &Ctx<'js>) -> QJSResult<Value<'js>> {
        match self {
            ErrorCode::UnknownSymbol => "unknownSymbol",
            ErrorCode::UnicodeTextInMathMode => "unicodeTextInMathMode",
            ErrorCode::MathVsTextUnits => "mathVsTextUnits",
            ErrorCode::CommentAtEnd => "commentAtEnd",
            ErrorCode::HtmlExtension => "htmlExtension",
            ErrorCode::NewLineInDisplayMode => "newLineInDisplayMode",
        }
        .into_js(ctx)
    }
}
