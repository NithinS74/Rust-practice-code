pub fn prim() {
    let edges = vec![
        (1, 15, 18.3),
        (2, 10, 11.7),
        (4, 8, 19.9),
        (4, 29, 16.6),
        (5, 24, 11.4),
        (5, 26, 16.0),
        (6, 13, 5.0),
        (7, 22, 15.4),
        (8, 30, 16.0),
        (9, 14, 8.6),
        (9, 18, 19.3),
        (9, 20, 17.9),
        (10, 15, 2.5),
        (10, 16, 15.9),
        (10, 25, 15.0),
        (11, 16, 13.9),
        (11, 23, 0.6),
        (11, 25, 8.8),
        (12, 17, 18.2),
        (13, 19, 9.3),
        (13, 22, 6.1),
        (14, 21, 7.5),
        (14, 29, 7.3),
        (16, 27, 3.1),
        (18, 26, 18.8),
        (19, 30, 10.6),
        (20, 25, 4.4),
        (20, 27, 8.0),
        (21, 25, 13.9),
        (21, 27, 6.2),
        (22, 25, 10.9),
        (23, 26, 19.7),
        (24, 26, 9.8),
        (24, 28, 5.4),
        (27, 30, 2.4),
    ];

    for &(u, v, weight) in &edges {
        adj_list[u].push((v, weight));
        adj_list[v].push((u, weight));
    }

    let mut mst_edges: Vec<(usize, usize, f64)> = Vec::new();

    visited[1] = true;
    for &(neighbor, weight) in &adj_list[1] {
        candidate_edges.push((weight, 1, neighbor));
    }

    while !candidate_edges.is_empty() {
        let mut min_index = 0;
        for i in 1..candidate_edges.len() {
            if candidate_edges[i].0 < candidate_edges[min_index].0 {
                min_index = i;
            }
        }

        let (weight, u, v) = candidate_edges.remove(min_index);

        if visited[v] {
            continue;
        }

        visited[v] = true;
        mst_edges.push((u, v, weight));

        for &(neighbor, edge_weight) in &adj_list[v] {
            if !visited[neighbor] {
                candidate_edges.push((edge_weight, v, neighbor));
            }
        }
    }

    let mut total = 0.0;
    println!("Edges in the Minimum Spanning Tree:");
    for (u, v, weight) in mst_edges {
        println!("Edge: {} - {}, Weight: {}", u, v, weight);
        total += weight;
    }
    println!("total weight {total}");
}
