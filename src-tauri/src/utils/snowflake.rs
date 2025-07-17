use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

// 定义雪花ID的各部分位数
const WORKER_ID_BITS: u64 = 5;
const DATACENTER_ID_BITS: u64 = 5;
const SEQUENCE_BITS: u64 = 12;

// 计算各部分的最大值
const MAX_WORKER_ID: u64 = (1 << WORKER_ID_BITS) - 1; // 31
const MAX_DATACENTER_ID: u64 = (1 << DATACENTER_ID_BITS) - 1; // 31

// 计算各部分的位移
const WORKER_ID_SHIFT: u64 = SEQUENCE_BITS; // 12
const DATACENTER_ID_SHIFT: u64 = SEQUENCE_BITS + WORKER_ID_BITS; // 17
const TIMESTAMP_SHIFT: u64 = SEQUENCE_BITS + WORKER_ID_BITS + DATACENTER_ID_BITS; // 22

// 序列号的掩码，用于在同一毫秒内循环
const SEQUENCE_MASK: u64 = (1 << SEQUENCE_BITS) - 1; // 4095

// 自定义纪元（Epoch），这里设置为 2020-01-01 00:00:00 UTC 的毫秒数
// 使用一个较近的日期可以延长雪花算法的使用寿命
const CUSTOM_EPOCH: u64 = 1577836800000;

/// 用于保护在多线程环境下共享的状态
struct SnowflakeState {
    last_timestamp: u64,
    sequence: u64,
}

/// 雪花ID生成器
#[derive(Clone)]
pub struct Snowflake {
    worker_id: u64,
    datacenter_id: u64,
    state: Arc<Mutex<SnowflakeState>>,
}

impl Snowflake {
    /// 创建一个新的雪花ID生成器
    ///
    /// # Panics
    /// 如果 `worker_id` 或 `datacenter_id` 超出范围 (0-31)，则会 panic。
    pub fn new(worker_id: u64, datacenter_id: u64) -> Self {
        if worker_id > MAX_WORKER_ID {
            panic!("Worker ID must be between 0 and {}", MAX_WORKER_ID);
        }
        if datacenter_id > MAX_DATACENTER_ID {
            panic!("Datacenter ID must be between 0 and {}", MAX_DATACENTER_ID);
        }

        let state = Arc::new(Mutex::new(SnowflakeState {
            last_timestamp: 0,
            sequence: 0,
        }));

        Snowflake {
            worker_id,
            datacenter_id,
            state,
        }
    }

    /// 生成下一个唯一的雪花ID
    ///
    /// # Returns
    /// 返回一个 `Result<u64, &'static str>`。
    /// - `Ok(u64)`: 成功生成ID。
    /// - `Err(&'static str)`: 如果系统时钟回拨，则返回错误。
    pub fn next_id(&self) -> Result<u64, &'static str> {
        // 获取互斥锁，保护状态
        let mut state = self.state.lock().unwrap();

        let mut timestamp = Self::get_time_ms();

        // 检测到时钟回拨
        if timestamp < state.last_timestamp {
            return Err("Clock moved backwards. Refusing to generate id.");
        }

        // 如果在同一毫秒内
        if timestamp == state.last_timestamp {
            // 序列号自增，并使用掩码进行位与运算，防止溢出
            state.sequence = (state.sequence + 1) & SEQUENCE_MASK;
            // 如果序列号达到最大值（即溢出后变为0），则需要等待下一毫秒
            if state.sequence == 0 {
                timestamp = self.til_next_millis(state.last_timestamp);
            }
        } else {
            // 如果是新的毫秒，则序列号重置为0
            state.sequence = 0;
        }

        // 更新最后的时间戳
        state.last_timestamp = timestamp;

        // 组合ID的各个部分
        let id = ((timestamp - CUSTOM_EPOCH) << TIMESTAMP_SHIFT)
            | (self.datacenter_id << DATACENTER_ID_SHIFT)
            | (self.worker_id << WORKER_ID_SHIFT)
            | state.sequence;

        Ok(id)
    }

    /// 阻塞直到下一毫秒
    fn til_next_millis(&self, last_timestamp: u64) -> u64 {
        let mut timestamp = Self::get_time_ms();
        while timestamp <= last_timestamp {
            // 可以选择 `std::thread::yield_now()` 或短暂 sleep 来避免CPU空转
            std::thread::yield_now();
            timestamp = Self::get_time_ms();
        }
        timestamp
    }

    /// 获取当前时间的毫秒数（相对于UNIX纪元）
    fn get_time_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64
    }
}

