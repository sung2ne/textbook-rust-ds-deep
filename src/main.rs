fn main() {
    // 커스텀 힙 테스트
    let mut heap = MaxHeap::new();
    for v in [3, 1, 7, 5, 2, 8, 4] {
        heap.push(v);
    }

    print!("pop 순서: ");
    while let Some(v) = heap.pop() {
        print!("{} ", v);  // 8 7 5 4 3 2 1
    }
    println!();

    // 힙 정렬 테스트
    let mut data = vec![5, 3, 8, 1, 9, 2, 7, 4, 6];
    heap_sort(&mut data);
    println!("힙 정렬: {:?}", data);
    // [1, 2, 3, 4, 5, 6, 7, 8, 9]

    // 우선순위 큐 응용: 작업 스케줄러
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    #[derive(Eq, PartialEq)]
    struct Task {
        priority: u32,
        name: &'static str,
    }

    impl Ord for Task {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.priority.cmp(&other.priority)
        }
    }

    impl PartialOrd for Task {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut scheduler: BinaryHeap<Task> = BinaryHeap::new();
    scheduler.push(Task { priority: 3, name: "저장 백업" });
    scheduler.push(Task { priority: 10, name: "긴급 패치" });
    scheduler.push(Task { priority: 7, name: "로그 분석" });

    while let Some(task) = scheduler.pop() {
        println!("처리: [{}] {}", task.priority, task.name);
    }
    // 처리: [10] 긴급 패치
    // 처리: [7] 로그 분석
    // 처리: [3] 저장 백업
}