// DO NOT EDIT.
// swift-format-ignore-file
//
// Generated by the Swift generator plugin for the protocol buffer compiler.
// Source: did_document_format.proto
//
// For information on using the generated types, please see the documentation:
//   https://github.com/apple/swift-protobuf/

import Foundation
import SwiftProtobuf

// If the compiler emits an error on this type, it is because this file
// was generated by a version of the `protoc` Swift plug-in that is
// incompatible with the version of SwiftProtobuf to which you are linking.
// Please ensure that you are building against the same version of the API
// that was used to generate this file.
fileprivate struct _GeneratedWithProtocGenSwiftVersion: SwiftProtobuf.ProtobufAPIVersionCheck {
  struct _2: SwiftProtobuf.ProtobufAPIVersion_2 {}
  typealias Version = _2
}

enum Document_VerificationType: SwiftProtobuf.Enum {
  typealias RawValue = Int
  case ed25519VerificationKey2020 // = 0
  case sr25519VerificationKey2020 // = 1
  case UNRECOGNIZED(Int)

  init() {
    self = .ed25519VerificationKey2020
  }

  init?(rawValue: Int) {
    switch rawValue {
    case 0: self = .ed25519VerificationKey2020
    case 1: self = .sr25519VerificationKey2020
    default: self = .UNRECOGNIZED(rawValue)
    }
  }

  var rawValue: Int {
    switch self {
    case .ed25519VerificationKey2020: return 0
    case .sr25519VerificationKey2020: return 1
    case .UNRECOGNIZED(let i): return i
    }
  }

}

#if swift(>=4.2)

extension Document_VerificationType: CaseIterable {
  // The compiler won't synthesize support with the UNRECOGNIZED case.
  static let allCases: [Document_VerificationType] = [
    .ed25519VerificationKey2020,
    .sr25519VerificationKey2020,
  ]
}

#endif  // swift(>=4.2)

struct Document_VerificationMethod {
  // SwiftProtobuf.Message conformance is added in an extension below. See the
  // `Message` and `Message+*Additions` files in the SwiftProtobuf library for
  // methods supported on all messages.

  var id: String = String()

  var type: Document_VerificationType = .ed25519VerificationKey2020

  var controller: String = String()

  var publicKeyMultibase: String = String()

  var unknownFields = SwiftProtobuf.UnknownStorage()

  init() {}
}

struct Document_Signature {
  // SwiftProtobuf.Message conformance is added in an extension below. See the
  // `Message` and `Message+*Additions` files in the SwiftProtobuf library for
  // methods supported on all messages.

  var type: Document_VerificationType = .ed25519VerificationKey2020

  var issuer: String = String()

  var hash: String = String()

  var unknownFields = SwiftProtobuf.UnknownStorage()

  init() {}
}

struct Document_Service {
  // SwiftProtobuf.Message conformance is added in an extension below. See the
  // `Message` and `Message+*Additions` files in the SwiftProtobuf library for
  // methods supported on all messages.

  var id: String = String()

  var type: String = String()

  var serviceEndpoint: String = String()

  var data: String = String()

  var unknownFields = SwiftProtobuf.UnknownStorage()

  init() {}
}

struct Document_Document {
  // SwiftProtobuf.Message conformance is added in an extension below. See the
  // `Message` and `Message+*Additions` files in the SwiftProtobuf library for
  // methods supported on all messages.

  var id: String = String()

  var controller: String = String()

  var verificationMethods: [Document_VerificationMethod] = []

  var signature: Document_Signature {
    get {return _signature ?? Document_Signature()}
    set {_signature = newValue}
  }
  /// Returns true if `signature` has been explicitly set.
  var hasSignature: Bool {return self._signature != nil}
  /// Clears the value of `signature`. Subsequent reads from it will return its default value.
  mutating func clearSignature() {self._signature = nil}

  var services: [Document_Service] = []

  var authentications: [String] = []

  var unknownFields = SwiftProtobuf.UnknownStorage()

  init() {}

  fileprivate var _signature: Document_Signature? = nil
}

#if swift(>=5.5) && canImport(_Concurrency)
extension Document_VerificationType: @unchecked Sendable {}
extension Document_VerificationMethod: @unchecked Sendable {}
extension Document_Signature: @unchecked Sendable {}
extension Document_Service: @unchecked Sendable {}
extension Document_Document: @unchecked Sendable {}
#endif  // swift(>=5.5) && canImport(_Concurrency)

// MARK: - Code below here is support for the SwiftProtobuf runtime.

fileprivate let _protobuf_package = "document"

extension Document_VerificationType: SwiftProtobuf._ProtoNameProviding {
  static let _protobuf_nameMap: SwiftProtobuf._NameMap = [
    0: .same(proto: "Ed25519VerificationKey2020"),
    1: .same(proto: "Sr25519VerificationKey2020"),
  ]
}

