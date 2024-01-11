use crate::types::{FileDescriptor, MessageIndex};
use std::cmp::min;
use std::collections::HashMap;

/// A recursive strongly connected component function
///
/// Uses Tarjan's algorithm
/// https://www.geeksforgeeks.org/tarjan-algorithm-find-strongly-connected-components/
#[allow(clippy::too_many_arguments)]
fn scc(
    vertices: &[MessageIndex],
    desc: &FileDescriptor,
    u: usize,
    count: &mut isize,
    low: &mut [isize],
    disc: &mut [isize],
    stack: &mut Vec<usize>,
    sccs: &mut Vec<Vec<usize>>,
    ids: &HashMap<MessageIndex, usize>,
) {
    disc[u] = *count;
    low[u] = *count;
    *count += 1;
    stack.push(u);

    for &v in vertices[u]
        .get_message(desc)
        .all_fields()
        .filter(|f| !f.boxed && !f.frequency.is_repeated())
        .filter_map(|f| f.typ.message())
        .filter_map(|m| ids.get(m))
    {
        if disc[v] == -1 {
            scc(vertices, desc, v, count, low, disc, stack, sccs, ids);
            low[u] = min(low[u], low[v]);
        } else if stack.contains(&v) {
            low[u] = min(low[u], disc[v]);
        }
    }

    if low[u] == disc[u] {
        let mut scc = Vec::new();
        while let Some(w) = stack.pop() {
            scc.push(w);
            if w == u {
                break;
            }
        }
        sccs.push(scc);
    }
}

impl FileDescriptor {
    fn flatten_messages(&self) -> Vec<MessageIndex> {
        let mut all_msgs = self
            .messages
            .iter()
            .map(|m| m.index.clone())
            .collect::<Vec<_>>();
        let mut vertices = Vec::with_capacity(all_msgs.len());
        while let Some(m) = all_msgs.pop() {
            all_msgs.extend(m.get_message(self).messages.iter().map(|m| m.index.clone()));
            vertices.push(m);
        }
        vertices
    }

    pub fn sccs(&self) -> Vec<Vec<MessageIndex>> {
        let vertices = self.flatten_messages();
        let ids = vertices
            .iter()
            .enumerate()
            .map(|(i, m)| (m.get_message(self).index.clone(), i))
            .collect::<HashMap<_, _>>();
        let mut low = vec![-1; vertices.len()];
        let mut disc = vec![-1; vertices.len()];
        let mut stack: Vec<usize> = Vec::new();
        let mut count = 0isize;
        let mut sccs: Vec<Vec<usize>> = Vec::new();
        for u in 0..vertices.len() {
            if disc[u] == -1 {
                scc(
                    &vertices, self, u, &mut count, &mut low, &mut disc, &mut stack, &mut sccs,
                    &ids,
                );
            }
        }
        sccs.into_iter()
            .map(|scc| scc.into_iter().map(|i| vertices[i].clone()).collect())
            .collect()
    }
}
