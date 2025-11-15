#pragma once
#include <cstdint>
#include <cstring>
#include <string>
#include <vector>

struct SetPlayerInfo {
    uint32_t mid = 0;
    uint32_t pId = 0;
    std::string UserProfileName;

    std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer;
        size_t approx = 0;
        approx += 4;
        approx += 4;
        approx += 4 + UserProfileName.size();
        buffer.reserve(approx);

        auto put_u32 = [&](uint32_t v) {
            uint8_t b[4]; std::memcpy(b, &v, 4); buffer.insert(buffer.end(), b, b+4);
        };
        auto put_u64 = [&](uint64_t v) {
            uint8_t b[8]; std::memcpy(b, &v, 8); buffer.insert(buffer.end(), b, b+8);
        };
        auto put_f32 = [&](float v) {
            uint8_t b[4]; std::memcpy(b, &v, 4); buffer.insert(buffer.end(), b, b+4);
        };
        auto put_str = [&](const std::string& s) {
            put_u32(static_cast<uint32_t>(s.size()));
            buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(s.data()), reinterpret_cast<const uint8_t*>(s.data()) + s.size());
        };
        auto put_vec_u32 = [&](const std::vector<uint32_t>& v) {
            put_u32(static_cast<uint32_t>(v.size()));
            for (auto x : v) put_u32(x);
        };
        auto put_vec_u64 = [&](const std::vector<uint64_t>& v) {
            put_u32(static_cast<uint32_t>(v.size()));
            for (auto x : v) put_u64(x);
        };
        auto put_vec_f32 = [&](const std::vector<float>& v) {
            put_u32(static_cast<uint32_t>(v.size()));
            for (auto x : v) put_f32(x);
        };
        auto put_vec_str = [&](const std::vector<std::string>& v) {
            put_u32(static_cast<uint32_t>(v.size()));
            for (auto &s : v) put_str(s);
        };

        put_u32(mid);
        put_u32(pId);
        put_str(UserProfileName);
        return buffer;
    }

private:
    static bool try_deserialize_at(const std::vector<uint8_t>& buf, size_t start, SetPlayerInfo& out) {
        size_t off = start;
        auto need = [&](size_t n)->bool { return buf.size() - off >= n; };
        auto get_u32 = [&](uint32_t& v)->bool { if (!need(4)) return false; std::memcpy(&v, buf.data()+off, 4); off += 4; return true; };
        auto get_u64 = [&](uint64_t& v)->bool { if (!need(8)) return false; std::memcpy(&v, buf.data()+off, 8); off += 8; return true; };
        auto get_f32 = [&](float& v)->bool { if (!need(4)) return false; std::memcpy(&v, buf.data()+off, 4); off += 4; return true; };
        auto get_str = [&](std::string& s)->bool {
            uint32_t len = 0; if (!get_u32(len)) return false;
            constexpr uint32_t MAX_FIELD = 1u << 20; if (len > MAX_FIELD) return false;
            if (!need(len)) return false; s.assign(reinterpret_cast<const char*>(buf.data()+off), len); off += len; return true;
        };
        auto get_vec_u32 = [&](std::vector<uint32_t>& v)->bool { uint32_t n=0; if(!get_u32(n)) return false; v.clear(); v.reserve(n); for(uint32_t i=0;i<n;++i){ uint32_t t; if(!get_u32(t)) return false; v.push_back(t); } return true; };
        auto get_vec_u64 = [&](std::vector<uint64_t>& v)->bool { uint32_t n=0; if(!get_u32(n)) return false; v.clear(); v.reserve(n); for(uint32_t i=0;i<n;++i){ uint64_t t; if(!get_u64(t)) return false; v.push_back(t); } return true; };
        auto get_vec_f32 = [&](std::vector<float>& v)->bool { uint32_t n=0; if(!get_u32(n)) return false; v.clear(); v.reserve(n); for(uint32_t i=0;i<n;++i){ float t; if(!get_f32(t)) return false; v.push_back(t); } return true; };
        auto get_vec_str = [&](std::vector<std::string>& v)->bool { uint32_t n=0; if(!get_u32(n)) return false; v.clear(); v.reserve(n); for(uint32_t i=0;i<n;++i){ std::string s; if(!get_str(s)) return false; v.push_back(std::move(s)); } return true; };
        uint32_t __mid = 0;
        uint32_t __pId = 0;
        std::string __UserProfileName;

        if (!get_u32(__mid)) return false;
        if (!get_u32(__pId)) return false;
        if (!get_str(__UserProfileName)) return false;

        out.mid = __mid;
        out.pId = __pId;
        out.UserProfileName = std::move(__UserProfileName);
        return true;
    }

public:
    static SetPlayerInfo deserialize(const std::vector<uint8_t>& buffer) {
        SetPlayerInfo out;
        size_t off0 = 0;
        if (buffer.size() >= 8) {
            uint32_t len = 0; std::memcpy(&len, buffer.data(), 4);
            if (len == buffer.size() || len + 4 == buffer.size()) off0 = 4;
        }
        if (try_deserialize_at(buffer, off0, out)) return out;
        if (off0 == 4 && try_deserialize_at(buffer, 0, out)) return out;
        return out;
    }
};
