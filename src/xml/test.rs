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

#[feature(macro_rules)];

extern mod xml;

macro_rules! expect_attribute(
    ($iter:ident, $expected_name:expr, $expected_ns:expr, $expected_value:expr, $attribute_check:expr) => ({
        let next = ($iter).next();
        assert!(next.is_some(), "Expected next attribute - there is none");
        let attr = next.unwrap();
        let name = attr.name();
        let value = attr.value();
        assert_eq!(name.slice_from(0), $expected_name);
        let expected_ns : Option<(Option<&str>, &str)> = $expected_ns;
        let prefix = attr.namespace().map(|x| x.prefix());
        let namespace = attr.namespace().map(|x| x.href());
        assert_eq!(prefix.as_ref().map(|x| x.as_ref().map(|y| y.slice_from(0))), expected_ns.as_ref().map(|x| x.n0_ref().clone()));
        assert_eq!(namespace.as_ref().map(|x| x.slice_from(0)), expected_ns.as_ref().map(|x| x.n1_ref().clone()));
        assert_eq!(value.slice_from(0), $expected_value);
        {
            let mut $iter = attr.children_iter();
            $attribute_check;
            assert!($iter.next().is_none(), "Unexpected children");
        }
    })
)

macro_rules! expect_cdata(
    ($iter:ident, $expected_text:expr) => ({
        use xml::TextNode;
        let next = ($iter).next();
        assert!(next.is_some(), "Expected CDATA section but there is no further element");
        let text = next.unwrap().get_cdata();
        assert!(text.is_some(), "Extected CDATA section but got other children");
        let cur = text.unwrap();
        let content = cur.content();
        assert_eq!(content.slice_from(0), $expected_text);
    });
)

macro_rules! expect_comment(
    ($iter:ident, $expected_text:expr) => ({
        let next = ($iter).next();
        assert!(next.is_some(), "Expected comment but there is no further element");
        let text = next.unwrap().get_comment();
        assert!(text.is_some(), "Extected comment but got other children");
        let cur = text.unwrap();
        let comment = cur.comment();
        assert_eq!(comment.slice_from(0), $expected_text);
    });
)


macro_rules! expect_root_elem(
    ($root:ident, $iter:ident, $expected_name:expr, $expected_ns:expr, $attr_check:expr, $elem_check:expr) => ({
        use xml::NamedNode;
        let name = $root.name();
        assert_eq!(name.slice_from(0), $expected_name);
        let expected_ns : Option<(Option<&str>, &str)> = $expected_ns;
        let prefix = ($root).namespace().map(|x| x.prefix());
        let namespace = ($root).namespace().map(|x| x.href());
        assert_eq!(prefix.as_ref().map(|x| x.as_ref().map(|y| y.slice_from(0))), expected_ns.as_ref().map(|x| x.n0_ref().clone()));
        assert_eq!(namespace.as_ref().map(|x| x.slice_from(0)), expected_ns.as_ref().map(|x| x.n1_ref().clone()));
        {
            let mut $iter = $root.attribute_iter();
            $attr_check;
            assert!($iter.next().is_none(), "Unexpected attribute");
        }
        {
            let mut $iter = $root.children_iter();
            $elem_check;
            assert!($iter.next().is_none(), "Unexpected children");
        }
    })
)

macro_rules! expect_elem(
    ($iter:ident, $expected_name:expr, $expected_ns:expr, $attr_check:expr, $elem_check:expr) => ({
        let next = ($iter).next();
        assert!(next.is_some(), "Expected element but there is no further element");
        let elem = next.unwrap().get_element();
        assert!(elem.is_some(), "Extected element but got other children");
        let cur = elem.unwrap();
        let name = cur.name();
        assert_eq!(name.slice_from(0), $expected_name);
        let expected_ns : Option<(Option<&str>, &str)> = $expected_ns;
        let prefix = cur.namespace().map(|x| x.prefix());
        let namespace = cur.namespace().map(|x| x.href());
        assert_eq!(prefix.as_ref().map(|x| x.as_ref().map(|y| y.slice_from(0))), expected_ns.as_ref().map(|x| x.n0_ref().clone()));
        assert_eq!(namespace.as_ref().map(|x| x.slice_from(0)), expected_ns.as_ref().map(|x| x.n1_ref().clone()));        
        {
            let mut $iter = cur.attribute_iter();
            $attr_check;
            assert!($iter.next().is_none(), "Unexpected attribute");
        }
        {
            let mut $iter = cur.children_iter();
            $elem_check;
            assert!($iter.next().is_none(), "Unexpected children");
        }
    })
)


