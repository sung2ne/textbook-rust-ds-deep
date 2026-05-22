use std::io::{self, BufRead, BufReader, Cursor};

fn process_in_batches<I>(lines: I, batch_size: usize) -> usize
where
    I: Iterator<Item = String>,
{
    let mut total = 0;
    let mut batch: Vec<String> = Vec::with_capacity(batch_size);

    for line in lines {
        batch.push(line);

        if batch.len() == batch_size {
            // 배치가 가득 차면 처리
            total += batch.len();
            println!("배치 처리: {} 줄", batch.len());
            batch.clear();  // clear()는 capacity를 유지한다 (중요!)
        }
    }

    // 남은 줄 처리
    if !batch.is_empty() {
        total += batch.len();
        println!("마지막 배치: {} 줄", batch.len());
    }

    total
}

fn main() {
    // 가상의 스트림 (실제로는 File 또는 네트워크 소켓)
    let data = "줄1\n줄2\n줄3\n줄4\n줄5\n줄6\n줄7\n줄8\n줄9\n줄10\n";
    let cursor = Cursor::new(data);
    let reader = BufReader::new(cursor);

    let lines = reader
        .lines()
        .filter_map(|l| l.ok());

    let total = process_in_batches(lines, 3);
    println!("총 처리: {} 줄", total);
}