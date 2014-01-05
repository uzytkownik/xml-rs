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

use std::libc::{c_char, c_int, c_long, c_uchar, c_ushort, c_void};

pub type xmlChar = c_uchar;

#[deriving(ToStr)]
#[repr(C)]
pub enum xmlElementType {
    ElementNode = 1,
    AttributeNode = 2,
    TextNode = 3,
    CDataSectionNode = 4,
    EntityRefNode = 5,
    EntityNode = 6,
    PINode = 7,
    CommentNode = 8,
    DocumentNode = 9,
    DocumentTypeNode = 10,
    DocumentFragNode = 11,
    NotationNode = 12,
    HtmlDocumentNode = 13,
    DTDNode = 14,
    ElementDecl = 15,
    AttributeDecl = 16,
    EntityDecl = 17,
    NamespaceDecl = 18,
    XIncludeStart = 19,
    XIncludeEnd = 20,
    DOCBDocumentNode = 21
}

pub struct xmlAttr {
    _private: *c_void,
    _type: xmlElementType, // AttributeNode
    name: *c_char,
    children: *xmlNode,
    last: *xmlNode,
    parent: *xmlNode,
    next: *xmlAttr,
    prev: *xmlAttr,
    doc: *xmlDoc,
    ns: *xmlNs,
    atype: xmlAttributeType,
    psvi: *c_void
}

#[allow(dead_code)]
pub struct xmlAttribute {
    _private: *c_void,
    _type: xmlElementType, // AttributeDecl
    name: *c_char,
    children: *xmlNode, // NULL
    last: *xmlNode, // NULL
    parent: *xmlDtd, // NULL 
    next: *xmlNode,
    prev: *xmlNode,
    doc: *xmlDoc,
    nexth: *xmlAttribute,
    atype: xmlAttributeType,
    def: xmlAttributeDefault,
    defaultValue: *xmlChar,
    tree: *xmlEnumeration,
    prefix: *xmlChar,
    elem: *xmlChar
}

#[allow(dead_code)]
#[repr(C)]
pub enum xmlAttributeDefault {
    None = 1,
    Required = 2,
    Implied = 3,
    Fixed = 4
}

#[repr(C)]
pub enum xmlAttributeType {
    CDATA = 1,
    ID = 2,
    IDRef = 3,
    IDRefs = 4,
    Entity = 5,
    Entities = 6,
    NMToken = 7,
    NMTokens = 8,
    Enumeration = 9,
    Notation = 10
}

pub struct xmlDoc {
    _private: *c_void,
    _type: xmlElementType, // DocumentNode
    name: *c_char,
    children: *xmlNode,
    last: *xmlNode,
    parent: *xmlNode,
    next: *xmlNode,
    prev: *xmlNode,
    doc: *xmlDoc,
    compression: c_int,
    standalone: c_int,
    intSubset: *xmlDtd,
    extSubset: *xmlDtd,
    oldNs: *xmlNs,
    version: *c_char,
    encoding: *c_char,
    ids: *c_void,
    refs: *c_void,
    url: *c_char,
    charset: c_int,
    dict: *c_void,
    psvi: *c_void,
    parseFlags: c_int,
    properties: c_int
}

pub struct xmlDtd {
    _private: *c_void,
    _type: xmlElementType, // DTDNode
    name: *c_char,
    children: *xmlNode,
    last: *xmlNode,
    parent: *xmlDoc,
    next: *xmlNode,
    prev: *xmlNode,
    doc: *xmlDoc,
    notations: *c_void,
    elements: *c_void,
    attributes: *c_void,
    entities: *c_void,
    externalId: *xmlChar,
    systemId: *xmlChar,
    pentities: *c_void
}

#[allow(dead_code)]
pub struct xmlElement {
    _private: *c_void,
    _type: xmlElementType, // ElementDecl
    name: *c_char,
    children: *xmlNode, // NULL
    last: *xmlNode, // NULL
    parent: *xmlDtd,
    next: *xmlNode,
    prev: *xmlNode,
    doc: *xmlDoc,
    etype: xmlElementTypeVal,
    content: xmlElementContent,
    attributes: *xmlAttribute,
    prefix: *xmlChar,
    contModel: *c_void
}

