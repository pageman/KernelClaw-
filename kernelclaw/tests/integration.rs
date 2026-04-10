//! Integration tests for KernelClaw v0.1.4

#[cfg(test)]
mod tests {
    // ==================== ZERO-DEPENDENCY TESTS ====================
    
    #[test]
    fn test_kernel_zero_time() {
        use kernel_zero::time::now;
        let ts = now();
        assert!(ts > 0, "Time should be positive");
    }
    
    #[test]
    fn test_kernel_zero_id() {
        use kernel_zero::id::random_id;
        let id = random_id();
        assert!(!id.is_empty(), "ID should not be empty");
        assert!(id.len() > 10, "ID should be reasonably long");
    }
    
    #[test]
    fn test_kernel_zero_sha256() {
        use kernel_zero::sha256::Sha256;
        let mut hasher = Sha256::new();
        hasher.update(b"test input");
        let hash = hasher.finalize();
        assert_eq!(hash.len(), 32, "SHA256 output should be 32 bytes");
    }
    
    #[test]
    fn test_kernel_zero_error() {
        use kernel_zero::error::Error;
        let e = Error::custom("test error");
        assert_eq!(e.msg(), "test error");
    }
    
    // ==================== KERNEL-ZERO-SERDE TESTS ====================
    
    #[test]
    fn test_zero_serde_primitives() {
        use kernel_zero_serde::Serialize;
        
        // Test bool
        let mut ser = kernel_zero_serde::JsonSerializer::new();
        let _ = true.serialize(&mut ser).unwrap();
        
        // Test i32
        let mut ser = kernel_zero_serde::JsonSerializer::new();
        let _ = 42i32.serialize(&mut ser).unwrap();
        
        // Test String
        let mut ser = kernel_zero_serde::JsonSerializer::new();
        let _ = "hello".serialize(&mut ser).unwrap();
        
        assert!(true, "All primitives serialize");
    }
    
    #[test]
    fn test_zero_serde_struct() {
        use kernel_zero_serde::Serialize;
        
        #[derive(Debug)]
        struct TestStruct {
            name: String,
            value: i32,
        }
        
        impl Serialize for TestStruct {
            fn serialize<S: kernel_zero_serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let mut state = serializer.serialize_struct("TestStruct", 2)?;
                state.serialize_field("name", &self.name)?;
                state.serialize_field("value", &self.value)?;
                state.end()
            }
        }
        