#[tauri::command]
pub fn generate_snowflake_id(
    generator: tauri::State<'_, Arc<Snowflake>>
) -> Result<String, String> {
    // 调用 next_id()
    match generator.next_id() {
        Ok(id) => {
            Ok(id.to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_new_with_valid_ids() {
        let generator = Snowflake::new(0, 0);
        assert_eq!(generator.worker_id, 0);
        assert_eq!(generator.datacenter_id, 0);

        let generator = Snowflake::new(31, 31);
        assert_eq!(generator.worker_id, 31);
        assert_eq!(generator.datacenter_id, 31);
    }

    #[test]
    #[should_panic]
    fn test_new_with_invalid_worker_id() {
        Snowflake::new(32, 0);
    }

    #[test]
    #[should_panic]
    fn test_new_with_invalid_datacenter_id() {
        Snowflake::new(0, 32);
    }

    /// 测试ID结构和反解析
    #[test]
    fn test_id_deconstruction() {
        let worker_id = 5;
        let datacenter_id = 10;
        let generator = Snowflake::new(worker_id, datacenter_id);
        let id = generator.next_id().unwrap();

        let sequence = id & SEQUENCE_MASK;
        let decoded_worker_id = (id >> WORKER_ID_SHIFT) & MAX_WORKER_ID;
        let decoded_datacenter_id = (id >> DATACENTER_ID_SHIFT) & MAX_DATACENTER_ID;
        println!("{}", id);
        assert_eq!(sequence, 0); // 在新的一毫秒，序列号应为0
        assert_eq!(decoded_worker_id, worker_id);
        assert_eq!(decoded_datacenter_id, datacenter_id);
    }

    /// 测试序列号自增
    #[test]
    fn test_sequence_increment() {
        let generator = Snowflake::new(1, 1);
        let id1 = generator.next_id().unwrap();
        let id2 = generator.next_id().unwrap();

        // 假设测试运行得足够快，在同一毫秒内
        // 验证ID是递增的，并且差值为1
        assert!(id2 > id1);
        if (id1 >> TIMESTAMP_SHIFT) == (id2 >> TIMESTAMP_SHIFT) {
            assert_eq!(id2 - id1, 1);
        }
    }

    /// 测试单线程生成ID的唯一性
    #[test]
    fn test_uniqueness_single_thread() {
        let generator = Snowflake::new(2, 2);
        let num_ids = 50_000;
        let mut id_set = HashSet::with_capacity(num_ids);

        for _ in 0..num_ids {
            let id = generator.next_id().unwrap();
            assert!(id_set.insert(id), "Duplicate ID generated: {}", id);
        }
        assert_eq!(id_set.len(), num_ids);
    }

    /// 测试多线程生成ID的唯一性
    #[test]
    fn test_uniqueness_multi_thread() {
        // 使用 Arc 来在线程间共享生成器
        let generator = Arc::new(Snowflake::new(3, 3));
        let num_threads = 10;
        let ids_per_thread = 10_000;
        let total_ids = num_threads * ids_per_thread;

        let mut handles = vec![];

        for _ in 0..num_threads {
            let generator_clone = Arc::clone(&generator);
            let handle = thread::spawn(move || {
                let mut ids = Vec::with_capacity(ids_per_thread);
                for _ in 0..ids_per_thread {
                    ids.push(generator_clone.next_id().unwrap());
                }
                ids
            });
            handles.push(handle);
        }

        let mut all_ids = Vec::with_capacity(total_ids);
        for handle in handles {
            all_ids.extend(handle.join().unwrap());
        }

        // 使用 HashSet 验证唯一性
        let id_set: HashSet<_> = all_ids.into_iter().collect();
        assert_eq!(id_set.len(), total_ids, "Duplicate IDs were generated in a multi-threaded context.");
    }

    /// 测试跨毫秒边界时序列号重置
    #[test]
    fn test_sequence_reset_across_millis() {
        let generator = Snowflake::new(4, 4);

        // 生成第一个ID
        let id1 = generator.next_id().unwrap();

        // 等待超过一毫秒
        thread::sleep(Duration::from_millis(2));

        // 生成第二个ID
        let id2 = generator.next_id().unwrap();

        let timestamp1 = id1 >> TIMESTAMP_SHIFT;
        let timestamp2 = id2 >> TIMESTAMP_SHIFT;

        let sequence1 = id1 & SEQUENCE_MASK;
        let sequence2 = id2 & SEQUENCE_MASK;

        // 时间戳应该已经推进
        assert!(timestamp2 > timestamp1);
        // 第二个ID的序列号应该已重置为0
        assert_eq!(sequence2, 0);
    }

    /// 模拟时钟回拨，测试是否返回错误
    #[test]
    fn test_clock_moving_backwards() {
        let generator = Snowflake::new(5, 5);

        // 手动操纵内部状态来模拟时钟回拨
        {
            let mut state = generator.state.lock().unwrap();
            // 生成一个未来的时间戳
            let future_timestamp = Snowflake::get_time_ms() + 100;
            state.last_timestamp = future_timestamp;
        }

        // 再次尝试生成ID，此时当前时间会小于 `last_timestamp`
        let result = generator.next_id();
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Clock moved backwards. Refusing to generate id.");
    }
}