macro_rules! expect_text(
    ($iter:ident, $expected_text:expr) => ({
        use xml::TextNode;
        let next = ($iter).next();
        assert!(next.is_some(), "Expected text but there is no further element");
        let text = next.unwrap().get_text();
        assert!(text.is_some(), "Extected text but got other children");
        let cur = text.unwrap();
        let content = cur.content();
        assert_eq!(content.slice_from(0), $expected_text);
    });
)

fn read_memory(xml: &[u8]) -> Option<xml::Document> {
    let mut reader = std::io::mem::BufReader::new(xml);
    xml::Document::read(&mut reader)
}

#[test]
fn test_simple_fail() {
    let xml = "<?xml version=\"1.0\"> <test />".as_bytes();
    let doc = read_memory(xml);
    assert!(doc.is_none());
}

#[test]
fn test_simple_parse() {
    let xml = "<?xml version=\"1.0\"?> <test />".as_bytes();
    let doc = read_memory(xml).unwrap();
    let root = doc.get_root_element().unwrap();
    expect_root_elem!(root, iter, "test", None, {}, {});
}

#[test]
fn test_subelements() {
    let xml = "<?xml version=\"1.0\"?> <test> <![CDATA[test]]><a> <b></b>aaa<!-- comment --></a><c/></test>".as_bytes();
    let doc = read_memory(xml).unwrap();
    let root = doc.get_root_element().unwrap();
    expect_root_elem!(root, iter, "test", None, {}, {
        expect_text!(iter, " ");
        expect_cdata!(iter, "test");
        expect_elem!(iter, "a", None, {}, {
            expect_text!(iter, " ");
            expect_elem!(iter, "b", None, {}, {});
            expect_text!(iter, "aaa");
            expect_comment!(iter, " comment ");
        });
        expect_elem!(iter, "c", None, {}, {});
    });
}

#[test]
fn test_attributes() {
    let xml = "<?xml version=\"1.0\"?> <test a=\"b\"> <![CDATA[test]]><a> <b test=\"a\" test2=\"c\"></b>aaa<!-- comment --></a><c/></test>".as_bytes();
    let doc = read_memory(xml).unwrap();
    let root = doc.get_root_element().unwrap();
    expect_root_elem!(root, iter, "test", None, {
        expect_attribute!(iter, "a", None, "b", {
            expect_text!(iter, "b");
        });
    },{
        expect_text!(iter, " ");
        expect_cdata!(iter, "test");
        expect_elem!(iter, "a", None, {}, {
            expect_text!(iter, " ");
            expect_elem!(iter, "b", None, {
                expect_attribute!(iter, "test", None, "a", {
                    expect_text!(iter, "a");
                });
                expect_attribute!(iter, "test2", None, "c", {
                    expect_text!(iter, "c");
                });
            },{});
            expect_text!(iter, "aaa");
            expect_comment!(iter, " comment ");
        });
        expect_elem!(iter, "c", None, {}, {});
    });
}

#[test]
fn test_ns() {
    let xml = "<?xml version=\"1.0\"?><h:html xmlns:h=\"http://www.w3.org/TR/html4/\"><head xmlns=\"http://www.w3.org/TR/html4/\"><meta /></head><h:body xml:lang=\"en\"><my:elem /></h:body></h:html>".as_bytes();
    let doc = read_memory(xml).unwrap();
    let root = doc.get_root_element().unwrap();
    let html4 = "http://www.w3.org/TR/html4/";
    expect_root_elem!(root, iter, "html", Some((Some("h"), html4)), {}, {
        expect_elem!(iter, "head", Some((None, html4)), {}, {
            expect_elem!(iter, "meta", Some((None, html4)), {}, {});
        });
        expect_elem!(iter, "body", Some((Some("h"), html4)), {
            expect_attribute!(iter, "lang", Some((Some("xml"), "http://www.w3.org/XML/1998/namespace")), "en", {
                expect_text!(iter, "en");
            });
        }, {
            expect_elem!(iter, "my:elem", None, {}, {});
        });
    });
}

