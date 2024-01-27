use crate::custom_error::AocError;

use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;
use tracing::info;

fn mincut_edmond_karp(
    g: &HashMap<&str, HashSet<&str>>,
    s: &str,
    t: &str,
) -> Option<usize> {
    let mut flow = HashMap::new();
    let mut f = 0;

    while f <= 3 {
        let mut pred = HashMap::new();
        let mut queue = VecDeque::from_iter([s]);
        let mut seen_vertices = 0;

        while let Some(cur) = queue.pop_front() {
            if pred.contains_key(t) {
                break;
            }

            for &next in &g[cur] {
                if next != s
                    && !pred.contains_key(next)
                    && *flow.get(&(cur, next)).unwrap_or(&0)
                        < 1
                {
                    pred.insert(next, cur);
                    queue.push_back(next);
                }
            }

            seen_vertices += 1;
        }

        if !pred.contains_key(t) {
            if seen_vertices == g.len() {
                return None;
            }

            return Some(
                seen_vertices * (g.len() - seen_vertices),
            );
        }

        let mut df = i64::MAX;
        let mut cur = t;

        while let Some(&prev) = pred.get(cur) {
            df = df.min(
                1 - *flow.get(&(prev, cur)).unwrap_or(&0),
            );
            cur = prev;
        }

        let mut cur = t;
        while let Some(&prev) = pred.get(cur) {
            *flow.entry((prev, cur)).or_default() += df;
            *flow.entry((cur, prev)).or_default() -= df;
            cur = prev;
        }

        f += df;
    }

    None
}

#[tracing::instrument(skip(input))]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut graph = HashMap::<_, HashSet<_>>::new();
    for l in input.lines() {
        let (a, rest) = l.split_once(": ").unwrap();
        for b in rest.split(' ') {
            graph.entry(a).or_default().insert(b);
            graph.entry(b).or_default().insert(a);
        }
    }

    info!(?graph);

    let &start = graph.keys().next().unwrap();
    Ok(graph
        .keys()
        .find_map(|k| mincut_edmond_karp(&graph, start, k))
        .unwrap()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        assert_eq!("54", process(input)?);
        Ok(())
    }
}
