import ARKit
import Foundation

protocol ScannerDelegate: AnyObject {
    func didOutputScannedData(_ data: Data)
}

class Scanner: NSObject, ARSCNViewDelegate {
    private var sceneView: ARSCNView!
    private var isScanning: Bool = false
    weak var delegate: ScannerDelegate?

    override init() {
        super.init()
        setupSceneView()
    }

    private func setupSceneView() {
        sceneView = ARSCNView(frame: .zero)
        sceneView.delegate = self
        sceneView.session = ARSession()
    }

    func startScanning() {
        guard !isScanning else { return }
        isScanning = true
        let configuration = ARWorldTrackingConfiguration()
        configuration.planeDetection = [.horizontal, .vertical]
        configuration.environmentTexturing = .automatic
        sceneView.session.run(configuration, options: [.resetTracking, .removeExistingAnchors])
    }

    func stopScanning() {
        guard isScanning else { return }
        isScanning = false
        sceneView.session.pause()
    }

    func session(_ session: ARSession, didUpdate frame: ARFrame) {
        guard isScanning else { return }
        let pointCloud = frame.rawFeaturePoints
        if let points = pointCloud?.points {
            let data = Data(bytes: points, count: points.count * MemoryLayout<float3>.size)
            delegate?.didOutputScannedData(data)
        }
    }
}