#[allow(dead_code)]
#[repr(C)]
pub enum xmlElementTypeVal {
    Undefined,
    Empty,
    Any,
    Mixed,
    ElementType
}

#[allow(dead_code)]
pub struct xmlElementContent {
    _type: xmlElementContentType,
    ocur: xmlElementContentOccur,
    name: *xmlChar,
    first: *xmlElementContent,
    second: *xmlElementContent,
    parent: *xmlElementContent,
    prefix: *xmlChar
}

#[allow(dead_code)]
#[repr(C)]
pub enum xmlElementContentType {
    PCData,
    ElementContent,
    Seq,
    Or
}

#[allow(dead_code)]
#[repr(C)]
pub enum xmlElementContentOccur {
    Once,
    Opt,
    Mult,
    Plus
}

#[allow(dead_code)]
pub struct xmlEntity {
    _private: *c_void,
    _type: xmlElementType, // EntityDecl
    name: *c_char,
    children: *xmlNode,
    last: *xmlNode,
    parent: *xmlDtd,
    next: *xmlNode,
    prev: *xmlNode,
    doc: *xmlDoc,
    orig: *xmlChar,
    content: *xmlChar,
    length: xmlEntityType,
    externalID: *xmlChar,
    systemID: *xmlChar,
    nexte: *c_void,
    uri: *xmlChar,
    owner: c_int,
    checked: c_int
}

#[allow(dead_code)]
#[repr(C)]
pub enum xmlEntityType {
    InternalGeneralEntity = 1,
    ExternalGeneralParsedEntity = 2,
    ExternalGeneralUnparsedEntity = 3,
    InternalParameterEntity = 4,
    ExternalParameterEntity = 5,
    InternalPredefinedEntity = 6
}

#[allow(dead_code)]
pub struct xmlEnumeration {
    next: *xmlEnumeration,
    name: *xmlChar
}

pub struct xmlNode {
    _private: *c_void,
    _type: xmlElementType,
    name: *xmlChar,
    children: *xmlNode,
    last: *xmlNode,
    parent: *xmlNode,
    next: *xmlNode,
    prev: *xmlNode,
    doc: *xmlDoc,
    ns: *xmlNs,
    content: *xmlChar,
    properties: *xmlAttr,
    nsDef: *xmlNs,
    psvi: *c_void,
    line: c_ushort,
    extra: c_ushort
}

pub struct xmlNs {
    next: *xmlNs,
    _type: xmlElementType,
    href: *xmlChar,
    prefix: *xmlChar,
    private: *c_void,
    context: *xmlDoc
}

enum xmlSaveCtxt {}

#[link(name = "xml2")]
extern "C" {
    pub fn xmlCheckVersion(version: c_int);

    // Parser API
    pub fn xmlReadIO(ioread: extern "C" fn (context: *mut c_void, buffer: *mut c_char, len: c_int) -> c_int,
                     ioclose: extern "C" fn (context: *mut c_void) -> c_int,
                     context: *mut c_void,
                     url: *c_char,
                     encoding: *c_char,
                     options: c_int) -> *xmlDoc;

    // Tree API
    pub fn xmlDocGetRootElement(doc: *xmlDoc) -> *xmlNode;
    pub fn xmlFreeDoc(doc: *xmlDoc);

    // XML Save API
    pub fn xmlSaveClose(ctx: *xmlSaveCtxt) -> c_int;
    pub fn xmlSaveDoc(ctx: *xmlSaveCtxt, doc: *xmlDoc) -> c_long;
    pub fn xmlSaveToIO(iowrite: extern "C" fn (context: *mut c_void, buffer: *c_char, len: c_int) -> c_int,
                       ioclose: extern "C" fn (context: *mut c_void) -> c_int,
                       context: *mut c_void,
                       encoding: *c_char,
                       options: c_int) -> *xmlSaveCtxt;
}

pub static xmlVersion : c_int = 20901;
