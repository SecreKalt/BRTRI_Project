# BRTRI (Blender Real-Time LiDAR Integration)

A real-time 3D scanning system using iPhone LiDAR with direct streaming to Mac/Blender.

## Project Overview
- iPhone acts as LiDAR scanner (zero local processing)
- Bridge application handles data transfer and processing
- Blender plugin provides real-time visualization

## Components
### Blender Plugin
- Real-time point cloud visualization
- ZMQ communication
- Performance-optimized processing

### Bridge Application (Rust)
- High-performance data routing
- Point cloud processing
- Zero-copy data handling

### iOS App
- LiDAR data capture
- Real-time streaming
- Hardware-accelerated compression

## Technical Requirements
- Apple device with LiDAR
- Mac for Bridge application
- Blender 3.3+
- Rust toolchain
- Python 3.10+

## Dependencies
- Open3D
- PyZMQ
- PCL
- tokio
- rayon

## Development Status
Early development - Rapid prototyping phase

## License
MIT License

## Contributing
Contributions welcome! Please read CONTRIBUTING.md
