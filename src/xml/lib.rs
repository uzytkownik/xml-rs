/*
 * Copyright (C) 2014 Maciej Piechotka
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

#[comment = "Wrapper for libxml2's DOM parser."];
#[crate_id = "github.com/uzytkownik/xml-rs#xml:0.1"];
#[crate_type = "lib"];
#[license = "MIT"];

extern mod extra;

mod ffi;

/**
 * An XML node that contains text.
 */
pub trait TextNode {
    fn content(&self) -> ~str;
}

/**
 * An attribute of element.
 */
pub struct Attribute<'r> {
    priv attr: &'r ffi::xmlAttr
}

/**
 * Iterator over children of attribute
 */
pub struct AttributeChildrenIterator<'r> {
    priv cur: Option<&'r ffi::xmlNode>
}

/**
 * A CDATA section.
 */
pub struct CData<'r> {
    priv node: &'r ffi::xmlNode
}

/**
 * An XML comment.
 */
pub struct Comment<'r> {
    priv node: &'r ffi::xmlNode
}

/**
 * An XML document.
 */
pub struct Document {
    priv doc: *ffi::xmlDoc
}

/**
 * An XML element
 */
pub struct Element<'r> {
    priv node: &'r ffi::xmlNode
}

/**
 * Iterator over children of XML element
 */
pub struct ElementChildrenIterator<'r> {
    priv cur: Option<&'r ffi::xmlNode>
}

/**
 * Iterator over attributes of XML element
 */
pub struct ElementAttributeIterator<'r> {
    priv cur: Option<&'r ffi::xmlAttr>
}

/**
 * Text inside an XML.
 */
pub struct Text<'r> {
    priv node: &'r ffi::xmlNode
}

/**
 * Possible children of an attribute.
 */
pub enum AttributeChild<'r> {
    TextAttributeChild(Text<'r>)
}

/**
 * Possible children of an element.
 */
pub enum ElementChild<'r> {
    ElementElementChild(Element<'r>),
    TextElementChild(Text<'r>),
    CDataElementChild(CData<'r>),
    CommentElementChild(Comment<'r>)
}

impl Document {
    /**
     * Find the root element, if it exists.
     */
    pub fn get_root_element<'r>(&'r self) -> Option<Element<'r>> {
        unsafe {
            ptr_to_option(ffi::xmlDocGetRootElement(self.doc)).map(|elem| Element{node: &*elem})
        }
    }
}

#[unsafe_destructor]
impl Drop for Document {
    fn drop(&mut self) {
        unsafe {
            ffi::xmlFreeDoc(self.doc);
        }
    }
}

impl<'r> Comment<'r> {
    /// Get comment contents.
    pub fn comment(&self) -> ~str {
        unsafe {
            std::str::raw::from_c_str(self.node.content as *i8)
        }
    }
}

impl<'r> Attribute<'r> {
    /**
     * Gets the name of the attribute.
     */
    pub fn name(&self) -> ~str {
        unsafe {std::str::raw::from_c_str(self.attr.name as *i8)}
    }
    /**
     * Iterate over children
     */
    pub fn children_iter(&self) -> AttributeChildrenIterator<'r> {
        AttributeChildrenIterator {
            cur: ptr_to_option(self.attr.children).map(|cur| unsafe {&*cur})
        }
    }
    /**
     * Gets the value of the attribute
     */
    pub fn value(&self) -> ~str {
        self.children_iter().map(|c| {
            match c {
                TextAttributeChild(t) => t.content()
            }
        }).to_owned_vec().concat()
    }
}

impl<'r> Element<'r> {
    /**
     * Gets the name of the element.
     */
    pub fn name(&self) -> ~str {
        unsafe {std::str::raw::from_c_str(self.node.name as *i8)}
    }
    /**
     * Iterate over children
     */
    pub fn children_iter(&self) -> ElementChildrenIterator<'r> {
        ElementChildrenIterator {
            cur: ptr_to_option(self.node.children).map(|cur| unsafe {&*cur})
        }
    }
    /**
     * Iterate over arguments.
     */
    pub fn attribute_iter(&self) -> ElementAttributeIterator<'r> {
        ElementAttributeIterator {
            cur: ptr_to_option(self.node.properties).map(|cur| unsafe {&*cur})
        }
    }
}

impl<'r> Clone for AttributeChildrenIterator<'r> {
    fn clone(&self) -> AttributeChildrenIterator<'r> {
        AttributeChildrenIterator{cur: self.cur}
    }
}

impl<'r> Iterator<AttributeChild<'r>> for AttributeChildrenIterator<'r> {
    fn next(&mut self) -> Option<AttributeChild<'r>> {
        self.cur.and_then(|cur| {
            self.cur = unsafe {ptr_to_option(cur.next).map(|next| &*next)};
            match cur._type {
                ffi::TextNode => Some(TextAttributeChild(Text {node: cur})),
                t => {
                    error!("Unsupported type {}", t.to_str());
                    self.next()
                }
            }
        })
    }
}

