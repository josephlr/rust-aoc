use crate::util::{Answer, ByWhitespace, Error, Result};

#[derive(Default)]
struct Node {
    nodes: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn make(mut inputs: impl Iterator<Item = usize>) -> Result<Self> {
        let node = Self::new(&mut inputs).ok_or(Error::Custom("Insufficient Input"))?;
        if inputs.next().is_some() {
            return Err(Error::Custom("Extra Input"));
        }
        Ok(node)
    }

    fn new(iter: &mut impl Iterator<Item = usize>) -> Option<Self> {
        let num_nodes = iter.next()?;
        let metadata_len = iter.next()?;

        let mut node = Self::default();
        for _ in 0..num_nodes {
            node.nodes.push(Self::new(iter)?);
        }
        for _ in 0..metadata_len {
            node.metadata.push(iter.next()?);
        }
        Some(node)
    }

    fn metadata_sum(&self) -> usize {
        let s1: usize = self.metadata.iter().cloned().sum();
        let s2: usize = self.nodes.iter().map(|n| n.metadata_sum()).sum();
        s1 + s2
    }
    fn value(&self) -> usize {
        if self.nodes.is_empty() {
            return self.metadata.iter().cloned().sum();
        }
        self.metadata
            .iter()
            .flat_map(|&idx| Some(self.nodes.get(idx - 1)?.value()))
            .sum()
    }
}

pub struct Sum;
impl Answer for Sum {
    type Input = ByWhitespace<usize>;
    type Output = usize;
    fn ans(&self, inputs: impl Iterator<Item = usize>) -> Result<Self::Output> {
        Ok(Node::make(inputs)?.metadata_sum())
    }
}

pub struct Value;
impl Answer for Value {
    type Input = ByWhitespace<usize>;
    type Output = usize;
    fn ans(&self, inputs: impl Iterator<Item = usize>) -> Result<Self::Output> {
        Ok(Node::make(inputs)?.value())
    }
}
