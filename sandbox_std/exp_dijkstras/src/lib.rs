//! Теория: как работает алгоритм Дейкстры
//! Алгоритм Дейкстры находит кратчайшие пути от одной вершины графа до всех остальных вершин.
//! Работает только для графов без рёбер отрицательного веса.
//!
//! Основные шаги:
//!
//! 1. Инициализация:
//!    - присвоить начальной вершине расстояние 0;
//!    - всем остальным вершинам — расстояние ∞;
//!    - создать множество непосещённых вершин.
//!
//! 2. Основной цикл:
//!    - выбрать непосещённую вершину с наименьшим расстоянием (на первом шаге — начальная вершина);
//!    - для каждого соседа этой вершины рассчитать новое расстояние как сумму текущего расстояния до вершины и веса ребра;
//!    - если новое расстояние меньше известного — обновить;
//!    - пометить вершину как посещённую.
//!
//! 3. Повторять шаг 2, пока не будут посещены все вершины или пока не будет достигнута целевая вершина.
//!
//! Сложность: O(V²) для простой реализации, O((V+E)logV) с использованием кучи (где V — количество вершин, E — количество рёбер).

use std::{cmp::Reverse, collections::BinaryHeap};

/// Формат - Vec<(Название города, Vec<(Индекс вершины соседа, расстояние)>)>
type CityMap = Vec<(String, Vec<(usize, usize)>)>;

/// Функция реализует алгоритм Дейкстры для нахождения кратчайшего пути
/// от начального города (указывается индексом) до конечного города (указывается индексом).
/// Возвращает вектор индексов городов, составляющих кратчайший путь.
pub fn dijkstra(cities: &CityMap, start_idx: usize, end: usize) -> Vec<usize> {
    let n = cities.len();
    let mut dist: Vec<usize> = vec![usize::MAX; n];
    let mut prev: Vec<Option<usize>> = vec![None; n]; // для восстановления пути
    let mut visited = vec![false; n];
    let mut heap = BinaryHeap::new();

    // Инициализация: расстояние от начальной вершины до самой себя = 0
    dist[start_idx] = 0;
    heap.push(Reverse((0, start_idx)));

    while let Some(Reverse((heap_weight, heap_city))) = heap.pop() {
        // Пропускаем уже обработанные вершины
        if visited[heap_city] {
            continue;
        }
        visited[heap_city] = true;

        // Если достигли конечной точки — можно завершать
        if heap_city == end {
            break;
        }

        // Обход соседей текущей вершины
        for &(city_index, weight) in &cities[heap_city].1 {
            let new_dist = heap_weight + weight;
            // Обновляем расстояние, если найден более короткий путь до этого города
            if new_dist < dist[city_index] {
                dist[city_index] = new_dist; // Обновляем расстояние от стартовой города до промежуточного, если нашли более короткий путь
                prev[city_index] = Some(heap_city); // Фиксируем, из какого города пришли
                heap.push(Reverse((new_dist, city_index)));
            }
        }
    }

    // Восстановление пути из массива prev
    if dist[end] == usize::MAX {
        return Vec::new(); // Путь не найден
    }

    // Восстанавливаем путь до города в обратном порядке
    let mut path = Vec::new();
    let mut current = end;
    while let Some(prev_city) = prev[current] {
        path.push(current);
        current = prev_city;
    }
    path.push(start_idx);
    path.reverse();

    path
}

#[test]
fn test_dijkstra() {
    let mut city_map: CityMap = Default::default();

    city_map.push(("Барнаул".to_string(), vec![(1, 234), (3, 550), (7, 159)])); // 0
    city_map.push((
        "Новосибирск".to_string(),
        vec![(0, 234), (2, 259), (4, 650), (9, 265)],
    )); // 1
    city_map.push(("Томск".to_string(), vec![(1, 259)])); // 2
    city_map.push(("Павлодар".to_string(), vec![(0, 550), (4, 420), (6, 450)])); // 3
    city_map.push(("Омск".to_string(), vec![(3, 420), (1, 650), (5, 295)])); // 4
    city_map.push(("Петропавловск".to_string(), vec![(4, 295), (6, 510)])); // 5
    city_map.push(("Астана".to_string(), vec![(5, 510), (3, 450)])); // 6
    city_map.push(("Бийск".to_string(), vec![(0, 159), (8, 260)])); // 7
    city_map.push(("Новокузнецк".to_string(), vec![(7, 260), (9, 222)])); // 8
    city_map.push(("Кемерово".to_string(), vec![(8, 222), (1, 265)])); // 9

    assert_eq!(dijkstra(&city_map, 0, 2), vec![0, 1, 2]);
    assert_eq!(dijkstra(&city_map, 8, 6), vec![8, 7, 0, 3, 6]);
    assert_eq!(dijkstra(&city_map, 0, 9), vec![0, 1, 9]);
    assert_eq!(dijkstra(&city_map, 0, 5), vec![0, 1, 4, 5]);
    assert_eq!(dijkstra(&city_map, 0, 6), vec![0, 3, 6]);
}
