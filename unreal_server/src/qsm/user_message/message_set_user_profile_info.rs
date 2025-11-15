use std::io;

// 자동 생성된 구조체 및 관련 메서드
        #[repr(C)]
        #[derive(Debug, Clone)]
        pub struct SetPlayerInfo {
           pub mid: u32,
   pub pId: u32,
   pub UserProfileName: String,}
    
        impl SetPlayerInfo {
            pub fn new(mid: u32, pId: u32, UserProfileName: String) -> Self {
                Self {
                    mid,
            pId,
            UserProfileName,        }
            }
    
            pub fn serialize(&self) -> Vec<u8> {
                let mut buffer = Vec::new();
                buffer.extend(&self.mid.to_le_bytes());
        buffer.extend(&self.pId.to_le_bytes());
        buffer.extend(&self.UserProfileName.len().to_le_bytes());
buffer.extend(self.UserProfileName.as_bytes());
                buffer
            }
    
            pub fn deserialize(buffer: &[u8]) -> io::Result<Self> {
                let mut offset = 0;
                let mut mid_bytes = [0u8; 4];
mid_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let mid = u32::from_le_bytes(mid_bytes);
offset += 4;
        let mut pId_bytes = [0u8; 4];
pId_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let pId = u32::from_le_bytes(pId_bytes);
offset += 4;
        let mut UserProfileName_length_bytes = [0u8; 4];
UserProfileName_length_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let UserProfileName_length = u32::from_le_bytes(UserProfileName_length_bytes);
offset += 4;
let UserProfileName = String::from_utf8(buffer[offset..offset + UserProfileName_length as usize].to_vec())
.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 string"))?;
offset += UserProfileName_length as usize;
                Ok(Self {
                                mid,
            pId,
            UserProfileName,
                })
            }
        }