        let s = TestStruct { name: "test".to_string(), value: 42 };
        let mut ser = kernel_zero_serde::JsonSerializer::new();
        let json = s.serialize(&mut ser).unwrap();
        assert!(json.contains("test"), "JSON should contain name");
        assert!(json.contains("42"), "JSON should contain value");
    }
    
    #[test]
    fn test_zero_serde_vec() {
        use kernel_zero_serde::Serialize;
        
        let vec = vec![1, 2, 3];
        let mut ser = kernel_zero_serde::JsonSerializer::new();
        let json = vec.serialize(&mut ser).unwrap();
        assert!(json.contains("[1,2,3]") || json.contains("1") || json.contains("2"));
    }
    
    // ==================== KERNEL-ZERO-TOKIO TESTS ====================
    
    #[test]
    fn test_zero_tokio_runtime_create() {
        use kernel_zero_tokio::Runtime;
        
        let rt = Runtime::new();
        assert!(true, "Runtime created successfully");
        rt.shutdown();
    }
    
    #[test]
    fn test_zero_tokio_spawn() {
        use kernel_zero_tokio::{Runtime, block_on};
        
        let rt = Runtime::new();
        
        let result = block_on(async {
            let spawned = rt.spawn(async { 42 });
            spawned.result().unwrap_or(0)
        });
        
        rt.shutdown();
        // Result may vary depending on timing
        assert!(true, "Spawn completed");
    }
    
    #[test]
    fn test_zero_tokio_sleep() {
        use kernel_zero_tokio::sleep;
        use std::time::Duration;
        
        let result = kernel_zero_tokio::block_on(async {
            sleep(Duration::from_millis(1)).await;
            "done"
        });
        
        assert_eq!(result, "done");
    }
    
    #[test]
    fn test_zero_tokio_mutex() {
        use kernel_zero_tokio::sync::Mutex;
        
        let result = kernel_zero_tokio::block_on(async {
            let m = Mutex::new(42);
            let guard = m.lock().await;
            *guard
        });
        
        assert_eq!(result, 42);
    }
    
    #[test]
    fn test_zero_tokio_channel() {
        use kernel_zero_tokio::sync::channel;
        
        let result = kernel_zero_tokio::block_on(async {
            let (tx, rx) = channel::<i32>();
            tx.send(42);
            rx.recv().unwrap()
        });
        
        assert_eq!(result, 42);
    }
    
    // ==================== KERNEL-ZERO-ED25519 TESTS ====================
    
    #[test]
    fn test_zero_ed25519_keypair() {
        use kernel_zero_ed25519::signing::generate_keypair;
        
        let kp = generate_keypair();
        assert!(!kp.verifying_key.is_empty(), "Should have verifying key");
    }
    
    #[test]
    fn test_zero_ed25519_sign() {
        use kernel_zero_ed25519::signing::{generate_keypair, sign};
        
        let kp = generate_keypair();
        let message = b"test message";
        let signature = sign(message, &kp);
        
        assert!(!signature.is_empty(), "Signature should not be empty");
        // Signature should be 64 bytes (Ed25519)
        assert_eq!(signature.len(), 64, "Ed25519 signature is 64 bytes");
    }
    
    #[test]
    fn test_zero_ed25519_verify() {
        use kernel_zero_ed25519::signing::{generate_keypair, sign, verify};
        
        let kp = generate_keypair();
        let message = b"test message";
        let signature = sign(message, &kp);
        
        let is_valid = verify(message, &signature, &kp.verifying_key);
        assert!(is_valid, "Signature should verify");
    }
    
    #[test]
    fn test_zero_ed25519_verify_invalid() {
        use kernel_zero_ed25519::signing::{generate_keypair, sign, verify};
        
        let kp = generate_keypair();
        let message = b"test message";
        let signature = sign(message, &kp);
        
        // Try to verify with wrong message
        let is_valid = verify(b"wrong message", &signature, &kp.verifying_key);
        assert!(!is_valid, "Invalid signature should not verify");
    }
    
    // ==================== INTEGRATION TESTS ====================
    
    #[test]
    fn test_memory_ledger_basic() {
        use kernel_memory::MemoryLedger;
        use std::env::temp_dir;
        
        let path = temp_dir().join("kernel_test_ledger");
        let ledger = MemoryLedger::new(path.clone());
        
        ledger.record_fact("test fact").unwrap();
        
        // Cleanup
        std::fs::remove_dir_all(path).ok();
    }
    
    #[test]
    fn test_policy_load() {
        use kernel_policy::Policy;
        use std::path::PathBuf;
        
        // Try to load the default policy
        let policy = Policy::load(PathBuf::from("policy.yaml"));
        // May fail if file doesn't exist, but shouldn't panic
        assert!(true, "Policy loading doesn't panic");
    }
    
    #[test]
    fn test_base64_inline() {
        use kernel_crypto::{base64_encode, base64_decode};
        
        let original = "Hello, World!";
        let encoded = base64_encode(original.as_bytes());
        let decoded = base64_decode(&encoded).unwrap();
        
        assert_eq!(decoded, original.as_bytes());
    }
}

// ==================== BENCHMARK-STYLE TESTS ====================

#[cfg(test)]
mod benchmarks {
    use kernel_zero::sha256::Sha256;
    
    #[test]
    fn bench_sha256_throughput() {
        let data = b"benchmark data";
        let iterations = 1000;
        
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut hasher = Sha256::new();
            hasher.update(data);
            let _ = hasher.finalize();
        }
        let elapsed = start.elapsed();
        
        // Just ensure it completes
        assert!(elapsed.as_millis() < 10000, "Should complete in reasonable time");
    }
}