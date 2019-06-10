//! dead-simple module to write svg. It's not efficient nor pretty, but it gets the job done.

use std::fmt::{Display, Formatter};

use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Document {
    children: Vec<Element>,
    viewbox: (f64, f64, f64, f64),
}

#[derive(Debug, Clone)]
pub struct Element {
    tag: &'static str,
    attributes: BTreeMap<&'static str, String>,
}

impl Document {
    pub fn new(viewbox: (f64, f64, f64, f64)) -> Self {
        Document {
            viewbox,
            children: vec![],
        }
    }

    pub fn push(mut self, element: Element) -> Self {
        self.children.push(element);
        self
    }
}

impl Element {
    pub fn path(paths: impl IntoIterator<Item = Vec<(f64, f64)>>) -> Self {
        let el = Element::new("path");

        let mut d = String::new();
        for (i, path) in paths.into_iter().filter(|c| !c.is_empty()).enumerate() {
            if i > 0 {
                d += " ";
            }

            d += &format!("M {},{} ", path[0].0, path[0].1);
            for pt in path.into_iter().skip(1) {
                d += &format!("L {},{} ", pt.0, pt.1);
            }
            d += "Z";
        }

        el.set("d", d)
    }

    pub fn polyline(v: impl IntoIterator<Item = (f64, f64)>) -> Self {
        let el = Element::new("polyline");

        el.set(
            "points",
            v.into_iter()
                .map(|(x, y)| format!("{},{}", x, y))
                .collect::<Vec<_>>()
                .join(" "),
        )
    }

    pub fn rect((ox, oy): (f64, f64), (width, height): (f64, f64)) -> Self {
        let el = Element::new("rect");

        el.set("x", ox.to_string())
            .set("y", oy.to_string())
            .set("width", width.to_string())
            .set("height", height.to_string())
    }

    pub fn fill(self, color: impl Into<String>) -> Self {
        self.set("fill", color)
    }

    pub fn set(mut self, attr: &'static str, value: impl Into<String>) -> Self {
        self.attributes.insert(attr, value.into());
        self
    }

    fn new(tag: &'static str) -> Self {
        Element {
            tag,
            attributes: BTreeMap::new(),
        }
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(
            f,
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="{} {} {} {}">"#,
            self.viewbox.0, self.viewbox.1, self.viewbox.2, self.viewbox.3
        )?;

        for e in &self.children {
            writeln!(f, "{}", e)?;
        }

        write!(f, "</svg>")?;

        Ok(())
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "<{} ", self.tag)?;

        for (a, v) in &self.attributes {
            write!(f, r#"{}="{}" "#, a, v)?;
        }

        write!(f, "/>")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let doc = Document::new((0.0, 0.0, 200.0, 200.0))
            .push(Element::rect((0.0, 0.0), (200.0, 200.0)).fill("red"))
            .push(Element::polyline(vec![
                (10.0, 20.0),
                (50.0, 20.0),
                (50.0, 50.0),
                (10.0, 50.0),
            ]))
            .push(
                Element::polyline(vec![
                    (160.0, 20.0),
                    (180.0, 60.0),
                    (140.0, 30.0),
                    (160.0, 20.0),
                ])
                .fill("none")
                .set("stroke", "black"),
            )
            .push(Element::path(vec![
                vec![(0.0, 10.0), (20.0, 30.0), (20.0, 50.0), (0.0, 50.0)],
                vec![(20.0, 20.0), (40.0, 40.0), (20.0, 0.0)],
            ]))
            .push(Element::path(vec![vec![], vec![(0.0, 10.0)]]));

        assert_eq!(
            doc.to_string(),
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 200 200">
<rect fill="red" height="200" width="200" x="0" y="0" />
<polyline points="10,20 50,20 50,50 10,50" />
<polyline fill="none" points="160,20 180,60 140,30 160,20" stroke="black" />
<path d="M 0,10 L 20,30 L 20,50 L 0,50 Z M 20,20 L 40,40 L 20,0 Z" />
<path d="M 0,10 Z" />
</svg>"#
        );
    }
}