#[test]
fn test_write() {
    use std::io::Decorator;
    let xml = "<?xml version=\"1.0\"?>\n<h:html xmlns:h=\"http://www.w3.org/TR/html4/\"><head xmlns=\"http://www.w3.org/TR/html4/\"><meta/></head><h:body xml:lang=\"en\"><my:elem/></h:body></h:html>\n".as_bytes();
    let doc = read_memory(xml).unwrap();
    let mut writer = std::io::mem::MemWriter::new();
    doc.write(&mut writer);
    assert_eq!(std::str::from_utf8(writer.inner_ref().as_slice()), std::str::from_utf8(xml))
}

#[test]
fn test_read_condition() {
    use std::io::{BrokenPipe,IoError,io_error};
    struct MyReader<'t> {
        buf: &'t [u8],
        pos: uint,
        error: &'t IoError
    }
    impl<'t> MyReader<'t> {
        fn new(buf: &'t [u8], error: &'t IoError) -> MyReader<'t> {
            MyReader {
                buf: buf,
                pos: 0,
                error: error
            }
        }
    }
    impl<'t> Reader for MyReader<'t> {
        fn read(&mut self, buf: &mut [u8]) -> Option<uint> {
            if self.eof() {return None;}
            let write_len = std::num::min(buf.len(), self.buf.len() - self.pos);
            {
                let input = self.buf.slice(self.pos, self.pos + write_len);
                let output = buf.mut_slice(0, write_len);
                assert_eq!(input.len(), output.len());
                std::vec::bytes::copy_memory(output, input);
            }
            self.pos += write_len;
            assert!(self.pos <= self.buf.len());
            if (self.pos == self.buf.len()) {
                let error = IoError {
                    kind: self.error.kind,
                    desc: self.error.desc,
                    detail: self.error.detail.clone()
                };
                io_error::cond.raise(error);
            }
            return Some(write_len);
        }
        fn eof(&mut self) -> bool { self.pos == self.buf.len() }
    }
    let xml = "<?xml version=\"1.0\"?>\n<h:html xmlns:h=\"http://www.w3.org/TR/html4/\"><head xmlns=\"http://www.w3.org/TR/html4/\">".as_bytes();
    let error = IoError {kind: BrokenPipe, desc: "Test error", detail: None};
    let mut reader = MyReader::new(xml, &error);
    let mut thrown = false;
    let trap = io_error::cond.trap(|err| {
        assert_eq!(err.kind, error.kind);
        assert_eq!(err.desc, error.desc);
        assert_eq!(err.detail, error.detail);
        thrown = true;
    });
    trap.inside(|| {
        xml::Document::read(&mut reader)
    });
    assert!(thrown);
}

#[test]
fn test_write_condition() {
    use std::io::{OtherIoError,io_error};
    let xml = "<?xml version=\"1.0\"?>\n<h:html xmlns:h=\"http://www.w3.org/TR/html4/\"><head xmlns=\"http://www.w3.org/TR/html4/\"><meta/></head><h:body xml:lang=\"en\"><my:elem/></h:body></h:html>\n".as_bytes();
    let doc = read_memory(xml).unwrap();
    let mut out = std::vec::with_capacity(20);
    let mut thrown = false;
    let size = {
        let mut writer = std::io::mem::BufWriter::new(out);
        io_error::cond.trap(|err| {
            assert_eq!(err.kind, OtherIoError);
            thrown = true;
        }).inside(|| {
            doc.write(&mut writer);
        });
        writer.tell() as uint
    };
    assert_eq!(out.slice_to(size), xml.slice_to(size));
}
