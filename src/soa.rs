// Struct of Arrays (SoA): 데이터 지향 설계
pub struct EntityManagerSoA {
    x: Vec<f32>,
    y: Vec<f32>,
    vx: Vec<f32>,
    vy: Vec<f32>,
    health: Vec<f32>,
}

impl EntityManagerSoA {
    pub fn new(n: usize) -> Self {
        Self {
            x: (0..n).map(|i| i as f32).collect(),
            y: (0..n).map(|i| i as f32).collect(),
            vx: vec![1.0; n],
            vy: vec![0.5; n],
            health: vec![100.0; n],
        }
    }

    // 위치 업데이트: x, y, vx, vy 배열만 접근
    pub fn update_positions(&mut self, dt: f32) {
        for i in 0..self.x.len() {
            self.x[i] += self.vx[i] * dt;
            self.y[i] += self.vy[i] * dt;
        }
    }
}