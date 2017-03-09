use std::borrow::Cow;
use html5ever_atoms::{ QualName, LocalName, Prefix };
use tendril::StrTendril;
use xml5ever::tree_builder::{ TreeSink, XmlTreeBuilderOpts, NodeOrText };
use xml5ever::tokenizer::{ XmlTokenizerOpts, QName, Attribute };
use xml5ever::driver::{ XmlParser, XmlParseOpts, parse_document };
use kuchiki::NodeRef;


#[derive(Default)]
pub struct ParseOpts {
    pub tokenizer: XmlTokenizerOpts,
    pub tree_builder: XmlTreeBuilderOpts,
    pub on_parse_error: Option<Box<FnMut(Cow<'static, str>)>>,
}

#[inline]
pub fn parse_xml() -> XmlParser<Sink> {
    parse_xml_with_options(ParseOpts::default())
}

pub fn parse_xml_with_options(opts: ParseOpts) -> XmlParser<Sink> {
    let sink = Sink {
        document_node: NodeRef::new_document(),
        on_parse_error: opts.on_parse_error
    };
    let xml5opts = XmlParseOpts {
        tokenizer: opts.tokenizer,
        tree_builder: opts.tree_builder,
    };

    parse_document(sink, xml5opts)
}


pub struct Sink {
    document_node: NodeRef,
    on_parse_error: Option<Box<FnMut(Cow<'static, str>)>>,
}


impl TreeSink for Sink {
    type Handle = NodeRef;
    type Output = NodeRef;

    fn finish(self) -> Self::Output { self.document_node }

    #[inline]
    fn parse_error(&mut self, message: Cow<'static, str>) {
        if let Some(ref mut handler) = self.on_parse_error {
            handler(message)
        }
    }

    #[inline]
    fn get_document(&mut self) -> NodeRef {
        self.document_node.clone()
    }

    #[inline]
    fn elem_name(&self, target: &NodeRef) -> QName {
        qualname_to_qname(target.as_element().unwrap().name.clone())
    }

    #[inline]
    fn create_element(&mut self, name: QName, attrs: Vec<Attribute>) -> NodeRef {
        let attrs = attrs.into_iter().map(|Attribute { name, value }| (qname_to_qualname(name), value.into()));
        NodeRef::new_element(qname_to_qualname(name), attrs)
    }

    #[inline]
    fn create_comment(&mut self, text: StrTendril) -> NodeRef {
        NodeRef::new_comment(text)
    }

    /// FIXME HACK, kuchiki need support PI.
    #[inline]
    fn create_pi(&mut self, target: StrTendril, data: StrTendril) -> NodeRef {
        use std::ops::Deref;

        let name = QualName::new(ns!(), LocalName::from(target.deref()));

        NodeRef::new_element(name, Some((
            QualName::new(ns!(), LocalName::from("data")),
            data.to_string()
        )))
    }

    #[inline]
    fn append(&mut self, parent: NodeRef, child: NodeOrText<NodeRef>) {
        match child {
            NodeOrText::AppendNode(node) => parent.append(node),
            NodeOrText::AppendText(text) => {
                if let Some(last_child) = parent.last_child() {
                    if let Some(existing) = last_child.as_text() {
                        existing.borrow_mut().push_str(&text);
                        return
                    }
                }
                parent.append(NodeRef::new_text(text))
            }
        }
    }

    #[inline]
    fn append_doctype_to_document(&mut self, name: StrTendril, public_id: StrTendril,
                                  system_id: StrTendril) {
        self.document_node.append(NodeRef::new_doctype(name, public_id, system_id))
    }

    #[inline]
    fn mark_script_already_started(&mut self, _node: NodeRef) {
        // FIXME: Is this useful outside of a browser?
    }
}

fn qualname_to_qname(name: QualName) -> QName {
    let QualName { ns, local } = name;
    let (prefix, local) = local.split_at(local.find(':').unwrap_or(0));

    QName {
        prefix: Prefix::from(prefix),
        local: LocalName::from(local),
        namespace_url: ns
    }
}

fn qname_to_qualname(name: QName) -> QualName {
    let QName { prefix, local, namespace_url } = name;

    QualName {
        ns: namespace_url,
        local: if prefix.is_empty() {
            local
        } else {
            LocalName::from(format!("{}:{}", prefix, local))
        }
    }
}
