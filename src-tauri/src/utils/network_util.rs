use serde::Serialize;
use std::time::Duration;
use tokio::net::TcpStream;

#[derive(Serialize, Clone)]
pub struct CheckPortResult {
    result: bool,
    reason: String,
}

impl CheckPortResult {
    pub fn new(result: bool, reason: &str) -> Self {
        Self {
            result,
            reason: reason.to_string(),
        }
    }
}

#[tauri::command]
pub async fn is_port_open(host: String, port: u16) -> Result<CheckPortResult, String> {
    let address = format!("{}:{}", host, port);
    let connection_attempt = TcpStream::connect(&address);

    match tokio::time::timeout(Duration::from_secs(5), connection_attempt).await {
        Ok(Ok(_)) => Ok(CheckPortResult::new(true, "端口开放")),
        Ok(Err(_)) => Ok(CheckPortResult::new(false, "端口关闭或被拒绝")),
        Err(_) => Err("连接超时".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn test_port_is_open() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let host = "127.0.0.1".to_string();
        println!("临时监听器已在端口 {} 上启动", port);
        let result = is_port_open(host, port).await.unwrap();
        assert_eq!(result.result, true);
        assert_eq!(result.reason, "端口开放");
    }

    /// 测试用例 2: 检查一个几乎可以肯定是关闭的端口
    #[tokio::test]
    async fn test_port_is_closed() {
        let port = 49150;
        let host = "127.0.0.1".to_string();
        let result = is_port_open(host, port).await.unwrap();
        assert_eq!(result.result, false);
        assert!(result.reason == "端口关闭或被拒绝" || result.reason == "连接超时");
    }

    /// 测试用例 3: 测试连接超时的情况
    /// 注意：这个测试会比较慢，因为需要等待超时
    #[tokio::test]
    async fn test_port_timeout() {
        let port = 81;
        let host = "8.138.224.34".to_string();
        let result = is_port_open(host, port).await.unwrap();
        assert_eq!(result.result, false);
        assert_eq!(result.reason, "连接超时");
    }

    #[tokio::test]
    async fn test_port_success_remote() {
        let host = "8.138.224.34".to_string();
        let result = is_port_open(host, 3306).await.unwrap();
        assert_eq!(result.result, true);
    }

    #[tokio::test]
    async fn test_port_fail_remote() {
        let host = "8.138.224.34".to_string();
        let result = is_port_open(host, 33076).await.unwrap();
        println!("{:?}", result.result);
        println!("{:?}", result.reason);
        assert_eq!(result.result, false);
    }
}
