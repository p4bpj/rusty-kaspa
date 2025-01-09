// swift-tools-version: 5.10
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let binaryTarget: Target = .binaryTarget(
    name: "KaspaWalletCoreRS",
    // IMPORTANT: Swift packages importing this locally will not be able to
    // import the rust core unless you use a relative path.
    // This ONLY works for local development. For a larger scale usage example, see https://github.com/stadiamaps/ferrostar.
    // When you release a public package, you will need to build a release XCFramework,
    // upload it somewhere (usually with your release), and update Package.swift.
    // This will probably be the subject of a future blog.
    // Again, see Ferrostar for a more complex example, including more advanced GitHub actions.
    path: "../target/ios/libkaspa_wallet_core-rs.xcframework"
)

let package = Package(
    name: "KaspaWalletCoreFFI",
    platforms: [
        .iOS(.v16),
    ],
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "KaspaWalletCoreFFI",
            targets: ["KaspaWalletCoreFFI"]
        ),
    ],
    targets: [
        binaryTarget,
        /*.target(
            name: "Foobar",
            dependencies: [.target(name: "UniFFI")],
            path: "apple/Sources/Foobar"
        ),*/
        .target(
            name: "KaspaWalletCoreFFI",
            dependencies: [.target(name: "KaspaWalletCoreRS")],
            path: "apple/Sources/UniFFI/"
        ),
        /*.testTarget(
            name: "FoobarTests",
            dependencies: ["Foobar"],
            path: "apple/Tests/FoobarTests"
        ),*/
    ]
)