impl<'r> Clone for ElementChildrenIterator<'r> {
    fn clone(&self) -> ElementChildrenIterator<'r> {
        ElementChildrenIterator{cur: self.cur}
    }
}

impl<'r> Iterator<ElementChild<'r>> for ElementChildrenIterator<'r> {
     fn next(&mut self) -> Option<ElementChild<'r>> {
        self.cur.and_then(|cur| {
            self.cur = unsafe {ptr_to_option(cur.next).map(|next| &*next)};
            match cur._type {
                ffi::ElementNode => Some(ElementElementChild(Element {node: cur})),
                ffi::TextNode => Some(TextElementChild(Text {node: cur})),
                ffi::CDataSectionNode => Some(CDataElementChild(CData {node: cur})),
                ffi::CommentNode => Some(CommentElementChild(Comment {node: cur})),
                t => {
                    error!("Unsupported type {}", t.to_str());
                    self.next()
                }
            }
        })
    }
}

impl<'r> Clone for ElementAttributeIterator<'r> {
    fn clone(&self) -> ElementAttributeIterator<'r> {
        ElementAttributeIterator {cur: self.cur}
    }
}

impl<'r> Iterator<Attribute<'r>> for ElementAttributeIterator<'r> {
    fn next(&mut self) -> Option<Attribute<'r>> {
        self.cur.and_then(|cur| {
            self.cur = unsafe {ptr_to_option(cur.next).map(|next| &*next)};
            Some(Attribute {attr: cur})
        })
    }
}

impl<'r> AttributeChild<'r> {
    /// Check if children is text
    pub fn is_text(self) -> bool {
        match (self) {
            TextAttributeChild(_) => true,
            //_ => false
        }
    }
    /// Return text if it is text
    pub fn get_text(self) -> Option<Text<'r>> {
        match (self) {
            TextAttributeChild(t) => Some(t),
            //_ => None
        }
    }
}

impl<'r> ElementChild<'r> {
    /// Check if child is an element.
    pub fn is_element(self) -> bool {
        match (self) {
            ElementElementChild(_) => true,
            _ => false
        }
    }
    /// Check if child is text.
    pub fn is_text(self) -> bool {
        match (self) {
            TextElementChild(_) => true,
            _ => false
        }
    }
    /// Check if child is CDATA section
    pub fn is_cdata(self) -> bool {
        match (self) {
            CDataElementChild(_) => true,
            _ => false
        }
    }
    /// Check if child is a comment.
    pub fn is_comment(self) -> bool {
        match (self) {
            CommentElementChild(_) => true,
            _ => false
        }
    }
    /// Get element if it is an element.
    pub fn get_element(self) -> Option<Element<'r>> {
        match (self) {
            ElementElementChild(e) => Some(e),
            _ => None
        }
    }
    /// Get text if it is text.
    pub fn get_text(self) -> Option<Text<'r>> {
        match (self) {
            TextElementChild(t) => Some(t),
            _ => None
        }
    }
    /// Get cdata section if it is cdata section.
    pub fn get_cdata(self) -> Option<CData<'r>> {
        match (self) {
            CDataElementChild(cd) => Some(cd),
            _ => None
        }
    }
    // Get comment if it is a comment.
    pub fn get_comment(self) -> Option<Comment<'r>> {
    match (self) {
            CommentElementChild(c) => Some(c),
            _ => None
        }
    }
}

impl<'r> TextNode for CData<'r> {
    fn content(&self) -> ~str {
        unsafe {
            std::str::raw::from_c_str(self.node.content as *i8)
        }
    }
}

impl<'r> TextNode for Text<'r> {
    fn content(&self) -> ~str {
        unsafe {
            std::str::raw::from_c_str(self.node.content as *i8)
        }
    }
}

/**
 * Reads the buffer treating it as if it came from url.
 *
 * It returns the document if buffer contained valid XML.
 */
pub fn read_memory_with_url(buffer: &[u8], url: Option<&str>) -> Option<Document> {
    unsafe {
        use std::libc::c_int;
        use std::ptr::null;
        ffi::xmlCheckVersion(ffi::xmlVersion);
        let doc = ffi::xmlReadMemory(buffer.as_ptr(), buffer.len() as c_int,
                                     url.map(|x| x.as_ptr()).unwrap_or(null()) as *i8,
                                     null(), 64 | 32 /* No errors or warnings for now */);
        ptr_to_option(doc).map(|doc| Document {doc: doc})
    }
}

/**
 * Reads the buffer and return the document if it contains valid XML.
 */
pub fn read_memory(buffer: &[u8]) -> Option<Document> {
    read_memory_with_url(buffer, None)
}

fn ptr_to_option<T>(ptr: *T) -> Option<*T> {
    if (std::ptr::is_not_null(ptr)) {
        Some(ptr)
    } else {
        None
    }
}
