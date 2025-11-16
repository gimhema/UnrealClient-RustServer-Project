use std::io;

// 자동 생성된 구조체 및 관련 메서드
        #[repr(C)]
        #[derive(Debug, Clone)]
        pub struct UpdatePlayersInfo {
           pub mid: u32,
   pub pid: u32,
   pub PlayerName: String,}
    
        impl UpdatePlayersInfo {
            pub fn new(mid: u32, pid: u32, PlayerName: String) -> Self {
                Self {
                    mid,
            pid,
            PlayerName,        }
            }
    
            pub fn serialize(&self) -> Vec<u8> {
                let mut buffer = Vec::new();
                buffer.extend(&self.mid.to_le_bytes());
        buffer.extend(&self.pid.to_le_bytes());
        buffer.extend(&self.PlayerName.len().to_le_bytes());
buffer.extend(self.PlayerName.as_bytes());
                buffer
            }
    
            pub fn deserialize(buffer: &[u8]) -> io::Result<Self> {
                let mut offset = 0;
                let mut mid_bytes = [0u8; 4];
mid_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let mid = u32::from_le_bytes(mid_bytes);
offset += 4;
        let mut pid_bytes = [0u8; 4];
pid_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let pid = u32::from_le_bytes(pid_bytes);
offset += 4;
        let mut PlayerName_length_bytes = [0u8; 4];
PlayerName_length_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let PlayerName_length = u32::from_le_bytes(PlayerName_length_bytes);
offset += 4;
let PlayerName = String::from_utf8(buffer[offset..offset + PlayerName_length as usize].to_vec())
.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 string"))?;
offset += PlayerName_length as usize;
                Ok(Self {
                                mid,
            pid,
            PlayerName,
                })
            }
        }