extension Document_VerificationMethod: SwiftProtobuf.Message, SwiftProtobuf._MessageImplementationBase, SwiftProtobuf._ProtoNameProviding {
  static let protoMessageName: String = _protobuf_package + ".VerificationMethod"
  static let _protobuf_nameMap: SwiftProtobuf._NameMap = [
    1: .same(proto: "id"),
    2: .same(proto: "type"),
    3: .same(proto: "controller"),
    4: .same(proto: "publicKeyMultibase"),
  ]

  mutating func decodeMessage<D: SwiftProtobuf.Decoder>(decoder: inout D) throws {
    while let fieldNumber = try decoder.nextFieldNumber() {
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every case branch when no optimizations are
      // enabled. https://github.com/apple/swift-protobuf/issues/1034
      switch fieldNumber {
      case 1: try { try decoder.decodeSingularStringField(value: &self.id) }()
      case 2: try { try decoder.decodeSingularEnumField(value: &self.type) }()
      case 3: try { try decoder.decodeSingularStringField(value: &self.controller) }()
      case 4: try { try decoder.decodeSingularStringField(value: &self.publicKeyMultibase) }()
      default: break
      }
    }
  }

  func traverse<V: SwiftProtobuf.Visitor>(visitor: inout V) throws {
    if !self.id.isEmpty {
      try visitor.visitSingularStringField(value: self.id, fieldNumber: 1)
    }
    if self.type != .ed25519VerificationKey2020 {
      try visitor.visitSingularEnumField(value: self.type, fieldNumber: 2)
    }
    if !self.controller.isEmpty {
      try visitor.visitSingularStringField(value: self.controller, fieldNumber: 3)
    }
    if !self.publicKeyMultibase.isEmpty {
      try visitor.visitSingularStringField(value: self.publicKeyMultibase, fieldNumber: 4)
    }
    try unknownFields.traverse(visitor: &visitor)
  }

  static func ==(lhs: Document_VerificationMethod, rhs: Document_VerificationMethod) -> Bool {
    if lhs.id != rhs.id {return false}
    if lhs.type != rhs.type {return false}
    if lhs.controller != rhs.controller {return false}
    if lhs.publicKeyMultibase != rhs.publicKeyMultibase {return false}
    if lhs.unknownFields != rhs.unknownFields {return false}
    return true
  }
}

extension Document_Signature: SwiftProtobuf.Message, SwiftProtobuf._MessageImplementationBase, SwiftProtobuf._ProtoNameProviding {
  static let protoMessageName: String = _protobuf_package + ".Signature"
  static let _protobuf_nameMap: SwiftProtobuf._NameMap = [
    1: .same(proto: "type"),
    2: .same(proto: "issuer"),
    3: .same(proto: "hash"),
  ]

  mutating func decodeMessage<D: SwiftProtobuf.Decoder>(decoder: inout D) throws {
    while let fieldNumber = try decoder.nextFieldNumber() {
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every case branch when no optimizations are
      // enabled. https://github.com/apple/swift-protobuf/issues/1034
      switch fieldNumber {
      case 1: try { try decoder.decodeSingularEnumField(value: &self.type) }()
      case 2: try { try decoder.decodeSingularStringField(value: &self.issuer) }()
      case 3: try { try decoder.decodeSingularStringField(value: &self.hash) }()
      default: break
      }
    }
  }

  func traverse<V: SwiftProtobuf.Visitor>(visitor: inout V) throws {
    if self.type != .ed25519VerificationKey2020 {
      try visitor.visitSingularEnumField(value: self.type, fieldNumber: 1)
    }
    if !self.issuer.isEmpty {
      try visitor.visitSingularStringField(value: self.issuer, fieldNumber: 2)
    }
    if !self.hash.isEmpty {
      try visitor.visitSingularStringField(value: self.hash, fieldNumber: 3)
    }
    try unknownFields.traverse(visitor: &visitor)
  }

  static func ==(lhs: Document_Signature, rhs: Document_Signature) -> Bool {
    if lhs.type != rhs.type {return false}
    if lhs.issuer != rhs.issuer {return false}
    if lhs.hash != rhs.hash {return false}
    if lhs.unknownFields != rhs.unknownFields {return false}
    return true
  }
}

extension Document_Service: SwiftProtobuf.Message, SwiftProtobuf._MessageImplementationBase, SwiftProtobuf._ProtoNameProviding {
  static let protoMessageName: String = _protobuf_package + ".Service"
  static let _protobuf_nameMap: SwiftProtobuf._NameMap = [
    1: .same(proto: "id"),
    2: .same(proto: "type"),
    3: .same(proto: "serviceEndpoint"),
    4: .same(proto: "data"),
  ]

