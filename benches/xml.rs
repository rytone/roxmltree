#![allow(dead_code)]

#[macro_use]
extern crate bencher;

extern crate roxmltree;
extern crate xmlparser;
extern crate xmltree;
extern crate sxd_document;
extern crate elementtree;
extern crate treexml;
extern crate xml;

use std::fs;
use std::env;
use std::io::Read;

use bencher::Bencher;

fn load_string(path: &str) -> String {
    let path = env::current_dir().unwrap().join(path);
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

fn load_data(path: &str) -> Vec<u8> {
    let path = env::current_dir().unwrap().join(path);
    let mut file = fs::File::open(&path).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    buf
}


fn tiny_xmlparser(bencher: &mut Bencher) {
    let text = load_string("fonts.conf");
    bencher.iter(|| {
        for t in xmlparser::Tokenizer::from(text.as_str()) {
            let _ = t.unwrap();
        }
    })
}

fn medium_xmlparser(bencher: &mut Bencher) {
    let text = load_string("medium.svg");
    bencher.iter(|| {
        for t in xmlparser::Tokenizer::from(text.as_str()) {
            let _ = t.unwrap();
        }
    })
}

fn large_xmlparser(bencher: &mut Bencher) {
    let text = load_string("large.plist");
    bencher.iter(|| {
        for t in xmlparser::Tokenizer::from(text.as_str()) {
            let _ = t.unwrap();
        }
    })
}


fn tiny_xmlrs(bencher: &mut Bencher) {
    let text = load_string("fonts.conf");
    bencher.iter(|| {
        for event in xml::EventReader::new(text.as_bytes()) {
            let _ = event.unwrap();
        }
    })
}

fn medium_xmlrs(bencher: &mut Bencher) {
    let text = load_string("medium.svg");
    bencher.iter(|| {
        for event in xml::EventReader::new(text.as_bytes()) {
            let _ = event.unwrap();
        }
    })
}

fn large_xmlrs(bencher: &mut Bencher) {
    let text = load_string("large.plist");
    bencher.iter(|| {
        for event in xml::EventReader::new(text.as_bytes()) {
            let _ = event.unwrap();
        }
    })
}


fn tiny_roxmltree(bencher: &mut Bencher) {
    let text = load_string("fonts.conf");
    bencher.iter(|| roxmltree::Document::parse(&text).unwrap())
}

fn medium_roxmltree(bencher: &mut Bencher) {
    let text = load_string("medium.svg");
    bencher.iter(|| roxmltree::Document::parse(&text).unwrap())
}

fn large_roxmltree(bencher: &mut Bencher) {
    let text = load_string("large.plist");
    bencher.iter(|| roxmltree::Document::parse(&text).unwrap())
}


fn tiny_xmltree(bencher: &mut Bencher) {
    let text = load_string("fonts.conf");
    bencher.iter(|| xmltree::Element::parse(text.as_bytes()).unwrap())
}

fn medium_xmltree(bencher: &mut Bencher) {
    let text = load_string("medium.svg");
    bencher.iter(|| xmltree::Element::parse(text.as_bytes()).unwrap())
}

fn large_xmltree(bencher: &mut Bencher) {
    let text = load_string("large.plist");
    bencher.iter(|| xmltree::Element::parse(text.as_bytes()).unwrap())
}


fn tiny_sdx_document(bencher: &mut Bencher) {
    let text = load_string("fonts.conf");
    bencher.iter(|| sxd_document::parser::parse(&text).unwrap())
}

fn medium_sdx_document(bencher: &mut Bencher) {
    let text = load_string("medium.svg");
    bencher.iter(|| sxd_document::parser::parse(&text).unwrap())
}

fn large_sdx_document(bencher: &mut Bencher) {
    let text = load_string("large.plist");
    bencher.iter(|| sxd_document::parser::parse(&text).unwrap())
}


fn tiny_elementtree(bencher: &mut Bencher) {
    let data = load_data("fonts.conf");
    bencher.iter(|| elementtree::Element::from_reader(&data[..]).unwrap())
}

fn medium_elementtree(bencher: &mut Bencher) {
    let data = load_data("medium.svg");
    bencher.iter(|| elementtree::Element::from_reader(&data[..]).unwrap())
}

fn large_elementtree(bencher: &mut Bencher) {
    let data = load_data("large.plist");
    bencher.iter(|| elementtree::Element::from_reader(&data[..]).unwrap())
}


fn tiny_treexml(bencher: &mut Bencher) {
    let data = load_data("fonts.conf");
    bencher.iter(|| treexml::Document::parse(&data[..]).unwrap())
}

fn medium_treexml(bencher: &mut Bencher) {
    let data = load_data("medium.svg");
    bencher.iter(|| treexml::Document::parse(&data[..]).unwrap())
}

fn large_treexml(bencher: &mut Bencher) {
    let data = load_data("large.plist");
    bencher.iter(|| treexml::Document::parse(&data[..]).unwrap())
}


benchmark_group!(roxmltree, tiny_roxmltree, medium_roxmltree, large_roxmltree);
benchmark_group!(xmltree, tiny_xmltree, medium_xmltree, large_xmltree);
benchmark_group!(sdx, tiny_sdx_document, medium_sdx_document, large_sdx_document);
benchmark_group!(elementtree, tiny_elementtree, medium_elementtree, large_elementtree);
benchmark_group!(treexml, tiny_treexml, medium_treexml, large_treexml);
benchmark_group!(xmlparser, tiny_xmlparser, medium_xmlparser, large_xmlparser);
benchmark_group!(xmlrs, tiny_xmlrs, medium_xmlrs, large_xmlrs);
benchmark_main!(roxmltree, xmltree, sdx, elementtree, treexml, xmlparser, xmlrs);
// benchmark_main!(roxmltree, xmlparser);
