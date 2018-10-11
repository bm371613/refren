use std::io;

pub trait Transform {
    fn transform(&self, reader: &mut io::Read, writer: &mut io::Write) -> io::Result<()>;
}

mod aho_corasick {
    use std::collections::{HashMap, VecDeque};
    use std::io;
    use std::io::BufRead;
    use std::iter::{once, FromIterator};
    use std::ops::Deref;

    use super::*;

    type NodeId = usize;

    #[derive(Debug)]
    struct Node {
        depth: u8,
        parent: Option<NodeId>,
        children: HashMap<char, NodeId>,
        label: Option<char>,
        in_dict: bool,
        suffix_link: Option<NodeId>,
        in_dict_suffix_link: Option<NodeId>,
    }

    #[derive(Debug)]
    struct Tree {
        nodes: Vec<Node>,
    }

    static ROOT_ID: NodeId = 0;

    impl Tree {
        fn new<'a, I: Iterator<Item = &'a str>>(patterns: I) -> Self {
            let mut result = Tree {
                nodes: vec![Node {
                    depth: 0,
                    parent: None,
                    children: HashMap::new(),
                    label: None,
                    in_dict: false,
                    suffix_link: None,
                    in_dict_suffix_link: None,
                }],
            };
            for pattern in patterns {
                result.extend(pattern.chars());
            }
            result.set_links();
            result
        }
        fn spawn_chid(&mut self, parent_id: NodeId, label: char) -> NodeId {
            let parent_depth = self.nodes[parent_id].depth;
            self.nodes.push(Node {
                depth: parent_depth + 1,
                parent: Some(parent_id),
                children: HashMap::new(),
                label: Some(label),
                in_dict: false,
                suffix_link: None,
                in_dict_suffix_link: None,
            });
            let child_id = self.nodes.len() - 1;
            self.nodes[parent_id].children.insert(label, child_id);
            child_id
        }
        fn extend<I: Iterator<Item = char>>(&mut self, chars: I) {
            let mut node_id = ROOT_ID;
            let mut create = false;
            for c in chars {
                if !create && !self.nodes[node_id].children.contains_key(&c) {
                    create = true;
                }
                node_id = if create {
                    self.spawn_chid(node_id, c)
                } else {
                    *self.nodes[node_id].children.get(&c).unwrap()
                }
            }
            self.nodes[node_id].in_dict = true;
        }
        fn set_links(&mut self) {
            let mut queue = VecDeque::with_capacity(self.nodes.len());
            queue.push_back(ROOT_ID);
            while let Some(node_id) = queue.pop_front() {
                queue.extend(self.nodes[node_id].children.values());

                self.nodes[node_id].suffix_link = match self.nodes[node_id].parent {
                    None => None,
                    Some(parent_id) => {
                        let label = self.nodes[node_id].label.unwrap();
                        let mut traversing_id = parent_id;
                        let mut suffix_link = ROOT_ID;
                        while let Some(traversing_id_suffix_link) =
                            self.nodes[traversing_id].suffix_link
                        {
                            traversing_id = traversing_id_suffix_link;
                            if let Some(&traversing_id_child) =
                                self.nodes[traversing_id].children.get(&label)
                            {
                                suffix_link = traversing_id_child;
                                break;
                            }
                        }
                        Some(suffix_link)
                    }
                };

                self.nodes[node_id].in_dict_suffix_link = match self.nodes[node_id].suffix_link {
                    None => None,
                    Some(suffix_link) => if self.nodes[suffix_link].in_dict {
                        Some(suffix_link)
                    } else {
                        self.nodes[suffix_link].in_dict_suffix_link
                    },
                }
            }
        }
    }

    pub struct Transformer {
        map: HashMap<String, String>,
        tree: Tree,
    }

    impl Transformer {
        pub fn from_map<I: IntoIterator<Item = (String, String)>>(map: I) -> Self {
            let map = HashMap::from_iter(map);
            let tree = Tree::new(map.keys().map(String::deref));
            Transformer { map, tree }
        }
    }

    impl Transform for Transformer {
        fn transform(&self, reader: &mut io::Read, writer: &mut io::Write) -> io::Result<()> {
            let mut line = String::new();
            let mut buf_reader = io::BufReader::new(reader);
            while buf_reader.read_line(&mut line)? > 0 {
                let char_ixs: Vec<usize> = line
                    .char_indices()
                    .map(|(i, _)| i)
                    .chain(once(line.len()))
                    .collect();
                let mut output_position = 0;
                let mut node_id = ROOT_ID;
                for (i, c) in line.chars().enumerate() {
                    loop {
                        if let Some(&child_id) = self.tree.nodes[node_id].children.get(&c) {
                            node_id = child_id;
                            break;
                        }
                        match self.tree.nodes[node_id].suffix_link {
                            Some(suffix_link) => {
                                node_id = suffix_link;
                            }
                            None => break,
                        }
                    }
                    let in_dict_node_maybe = if self.tree.nodes[node_id].in_dict {
                        Some(node_id)
                    } else {
                        self.tree.nodes[node_id].in_dict_suffix_link
                    };
                    if let Some(in_dict_node) = in_dict_node_maybe {
                        let depth = self.tree.nodes[in_dict_node].depth as usize;
                        let start_position = char_ixs[i + 1 - depth];
                        let end_position = char_ixs[i + 1];
                        writer.write(line[output_position..start_position].as_bytes())?;
                        writer.write(
                            self.map
                                .get(&line[start_position..end_position])
                                .unwrap()
                                .as_bytes(),
                        )?;
                        output_position = end_position;
                        node_id = ROOT_ID;
                    }
                }
                writer.write(line[output_position..].as_bytes())?;
                line.clear();
            }
            Ok(())
        }
    }

    #[test]
    fn test_utf8() {
        let transformer = Transformer::from_map(vec![
            ("aa".to_owned(), "aaa".to_owned()),
            (":)".to_owned(), "😊".to_owned()),
            ("😺".to_owned(), "a cat".to_owned()),
        ]);
        let src: Vec<u8> = "Whaaat? :)\n😺".bytes().collect();
        let mut dst: Vec<u8> = Vec::new();
        assert_eq!(transformer.transform(&mut &src[..], &mut dst).unwrap(), ());
        assert_eq!(String::from_utf8(dst).unwrap(), "Whaaaat? 😊\na cat");
    }

}

pub fn default_transformer<I: IntoIterator<Item = (String, String)>>(map: I) -> Box<Transform> {
    Box::new(aho_corasick::Transformer::from_map(map))
}
