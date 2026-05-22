// Array of Structs (AoS): 전통적인 방식
struct Entity {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    health: f32,
}

pub struct EntityManagerAoS {
    entities: Vec<Entity>,
}

impl EntityManagerAoS {
    pub fn new(n: usize) -> Self {
        let entities = (0..n)
            .map(|i| Entity {
                x: i as f32,
                y: i as f32,
                vx: 1.0,
                vy: 0.5,
                health: 100.0,
            })
            .collect();
        Self { entities }
    }

    // 모든 엔티티 위치 업데이트 — 물리 시뮬레이션의 핵심 루프
    pub fn update_positions(&mut self, dt: f32) {
        for e in &mut self.entities {
            e.x += e.vx * dt;
            e.y += e.vy * dt;
        }
    }
}