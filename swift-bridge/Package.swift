// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "PDFKitBridge",
    platforms: [
        .macOS(.v11)
    ],
    products: [
        .library(
            name: "PDFKitBridge",
            type: .static,
            targets: ["PDFKitBridge"])
    ],
    targets: [
        .target(
            name: "PDFKitBridge",
            path: "Sources/PDFKitBridge")
    ]
)
