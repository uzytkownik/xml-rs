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
 * An XML node with a name and namespace.
 */
pub trait NamedNode {
    fn name(&self) -> ~str;
    fn namespace<'r>(&'r self) -> Option<BorrowedNamespace<'r>>;
}

/**
 * An attribute of element.
 */
pub struct BorrowedAttribute<'r> {
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
pub struct BorrowedCData<'r> {
    priv node: &'r ffi::xmlNode
}

/**
 * An XML comment.
 */
pub struct BorrowedComment<'r> {
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
pub struct BorrowedElement<'r> {
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
 * An XML namespace
 */
pub struct BorrowedNamespace<'r> {
    priv ns: &'r ffi::xmlNs
}

/**
 * Text inside an XML.
 */
pub struct BorrowedText<'r> {
    priv node: &'r ffi::xmlNode
}

/**
 * Possible children of an attribute.
 */
pub enum AttributeChild<'r> {
    TextAttributeChild(BorrowedText<'r>)
}

/**
 * Possible children of an element.
 */
pub enum ElementChild<'r> {
    ElementElementChild(BorrowedElement<'r>),
    TextElementChild(BorrowedText<'r>),
    CDataElementChild(BorrowedCData<'r>),
    CommentElementChild(BorrowedComment<'r>)
}

impl Document {
    /**
     * Find the root element, if it exists.
     */
    pub fn get_root_element<'r>(&'r self) -> Option<BorrowedElement<'r>> {
        unsafe {
            ptr_to_option(ffi::xmlDocGetRootElement(self.doc)).map(|elem| BorrowedElement{node: &*elem})
        }
    }

    /**
     * Parse the document from reader.
     */
    pub fn read(reader: &mut Reader) -> Option<Document> {
        use std::io::{IoError,EndOfFile,io_error};
        use std::libc::{c_char, c_int, c_void};
        use std::ptr::{null,to_mut_unsafe_ptr};
        unsafe {ffi::xmlCheckVersion(ffi::xmlVersion)};
        struct Context<'t> {
            reader: &'t mut Reader,
            ioerror: Option<IoError>
        }
        unsafe fn get_context<'t>(context_ptr: &'t *mut c_void) -> &'t mut Context<'t> {
            &mut *(context_ptr.clone() as *mut Context<'t>)
        }
        extern "C" fn ioread(context_ptr: *mut c_void, buf: *mut c_char, len: c_int) -> c_int {
            unsafe {
                let context = get_context(&context_ptr);
                match context.ioerror {
                    Some(_) => -1,
                    None => io_error::cond.trap(|err| {
                            if err.kind != EndOfFile {
                                context.ioerror = Some(err)
                            }
                        }).inside(|| {
                            std::vec::raw::mut_buf_as_slice(buf as *mut u8, len as uint, |v| {
                                context.reader.read(v).map_default(-1, |x| x as c_int)
                            })
                        })
                }
            }
        }
        extern "C" fn ioclose(_: *mut c_void) -> c_int {0};
        let mut context = Context {
            reader: reader,
            ioerror: None
        };
        let doc = unsafe {
            let ctx = &mut context;
            ffi::xmlReadIO(ioread, ioclose,
                           to_mut_unsafe_ptr(ctx) as *mut c_void,
                           null(), null(),
                           64 | 32 /* No errors or warnings for now */)
        };
        match (context.ioerror, ptr_to_option(doc)) {
            (Some(err), _) => {io_error::cond.raise(err); None},
            (None, None) => None,
            (None, Some(doc)) => Some(Document {doc: doc}),
        }
    }

    /**
     * Write document to writer
     */
    pub fn write(&self, writer: &mut Writer) {
        use std::io::{IoError,io_error};
        use std::libc::{c_char, c_int, c_void};
        use std::ptr::{null,to_mut_unsafe_ptr};
        struct Context<'t> {
             writer: &'t mut Writer,
             ioerror: Option<IoError>
        }
        unsafe fn get_context<'t>(context_ptr: &'t *mut c_void) -> &'t mut Context<'t> {
            &mut *(context_ptr.clone() as *mut Context<'t>)
        }
        extern "C" fn iowrite(context_ptr: *mut c_void, buf: *c_char, len: c_int) -> c_int {
            unsafe {
                let context = get_context(&context_ptr);
                let vec = std::vec::raw::from_buf_raw(buf as *u8, len as uint);
                io_error::cond.trap(|err| (*context).ioerror = Some(err)).inside(|| {
                    context.writer.write(vec);
                });
                context.ioerror.as_ref().map_default(len, |_| -1)
            }
        }
        extern "C" fn ioclose(context_ptr: *mut c_void) -> c_int {
            unsafe {
                let context = get_context(&context_ptr);
                io_error::cond.trap(|err| (*context).ioerror = Some(err)).inside(|| {
                    context.writer.flush();
                });
                context.ioerror.as_ref().map_default(0, |_| -1)
            }
        }
        let mut context = Context {
            writer: writer,
            ioerror: None
        };
        unsafe {
            let ctx = &mut context;
            let save_ctx = ffi::xmlSaveToIO(iowrite, ioclose, to_mut_unsafe_ptr(ctx) as *mut c_void, null(), 0);
            ffi::xmlSaveDoc(save_ctx, self.doc);
            ffi::xmlSaveClose(save_ctx);
        }
        context.ioerror.map(|e| {io_error::cond.raise(e)});
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

impl<'r> BorrowedComment<'r> {
    /// Get comment contents.
    pub fn comment(&self) -> ~str {
        unsafe {
            std::str::raw::from_c_str(self.node.content as *i8)
        }
    }
}

impl<'r> BorrowedAttribute<'r> {
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

impl<'r> BorrowedElement<'r> {
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

impl<'r> BorrowedNamespace<'r> {
    /**
     * Get the namespace URI
     */
    pub fn href(&self) -> ~str {
        unsafe {
            std::str::raw::from_c_str(self.ns.href as *i8)
        }
    }
    /**
     * Get the namespace prefix
     */
    pub fn prefix(&self) -> Option<~str> {
        unsafe {
            ptr_to_option(self.ns.prefix).map(|p| std::str::raw::from_c_str(p as *i8))
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
                ffi::TextNode => Some(TextAttributeChild(BorrowedText {node: cur})),
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
                ffi::ElementNode => Some(ElementElementChild(BorrowedElement {node: cur})),
                ffi::TextNode => Some(TextElementChild(BorrowedText {node: cur})),
                ffi::CDataSectionNode => Some(CDataElementChild(BorrowedCData {node: cur})),
                ffi::CommentNode => Some(CommentElementChild(BorrowedComment {node: cur})),
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

impl<'r> Iterator<BorrowedAttribute<'r>> for ElementAttributeIterator<'r> {
    fn next(&mut self) -> Option<BorrowedAttribute<'r>> {
        self.cur.and_then(|cur| {
            self.cur = unsafe {ptr_to_option(cur.next).map(|next| &*next)};
            Some(BorrowedAttribute {attr: cur})
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
    pub fn get_text(self) -> Option<BorrowedText<'r>> {
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
    pub fn get_element(self) -> Option<BorrowedElement<'r>> {
        match (self) {
            ElementElementChild(e) => Some(e),
            _ => None
        }
    }
    /// Get text if it is text.
    pub fn get_text(self) -> Option<BorrowedText<'r>> {
        match (self) {
            TextElementChild(t) => Some(t),
            _ => None
        }
    }
    /// Get cdata section if it is cdata section.
    pub fn get_cdata(self) -> Option<BorrowedCData<'r>> {
        match (self) {
            CDataElementChild(cd) => Some(cd),
            _ => None
        }
    }
    // Get comment if it is a comment.
    pub fn get_comment(self) -> Option<BorrowedComment<'r>> {
    match (self) {
            CommentElementChild(c) => Some(c),
            _ => None
        }
    }
}

impl<'r> NamedNode for BorrowedAttribute<'r> {
    fn name(&self) -> ~str {
        unsafe {
            std::str::raw::from_c_str(self.attr.name as *i8)
        }
    }
    fn namespace<'t>(&'t self) -> Option<BorrowedNamespace<'t>> {
        unsafe {
            ptr_to_option(self.attr.ns).map(|ns| BorrowedNamespace{ns: &*ns})
        }
    }
}


impl<'r> TextNode for BorrowedCData<'r> {
    fn content(&self) -> ~str {
        unsafe {
            std::str::raw::from_c_str(self.node.content as *i8)
        }
    }
}

impl<'r> NamedNode for BorrowedElement<'r> {
    fn name(&self) -> ~str {
        unsafe {
            std::str::raw::from_c_str(self.node.name as *i8)
        }
    }
    fn namespace<'t>(&'t self) -> Option<BorrowedNamespace<'t>> {
        unsafe {
            ptr_to_option(self.node.ns).map(|ns| BorrowedNamespace{ns: &*ns})
        }
    }
}

impl<'r> TextNode for BorrowedText<'r> {
    fn content(&self) -> ~str {
        unsafe {
            std::str::raw::from_c_str(self.node.content as *i8)
        }
    }
}

fn ptr_to_option<T>(ptr: *T) -> Option<*T> {
    if (std::ptr::is_not_null(ptr)) {
        Some(ptr)
    } else {
        None
    }
}
