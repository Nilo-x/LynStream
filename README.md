# ⚡ LynStream
*A lightning-fast video streaming server built in Rust.*  

![Demo](show_case.gif)

---

## 🎯 What is this?

**LynStream** lets you upload videos, livestream using ffmpeg, and watch them instantly in mpv or ffplay.  

Think of it as a minimal backend similar to YouTube Live:  
- 📤 Push video into the server  
- 📺 View it live or later as Video On Demand (VOD)  
- 🔁 Multiple viewers can connect at once  

This project demonstrates **high-performance backend development with Rust**.

---

## ✨ Features

- **Live streaming**: ingest via ffmpeg, watch live with mpv  
- **Video on Demand (VOD)**: upload & play stored files  
- **Ring buffer playback**: new viewers start cleanly (no codec errors)  
- **Multiple viewers**: live broadcast via Tokio broadcast channels  
- **Extensible structure**: swap MemoryStorage for Redis/S3 just use StorageBackend trait  
- **Docker ready**: portable, runs anywhere  
- **CI pipeline**: automated build & test  

---

## 🚀 Quick Start

### 1. Run with Cargo
```bash
cargo run
```

### 2. Run with docker
```bash
docker build -t lynstream .
docker run -p 3000:3000 lynstream
```
