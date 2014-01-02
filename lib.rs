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
 * Text inside an XML.
 */
pub struct Text<'r> {
    priv node: &'r ffi::xmlNode
}

/**
 * Possible children of an element.
 */
pub enum ElementChildren<'r> {
    ElementElementChild(Element<'r>),
    TextElementChild(Text<'r>)
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
}

impl<'r> Clone for ElementChildrenIterator<'r> {
    fn clone(&self) -> ElementChildrenIterator<'r> {
        ElementChildrenIterator{cur: self.cur}
    }
}

impl<'r> Iterator<ElementChildren<'r>> for ElementChildrenIterator<'r> {
     fn next(&mut self) -> Option<ElementChildren<'r>> {
        self.cur.and_then(|cur| {
            self.cur = unsafe {ptr_to_option(cur.next).map(|next| &*next)};
            match cur._type {
                ffi::ElementNode => Some(ElementElementChild(Element {node: cur})),
                ffi::TextNode => Some(TextElementChild(Text {node: cur})),
                t => {
                    error!("Unsupported type {}", t.to_str());
                    self.next()
                }
            }
        })
    }
}

impl<'r> ElementChildren<'r> {
    /// Check if children is an element.
    pub fn is_element(self) -> bool {
        match (self) {
            ElementElementChild(_) => true,
            _ => false
        }
    }
    /// Check if children is text.
    pub fn is_text(self) -> bool {
        match (self) {
            TextElementChild(_) => true,
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
}

impl<'r> Text<'r> {
    pub fn content(&self) -> ~str {
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
