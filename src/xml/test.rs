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

macro_rules! expect_cdata(
    ($iter:ident, $expected_text:expr) => ({
        use xml::TextNode;
        let next = ($iter).next();
        assert!(next.is_some(), "Expected CDATA section but there is no further element");
        let text = next.unwrap().get_cdata();
        assert!(text.is_some(), "Extected CDATA section but got other children");
        let cur = text.unwrap();
        assert_eq!(cur.content(), $expected_text);
    });
)

macro_rules! expect_root_elem(
    ($root:expr, $iter:ident, $expected_name:expr, $elem_check:expr) => ({
        assert_eq!($root.name(), $expected_name);
        {
            let mut $iter = $root.children_iter();
            $elem_check;
            assert!($iter.next().is_none(), "Unexpected children");
        }
    })
)

macro_rules! expect_elem(
    ($iter:ident, $expected_name:expr, $elem_check:expr) => ({
        let next = ($iter).next();
        assert!(next.is_some(), "Expected element but there is no further element");
        let elem = next.unwrap().get_element();
        assert!(elem.is_some(), "Extected element but got other children");
        let cur = elem.unwrap();
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
        assert_eq!(cur.content(), $expected_text);
    });
)

#[test]
fn test_simple_fail() {
    let xml = "<?xml version=\"1.0\"> <test />".as_bytes();
    let doc = xml::read_memory(xml);
    assert!(doc.is_none());
}

#[test]
fn test_simple_parse() {
    let xml = "<?xml version=\"1.0\"?> <test />".as_bytes();
    let doc = xml::read_memory(xml).unwrap();
    let root = doc.get_root_element().unwrap();
    expect_root_elem!(root, iter, ~"test", {});
}

#[test]
fn test_subelements() {
    let xml = "<?xml version=\"1.0\"?> <test> <![CDATA[test]]><a> <b></b>aaa</a><c/></test>".as_bytes();
    let doc = xml::read_memory(xml).unwrap();
    let root = doc.get_root_element().unwrap();
    expect_root_elem!(root, iter, ~"test", {
        expect_text!(iter, ~" ");
        expect_cdata!(iter, ~"test");
        expect_elem!(iter, ~"a", {
            expect_text!(iter, ~" ");
            expect_elem!(iter, ~"b", {});
            expect_text!(iter, ~"aaa");
        });
        expect_elem!(iter, ~"c", {});
    });
}

