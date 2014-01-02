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

extern mod xml;

#[test]
fn test_simple_parse() {
    let xml = "<?xml version=\"1.0\"?> <test />".as_bytes();
    let doc = xml::read_memory(xml).unwrap();
    let root = doc.get_root_element().unwrap();
    assert_eq!(root.name(), ~"test");
    assert!(root.children_iter().next().is_none());
}

#[test]
fn test_simple_fail() {
    let xml = "<?xml version=\"1.0\"> <test />".as_bytes();
    let doc = xml::read_memory(xml);
    assert!(doc.is_none());
}

#[test]
fn test_subelements() {
    let xml = "<?xml version=\"1.0\"?> <test> <a> <b></b>aaa</a><c/></test>".as_bytes();
    let doc = xml::read_memory(xml).unwrap();
    let root = doc.get_root_element().unwrap();
    assert_eq!(root.name(), ~"test");
    let mut iter = root.children_iter();
    {
        let cur = iter.next().unwrap().get_text().unwrap();
        assert_eq!(cur.content(), ~" ");
    }
    {
        let cur = iter.next().unwrap().get_element().unwrap();
        assert_eq!(cur.name(), ~"a");
        let mut iter = cur.children_iter();
        {
            let cur = iter.next().unwrap().get_text().unwrap();
            assert_eq!(cur.content(), ~" ");
        }
        {
            let cur = iter.next().unwrap().get_element().unwrap();
            assert_eq!(cur.name(), ~"b");
            assert!(cur.children_iter().next().is_none());
        }
        {
            let cur = iter.next().unwrap().get_text().unwrap();
            assert_eq!(cur.content(), ~"aaa");            
        }
        assert!(iter.next().is_none());
    }
    {
        let cur = iter.next().unwrap().get_element().unwrap();
        assert_eq!(cur.name(), ~"c");
        assert!(cur.children_iter().next().is_none());
    }
    assert!(iter.next().is_none());
}