  mutating func decodeMessage<D: SwiftProtobuf.Decoder>(decoder: inout D) throws {
    while let fieldNumber = try decoder.nextFieldNumber() {
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every case branch when no optimizations are
      // enabled. https://github.com/apple/swift-protobuf/issues/1034
      switch fieldNumber {
      case 1: try { try decoder.decodeSingularStringField(value: &self.id) }()
      case 2: try { try decoder.decodeSingularStringField(value: &self.type) }()
      case 3: try { try decoder.decodeSingularStringField(value: &self.serviceEndpoint) }()
      case 4: try { try decoder.decodeSingularStringField(value: &self.data) }()
      default: break
      }
    }
  }

  func traverse<V: SwiftProtobuf.Visitor>(visitor: inout V) throws {
    if !self.id.isEmpty {
      try visitor.visitSingularStringField(value: self.id, fieldNumber: 1)
    }
    if !self.type.isEmpty {
      try visitor.visitSingularStringField(value: self.type, fieldNumber: 2)
    }
    if !self.serviceEndpoint.isEmpty {
      try visitor.visitSingularStringField(value: self.serviceEndpoint, fieldNumber: 3)
    }
    if !self.data.isEmpty {
      try visitor.visitSingularStringField(value: self.data, fieldNumber: 4)
    }
    try unknownFields.traverse(visitor: &visitor)
  }

  static func ==(lhs: Document_Service, rhs: Document_Service) -> Bool {
    if lhs.id != rhs.id {return false}
    if lhs.type != rhs.type {return false}
    if lhs.serviceEndpoint != rhs.serviceEndpoint {return false}
    if lhs.data != rhs.data {return false}
    if lhs.unknownFields != rhs.unknownFields {return false}
    return true
  }
}

extension Document_Document: SwiftProtobuf.Message, SwiftProtobuf._MessageImplementationBase, SwiftProtobuf._ProtoNameProviding {
  static let protoMessageName: String = _protobuf_package + ".Document"
  static let _protobuf_nameMap: SwiftProtobuf._NameMap = [
    1: .same(proto: "id"),
    2: .same(proto: "controller"),
    3: .same(proto: "verificationMethods"),
    4: .same(proto: "signature"),
    5: .same(proto: "services"),
    6: .same(proto: "authentications"),
  ]

  mutating func decodeMessage<D: SwiftProtobuf.Decoder>(decoder: inout D) throws {
    while let fieldNumber = try decoder.nextFieldNumber() {
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every case branch when no optimizations are
      // enabled. https://github.com/apple/swift-protobuf/issues/1034
      switch fieldNumber {
      case 1: try { try decoder.decodeSingularStringField(value: &self.id) }()
      case 2: try { try decoder.decodeSingularStringField(value: &self.controller) }()
      case 3: try { try decoder.decodeRepeatedMessageField(value: &self.verificationMethods) }()
      case 4: try { try decoder.decodeSingularMessageField(value: &self._signature) }()
      case 5: try { try decoder.decodeRepeatedMessageField(value: &self.services) }()
      case 6: try { try decoder.decodeRepeatedStringField(value: &self.authentications) }()
      default: break
      }
    }
  }

  func traverse<V: SwiftProtobuf.Visitor>(visitor: inout V) throws {
    // The use of inline closures is to circumvent an issue where the compiler
    // allocates stack space for every if/case branch local when no optimizations
    // are enabled. https://github.com/apple/swift-protobuf/issues/1034 and
    // https://github.com/apple/swift-protobuf/issues/1182
    if !self.id.isEmpty {
      try visitor.visitSingularStringField(value: self.id, fieldNumber: 1)
    }
    if !self.controller.isEmpty {
      try visitor.visitSingularStringField(value: self.controller, fieldNumber: 2)
    }
    if !self.verificationMethods.isEmpty {
      try visitor.visitRepeatedMessageField(value: self.verificationMethods, fieldNumber: 3)
    }
    try { if let v = self._signature {
      try visitor.visitSingularMessageField(value: v, fieldNumber: 4)
    } }()
    if !self.services.isEmpty {
      try visitor.visitRepeatedMessageField(value: self.services, fieldNumber: 5)
    }
    if !self.authentications.isEmpty {
      try visitor.visitRepeatedStringField(value: self.authentications, fieldNumber: 6)
    }
    try unknownFields.traverse(visitor: &visitor)
  }

  static func ==(lhs: Document_Document, rhs: Document_Document) -> Bool {
    if lhs.id != rhs.id {return false}
    if lhs.controller != rhs.controller {return false}
    if lhs.verificationMethods != rhs.verificationMethods {return false}
    if lhs._signature != rhs._signature {return false}
    if lhs.services != rhs.services {return false}
    if lhs.authentications != rhs.authentications {return false}
    if lhs.unknownFields != rhs.unknownFields {return false}
    return true
  }
}
