pub fn heap_sort(data: &mut Vec<i32>) {
    // 1단계: 전체를 힙으로 구성 (heapify)
    let n = data.len();
    // 마지막 내부 노드부터 루트 방향으로 sift_down
    for i in (0..n / 2).rev() {
        sift_down_sort(data, i, n);
    }

    // 2단계: 루트(최댓값)를 맨 뒤로 보내고 힙 크기를 줄임
    for end in (1..n).rev() {
        data.swap(0, end);         // 루트(최댓값)를 맨 끝으로
        sift_down_sort(data, 0, end); // 줄어든 힙 범위에서 sift_down
    }
}

fn sift_down_sort(data: &mut Vec<i32>, mut idx: usize, heap_size: usize) {
    loop {
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;
        let mut largest = idx;

        if left < heap_size && data[left] > data[largest] {
            largest = left;
        }
        if right < heap_size && data[right] > data[largest] {
            largest = right;
        }

        if largest == idx {
            break;
        }

        data.swap(idx, largest);
        idx = largest;
    }
}