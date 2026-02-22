use std::cmp::PartialEq;
use std::collections::HashMap;
use crate::rwl::ast::parser::{BlockType};
use crate::rwl::value::*;
use crate::rwl::error::*;
use crate::shared::area::*;
use crate::shared::color::*;
use crate::shared::graphics::*;
use crate::shared::graphics_utils::Rounding;
use crate::shared::theme::Theme;
use crate::shared::vec::Vec2;

const DEBUG: bool = false;

pub type UpdateCtx<'a, 'b> = (&'a mut GLDrawHandle<'b>, &'b Theme);
type Children = Vec<NodeWrapper>;

pub enum FrameDirection {
    Horizontal,
    Vertical
}
pub struct FrameData {
    dir: FrameDirection,
    flipped: bool
}

// data passed down to the children of a container
#[derive(Debug)]
#[derive(Clone)]
pub struct ContainerContext {
    anchor_x: AnchorX,
    anchor_y: AnchorY,
    pos: Option<Vec2>,
    color: Color
}

impl ContainerContext {
    pub fn new() -> ContainerContext {
        ContainerContext {
            anchor_x: AnchorX::Center,
            anchor_y: AnchorY::Center,
            pos: None,
            color: Color { r: 255, g: 255, b: 255, a: 255 }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Alignment {
    Left,
    Center,
    Right
}
#[derive(Debug, Copy, Clone)]
pub enum AnchorX {
    Left,
    Center,
    Right
}
#[derive(Debug, Copy, Clone)]
pub enum AnchorY {
    Top,
    Center,
    Bottom
}
pub type Anchor = (AnchorX, AnchorY);

impl From<Alignment> for AnchorX {
    fn from(value: Alignment) -> Self {
        match value {
            Alignment::Left   => AnchorX::Left,
            Alignment::Center => AnchorX::Center,
            Alignment::Right  => AnchorX::Right
        }
    }
}
impl From<AnchorX> for Alignment {
    fn from(value: AnchorX) -> Self {
        match value {
            AnchorX::Left   => Alignment::Left,
            AnchorX::Center => Alignment::Center,
            AnchorX::Right  => Alignment::Right
        }
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    pub pairs: HashMap<String, Value>,
    pub flags: Vec<String>
}

impl Header {
    pub fn new() -> Header {
        Header {
            pairs: HashMap::new(),
            flags: Vec::new()
        }
    }
    
    pub fn get(&self, name: &str) -> Option<&Value> {
        if self.pairs.contains_key(&String::from(name)) {
            return self.pairs.get(&String::from(name));
        }
        
        None
    }
    pub fn expect(&self, name: &str, type_name: &str) -> Result<Option<&Value>, Error> {
        if self.pairs.contains_key(&String::from(name)) {
            let val = self.get(name).unwrap();
            if val.get_type() != type_name {
                return Err(Error::ValueTypeMismatch(
                    ErrPosition,
                    String::from(type_name),
                    String::from(val.get_type())
                ))
            }
            return Ok(Some(val))
        }
        
        Ok(None)
    }
}

impl Header {
    pub fn has_flag(&self, name: &str) -> bool {
        self.flags.contains(&String::from(name))
    }
    pub fn set_flag(&mut self, name: &str, value: bool) {
        let has = self.has_flag(name);
        if !has && value {
            self.flags.push(String::from(name));
        } else if has && !value {
            let comp_val = String::from(name);
            self.flags.retain(|f| *f != comp_val);
        }
    }
    
    fn get_area_keys(&self, name: &str) -> Area {
        let mut left = 0.0;
        let mut right = 0.0;
        let mut bottom = 0.0;
        let mut top = 0.0;
        
        for pair in &self.pairs {
            let pair_name = pair.0.as_str();
            let pair_value = pair.1;
            
            #[allow(clippy::collapsible_if)]
            if pair_name == name {
                if let Value::Num(num) = pair_value {
                    left = *num;
                    right = *num;
                    bottom = *num;
                    top = *num;
                }
            } else if pair_name == name.to_owned() + "_x" {
                if let Value::Num(num) = pair_value {
                    left = *num;
                    right = *num;
                }
            } else if pair_name == name.to_owned() + "_y" {
                if let Value::Num(num) = pair_value {
                    bottom = *num;
                    top = *num;
                }
            } else if pair_name == name.to_owned() + "_l" || pair_name == name.to_owned() + "_left" {
                if let Value::Num(num) = pair_value {
                    left = *num;
                }
            } else if pair_name == name.to_owned() + "_r" || pair_name == name.to_owned() + "_right" {
                if let Value::Num(num) = pair_value {
                    right = *num;
                }
            } else if pair_name == name.to_owned() + "_b" || pair_name == name.to_owned() + "_bottom" {
                if let Value::Num(num) = pair_value {
                    bottom = *num;
                }
            } else if pair_name == name.to_owned() + "_t" || pair_name == name.to_owned() + "_top" {
                if let Value::Num(num) = pair_value {
                    top = *num;
                }
            }
        }
        
        Area {
            a: Vec2 (
                left,
                top
            ),
            b: Vec2 (
                right,
                bottom
            )
        }
    }
    pub fn get_padding(&self) -> Area {
        self.get_area_keys("padding")
    }
    pub fn get_margin(&self) -> Area {
        self.get_area_keys("margin")
    }
}

#[derive(Debug, Clone)]
pub struct NodeWrapper {
    node: Node,
    cache_data: Option<(Area, ContainerContext)>
}
#[derive(Debug, Clone)]
pub enum Node {
    Empty,
    
    Document {
        children: Children
    },
    Block {
        block_type: BlockType,
        children: Children,
        header: Header,
        
        // TODO: replace with struct
        render_data: Option<(Area, Option<Color>, Option<Rounding>)>
    },
    
    Element {
        value: Value,
        header: Header,
        
        // TODO: replace with struct
        render_data: Option<(Area, String, f32, Color)>,
    }
}

impl PartialEq<FrameDirection> for FrameDirection {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other),
            (FrameDirection::Horizontal, FrameDirection::Horizontal) |
            (FrameDirection::Vertical, FrameDirection::Vertical)
        )
    }
}

fn is_size_applicable(pair_name: &str, dir: &FrameDirection) -> bool {
    pair_name == "size" ||
        (pair_name == "width" && *dir == FrameDirection::Horizontal) ||
        (pair_name == "height" && *dir == FrameDirection::Vertical)
}

impl NodeWrapper {
    pub fn new<'a>(node: Node) -> NodeWrapper {
        NodeWrapper {
            node,
            cache_data: None
        }
    }
     
    pub fn render(&self, handle: &mut GLDrawHandle) {
        self.node.render(handle)
    }
    pub fn update(&mut self, update_ctx: UpdateCtx, parent_area: &Area, context: &mut ContainerContext) -> Result<(), Error> {
        self.cache_data = Some((*parent_area, context.clone()));
        self.node.update(update_ctx, parent_area, context)
    }
    pub fn update_from_cache(&mut self, update_ctx: UpdateCtx) -> Result<(), Error> {
        if let Some(mut data) = self.cache_data.clone() {
            self.update(update_ctx, &data.0, &mut data.1)?;
        } else {
            eprintln!("NodeWrapper::update_from_cache() called on node without cache");
        }
        
        Ok(())
    }
    
    pub fn get_header(&self) -> Option<&Header> {
        self.node.get_header()
    }
}

impl Node {
    pub fn new_empty() -> Node {
        Node::Empty
    }
    pub fn new_document(children: Children) -> Node {
        Node::Document {
            children
        }
    }
    pub fn new_block(
        block_type: BlockType,
        children: Children,
        header: Header
    ) -> Node {
        Node::Block {
            block_type,
            children,
            header,
            
            render_data: None
        }
    }
    pub fn new_element(
        value: Value,
        header: Header
    ) -> Node {
        Node::Element {
            value,
            header,
            
            render_data: None,
        }
    }
    
    pub fn render(&self, handle: &mut GLDrawHandle) {
        fn render_children(handle: &mut GLDrawHandle, children: &Children) {
            for child in children {
                child.render(handle);
            }
        }
        
        match self {
            Node::Document { children } => {
                render_children(handle, children);
            }
            
            Node::Block {
                block_type: BlockType::Frame,
                children,
                
                render_data: Some((area, ..)),
                ..
            } => {
                if DEBUG {
                    handle.draw_rectangle(area, Color { r: 0, g: 255, b: 0, a: 255});
                }
                
                render_children(handle, children);
            }
            Node::Block {
                children,
                
                render_data: Some((area, color, rounding)),
                ..
            } => {
                if let Some(color) = color {
                    handle.draw_filled_rectangle(area, &rounding.clone().unwrap_or(Rounding::default()), *color);
                }
                
                if DEBUG {
                    handle.draw_rectangle(area, Color { r: 255, g: 0, b: 0, a: 255});
                }
                
                render_children(handle, children);
            }
            
            Node::Element {
                render_data: Some((area, text, size, color)),
                ..
            } => {
                if DEBUG {
                    handle.draw_rectangle(area, Color { r: 0, g: 0, b: 255, a: 255});
                }
                
                // TODO: separate this into lines (and also change the struct) and handle alignment & color
                handle.draw_text(&text.clone(), area.a, *size * 2.0, *color);
            }
            
            _ => ()
        }
    }
    
    pub fn update(&mut self, update_ctx: UpdateCtx, parent_area: &Area, context: &mut ContainerContext) -> Result<(), Error> {
        match self {
            Node::Document { children } => {
                update_children(update_ctx, children, parent_area)?;
            }
            
            Node::Block {
                block_type: BlockType::Frame,
                header,
                children,
                
                render_data,
                ..
            } => {
                let area = parent_area.pad(header.get_margin());
                
                let mut dir = FrameDirection::Horizontal;
                let mut flipped = false;
                
                for flag in &header.flags {
                    match flag.as_str() {
                        "Horizontal" => {
                            dir = FrameDirection::Horizontal;
                        },
                        "Vertical" => {
                            dir = FrameDirection::Vertical;
                        }
                        
                        "Flipped" => {
                            flipped = !flipped;
                        },
                        
                        _ => ()
                    }
                }
                
                *render_data = Some(update_frame(update_ctx, &area, header, children, &FrameData {
                    dir,
                    flipped
                })?);
            }
            Node::Block {
                children,
                header,
                
                render_data,
                ..
            } => {
                let area = parent_area.pad(header.get_margin());
                
                let child_area = area.pad(header.get_padding());
                
                update_children((&mut *update_ctx.0, update_ctx.1), children, &child_area)?;
                
                *render_data = Some(update_block(update_ctx, &area, header)?);
            }
            
            Node::Element {
                value,
                header,
                
                render_data,
            } => {
                let area = parent_area.pad(header.get_margin());
                
                let data = update_element(update_ctx, &area, header, value, context)?;
                
                *render_data = Some(data);
            }
            
            Node::Empty => ()
        }
        
        Ok(())
    }
    
    pub fn get_header(&self) -> Option<&Header> {
        match self {
            Node::Empty | Node::Document { .. } =>
                None,
            
            Node::Block { header, .. } | Node::Element { header, .. } =>
                Some(header),
        }
    }
}

fn update_children(update_ctx: UpdateCtx, children: &mut Children, area: &Area) -> Result<(), Error> {
    let mut context = ContainerContext::new();
    for child in children {
        child.update((&mut *update_ctx.0, update_ctx.1), area, &mut context)?;
    }
    
    Ok(())
}

fn update_frame(
    update_ctx: UpdateCtx,
    area: &Area,
    header: &Header,
    children: &mut Children,
    data: &FrameData
) -> Result<(Area, Option<Color>, Option<Rounding>), Error> {
    let dir = &data.dir;
    let flipped = data.flipped;
    
    let mut used = 0.0;
    let total = match dir {
        FrameDirection::Horizontal => area.width(),
        FrameDirection::Vertical => area.height()
    };
    
    let content_area = area.pad(header.get_padding());
    
    let mut context = ContainerContext::new();
    
    for child in children {
        let mut size = total - used;
        
        let header = child.get_header();
        
        if let Some(header) = header {
            for pair in &header.pairs {
                let pair_name = pair.0.as_str();
                let pair_value = pair.1;
                
                if is_size_applicable(pair_name, dir) {
                    size = match pair_value {
                        Value::Num(val) =>
                            Ok(*val),
                        Value::Percentage(val) =>
                            Ok(val / 100.0 * (total - used)),
                        
                        Value::Str(..) =>
                            Err(Error::ValueTypeMismatch (
                                ErrPosition,
                                String::from("'num' or 'percentage'"),
                                String::from("str")
                            )),
                        Value::Color(..) =>
                            Err(Error::ValueTypeMismatch (
                                ErrPosition,
                                String::from("'num' or 'percentage'"),
                                String::from("color")
                            )),
                        Value::Property(..) =>
                            Err(Error::ValueTypeMismatch (
                                ErrPosition,
                                String::from("'num' or 'percentage'"),
                                String::from("property")
                            )),
                    }?;
                }
            }
        }
        
        let child_area = match dir {
            FrameDirection::Horizontal => Area {
                a: Vec2 (
                    if flipped { content_area.b.0 - used - size } else { content_area.a.0 + used },
                    content_area.a.1
                ),
                b: Vec2 (
                    if flipped { content_area.b.0 - used } else { content_area.a.0 + used + size },
                    content_area.b.1
                ),
            },
            FrameDirection::Vertical => Area {
                a: Vec2 (
                    content_area.a.0,
                    if flipped { content_area.b.1 - used - size } else { content_area.a.1 + used },
                ),
                b: Vec2 (
                    content_area.b.0,
                    if flipped { content_area.b.1 - used } else { content_area.a.1 + used + size },
                )
            }
        };
        
        child.update((&mut *update_ctx.0, update_ctx.1), &child_area, &mut context)?;
        
        used += size;
    }
    
    Ok(update_block(update_ctx, area, header)?)
}

fn update_block(
    update_ctx: UpdateCtx,
    area: &Area,
    header: &Header,
) -> Result<(Area, Option<Color>, Option<Rounding>), Error> {
    let color = header.expect("color", "color")?;
    
    let color = match color {
        Some(v) => Some(v.get_color(update_ctx.1).clone()),
        None => None
    };
    
    Ok((*area, color, Some(get_rounding(header)?)))
}

fn update_element(
    update_ctx: UpdateCtx,
    area: &Area,
    header: &Header,
    value: &Value,
    context: &mut ContainerContext
) -> Result<(Area, String, f32, Color), Error> {
    let text = match value {
        Value::Str(str) => str.as_str(),
        Value::Num(num) => &*num.to_string(),
        
        _ => {
            return Err(Error::InvalidElemType(ErrPosition, String::from(value.get_type())))
        }
    };
    
    let lines: Vec<String> = text
        .split('\n')
        .map(std::string::ToString::to_string)
        .collect();
    
    // font
    let size = header.expect("size", "num")?
        .unwrap_or(&Value::Num(10.0))
        .get_num();
    
    let spacing = header.expect("spacing", "num")?
        .unwrap_or(&Value::Num(1.0))
        .get_num();
    
    let line_height = header.expect("line_height", "num")?
        .unwrap_or(&Value::Num(1.0))
        .get_num();
    
    let alignment = get_alignment(header, Alignment::from(context.anchor_x))?;
    
    // get size
    let mut width: f32 = 0.0;
    let mut height = 2.0 * *size;
    
    for line in &lines {
        let line_width = update_ctx.0.text_line_width(line) as f32 * *spacing * *size / 5.0;
        width = width.max(line_width);
    }
    let positioned_area = position_element(area, header, Vec2(width, height), context)?;
    
    let color = header.expect("color", "color")?;
    if let Some(color) = color {
        context.color = color.get_color(update_ctx.1);
    }
    
    Ok((positioned_area, text.to_string(), *size, context.color))
}

fn position_element(
    area: &Area,
    header: &Header,
    size: Vec2,
    context: &mut ContainerContext
) -> Result<Area, Error> {
    // TODO: handle margin and maybe padding?
    let mut anchor = get_anchor(header)?;
    
    // if there is no position yet, set it to the center
    if context.pos.is_none() {
        if anchor.0.is_none() {
            anchor.0 = Some(AnchorX::Center);
        }
        if anchor.1.is_none() {
            anchor.1 = Some(AnchorY::Center);
        }
    }
    
    //println!("{anchor:?}, {size:?}, {context:?}");
    
    // get anchor data
    let x = if let Some(x_anchor) = anchor.0 {
        context.anchor_x = x_anchor;
        match x_anchor {
            AnchorX::Left => area.a.0,
            AnchorX::Center => area.center().0,
            AnchorX::Right => area.b.0,
        }
    } else { context.pos.unwrap().0 };
    
    let mut y = if let Some(y_anchor) = anchor.1 {
        context.anchor_y = y_anchor;
        match y_anchor {
            AnchorY::Top => area.a.1,
            AnchorY::Center => area.center().1,
            AnchorY::Bottom => area.b.1,
        }
    } else { context.pos.unwrap().1 };
    
    let offset_amount_x = match context.anchor_x {
        AnchorX::Left => 0.0,
        AnchorX::Center => 0.5,
        AnchorX::Right => 1.0
    };
    let offset_amount_y = match context.anchor_y {
        AnchorY::Top => 0.0,
        AnchorY::Center => 0.5,
        AnchorY::Bottom => 1.0
    };
    
    context.pos = Some(Vec2(x,y - size.1 * match context.anchor_y {
        AnchorY::Top | AnchorY::Center => -1.0,
        AnchorY::Bottom => 1.0
    }));
    
    let left = x - size.0 * (offset_amount_x);
    let right = x + size.0 * (1.0 - offset_amount_x);
    let top = y - size.1 * (offset_amount_y);
    let bottom = y + size.1 * (1.0 - offset_amount_y);
    
    Ok(Area {
        a: Vec2(left, top),
        b: Vec2(right, bottom)
    })
}

fn get_rounding(header: &Header) -> Result<Rounding, Error> {
    let mut rounding = Rounding::default();
    
    if let Some(v) = header.expect("rounding", "num")? {
        rounding = (*v.get_num()).into();
    }
    
    Ok(rounding)
}

const VALID_ANCHORS: [&str; 18] = [
    "tl","t","tr",
    "l", "c","r" ,
    "bl","b","br",
    
    "top left", "top",    "top right",
    
    "left",     "center", "right",
    
    "bottom",   "bottom", "bottom right"
];
fn get_anchor(header: &Header) -> Result<(Option<AnchorX>, Option<AnchorY>), Error> {
    let mut anchor_x = None;
    let mut anchor_y = None;
    
    for pair in &header.pairs {
        let pair_name = pair.0.as_str();
        let pair_value = pair.1;
        
        if pair_name == "anchor"
            && let Value::Str(name) = pair_value {
            if !VALID_ANCHORS.contains(&name.as_str()) {
                return Err(Error::InvalidAnchor(ErrPosition, name.clone()))
            }
            
            if name.ends_with('l') || name.ends_with("left") {
                anchor_x = Some(AnchorX::Left);
            }
            if name.ends_with('r') || name.ends_with("right") {
                anchor_x = Some(AnchorX::Right);
            }
            
            if name.starts_with('t') || name.ends_with("top") {
                anchor_y = Some(AnchorY::Top);
            }
            if name.starts_with('b') || name.ends_with("bottom") {
                anchor_y = Some(AnchorY::Bottom);
            }
        }
    }
    
    Ok((anchor_x, anchor_y))
}

const VALID_ALIGNMENTS: [&str; 6] = [
    "l", "c","r" ,
    
    "left", "center", "right",
];
fn get_alignment(header: &Header, default: Alignment) -> Result<Alignment, Error> {
    let mut alignment = default;
    
    for pair in &header.pairs {
        let pair_name = pair.0.as_str();
        let pair_value = pair.1;
        
        if pair_name == "alignment"
            && let Value::Str(name) = pair_value {
            if !VALID_ALIGNMENTS.contains(&name.as_str()) {
                return Err(Error::InvalidAlignment(ErrPosition, name.clone()))
            }
            
            if name.ends_with('l') || name.ends_with("left") {
                alignment = Alignment::Left;
            }
            if name.ends_with('r') || name.ends_with("right") {
                alignment = Alignment::Right;
            }
        }
    }
    
    Ok(alignment)
}
