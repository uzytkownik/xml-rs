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

#[unsafe_destructor]
impl Drop for Document {
    fn drop(&mut self) {
        unsafe {
            ffi::xmlFreeDoc(self.doc);
        }
    }
}

impl Document {
    /**
     * Find the root element, if it exists.
     */
    pub fn get_root_element<'r>(&'r self) -> Option<Element<'r>> {
        use std::ptr::is_not_null;
        unsafe {
            let elem = ffi::xmlDocGetRootElement(self.doc);
            if (is_not_null(elem)) {
                Some(Element {node: &*elem})
            } else {
                None
            }
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
     * Finds all children of the element.
     */
    pub fn children(&self) -> ~[ElementChildren<'r>] {
        std::vec::build(None, |push| {
            unsafe {
                let mut cur = self.node.children;
                while (std::ptr::is_not_null(cur)) {
                    match (*cur)._type {
                        ffi::ElementNode => push(ElementElementChild(Element {node: &*cur})),
                        ffi::TextNode => push(TextElementChild(Text {node: &*cur})),
                        t => error!("Unsupported type {}", t.to_str())
                    }
                    cur = (*cur).next;
                }
            }
        })
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
        use std::ptr::{null,is_not_null};
        ffi::xmlCheckVersion(ffi::xmlVersion);
        let doc = ffi::xmlReadMemory(buffer.as_ptr(), buffer.len() as c_int,
                                     url.map(|x| x.as_ptr()).unwrap_or(null()) as *i8,
                                     null(), 64 | 32 /* No errors or warnings for now */);
        if (is_not_null(doc)) {
            Some(Document {doc: doc})
        } else {
            None
        }
    }
}

/**
 * Reads the buffer and return the document if it contains valid XML.
 */
pub fn read_memory(buffer: &[u8]) -> Option<Document> {
    read_memory_with_url(buffer, None)
}
