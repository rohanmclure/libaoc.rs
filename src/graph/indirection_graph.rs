use std::{collections::HashMap, hash::Hash};

use super::{Graph, graph::ExtendableGraph};

struct IndirectionGraph<V, E> {
    start: Option<V>,
    nodes: HashMap<V, GNode<V, E>>
}

struct GNode<V, E> {
    neighbours: Vec<(E, *const V)>
}

impl<V,E> IndirectionGraph<V, E> {
    pub fn new() -> Self {
        IndirectionGraph {
            start: None,
            nodes: HashMap::new() 
        }
    }

    pub fn with_start<'a>(mut self, v: &'a V) -> Self
    where
        V: Copy+Eq+Hash
    {
        if let Some(_) = self.start {
            panic!("Should not reassign start.");
        }

        self.start = Some(*v);
        if !self.nodes.contains_key(v) {
            self.nodes.insert(*v, GNode { neighbours: Vec::new() });
        }
        self
    }

    pub fn with_edge<'a>(mut self, v: &'a V, u: &'a V, e: E) -> Self
    where
        V: Copy+Eq+Hash
    {
        self.push_edge(v, u, e);
        self
    }
}

impl<'a, V: 'a, E: 'a> ExtendableGraph<'a, V, E> for IndirectionGraph<V, E>
where
    V: Copy+Eq+Hash
{
    fn push_edge(&'a mut self,
                 v: &'a V, u: &'a V, e: E) {

        for w in vec![u, v].into_iter() {
            if !self.nodes.contains_key(w) {
                self.nodes.insert(*w, GNode { neighbours: Vec::new() });
            }
        }

        let v_ptr = self.nodes.get_key_value(v)
                              .unwrap().0
                              as *const V;

        self.nodes.get_mut(v).unwrap()
                  .neighbours.push((e, v_ptr));
    }
}

impl<'a, V: 'a, E: 'a> Graph<'a, V, E> for IndirectionGraph<V, E>
where
    V: Copy+Eq+Hash
{
    fn get_neighbours(&'a self, v: &'a V)
                   -> impl Iterator<Item=&'a V> {
        self.nodes[v]
            .neighbours.iter()
            .map(|(_e, v_ptr)| unsafe{ &**v_ptr })
    }

    fn get_arc(&'a self, v: &'a V, u: &'a V) -> Option<&'a E> {
        let mut ret = None;
        for (e, v_ptr) in self.nodes[v]
                              .neighbours.iter() {
            if unsafe{&**v_ptr} == u {
                ret = Some(e);
            }
        }
        ret
    }

    fn vertices_iter(&'a self) -> impl Iterator<Item=&'a V> {
        self.nodes.iter().map(|(v, _n)| v)
    }

    fn get_first(&'a self) -> &'a V {
        self.start.as_ref().unwrap()
    }
}
