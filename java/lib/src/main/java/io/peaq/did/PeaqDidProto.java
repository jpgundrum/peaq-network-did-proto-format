// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: did_document_format.proto

package io.peaq.did;

public final class PeaqDidProto {
  private PeaqDidProto() {}
  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistryLite registry) {
  }

  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistry registry) {
    registerAllExtensions(
        (com.google.protobuf.ExtensionRegistryLite) registry);
  }
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_document_VerificationMethod_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_document_VerificationMethod_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_document_Signature_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_document_Signature_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_document_Service_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_document_Service_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_document_Document_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_document_Document_fieldAccessorTable;

  public static com.google.protobuf.Descriptors.FileDescriptor
      getDescriptor() {
    return descriptor;
  }
  private static  com.google.protobuf.Descriptors.FileDescriptor
      descriptor;
  static {
    java.lang.String[] descriptorData = {
      "\n\031did_document_format.proto\022\010document\"z\n" +
      "\022VerificationMethod\022\n\n\002id\030\001 \001(\t\022(\n\004type\030" +
      "\002 \001(\0162\032.document.VerificationType\022\022\n\ncon" +
      "troller\030\003 \001(\t\022\032\n\022publicKeyMultibase\030\004 \001(" +
      "\t\"S\n\tSignature\022(\n\004type\030\001 \001(\0162\032.document." +
      "VerificationType\022\016\n\006issuer\030\002 \001(\t\022\014\n\004hash" +
      "\030\003 \001(\t\"J\n\007Service\022\n\n\002id\030\001 \001(\t\022\014\n\004type\030\002 " +
      "\001(\t\022\027\n\017serviceEndpoint\030\003 \001(\t\022\014\n\004data\030\004 \001" +
      "(\t\"\313\001\n\010Document\022\n\n\002id\030\001 \001(\t\022\022\n\ncontrolle" +
      "r\030\002 \001(\t\0229\n\023verificationMethods\030\003 \003(\0132\034.d" +
      "ocument.VerificationMethod\022&\n\tsignature\030" +
      "\004 \001(\0132\023.document.Signature\022#\n\010services\030\005" +
      " \003(\0132\021.document.Service\022\027\n\017authenticatio" +
      "ns\030\006 \003(\t*R\n\020VerificationType\022\036\n\032Ed25519V" +
      "erificationKey2020\020\000\022\036\n\032Sr25519Verificat" +
      "ionKey2020\020\001Bl\n\013io.peaq.didB\014PeaqDidProt" +
      "oP\001ZMgithub.com/peaqnetwork/peaq-network" +
      "-did-proto-format/golang/document;docume" +
      "ntb\006proto3"
    };
    descriptor = com.google.protobuf.Descriptors.FileDescriptor
      .internalBuildGeneratedFileFrom(descriptorData,
        new com.google.protobuf.Descriptors.FileDescriptor[] {
        });
    internal_static_document_VerificationMethod_descriptor =
      getDescriptor().getMessageTypes().get(0);
    internal_static_document_VerificationMethod_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_document_VerificationMethod_descriptor,
        new java.lang.String[] { "Id", "Type", "Controller", "PublicKeyMultibase", });
    internal_static_document_Signature_descriptor =
      getDescriptor().getMessageTypes().get(1);
    internal_static_document_Signature_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_document_Signature_descriptor,
        new java.lang.String[] { "Type", "Issuer", "Hash", });
    internal_static_document_Service_descriptor =
      getDescriptor().getMessageTypes().get(2);
    internal_static_document_Service_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_document_Service_descriptor,
        new java.lang.String[] { "Id", "Type", "ServiceEndpoint", "Data", });
    internal_static_document_Document_descriptor =
      getDescriptor().getMessageTypes().get(3);
    internal_static_document_Document_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_document_Document_descriptor,
        new java.lang.String[] { "Id", "Controller", "VerificationMethods", "Signature", "Services", "Authentications", });
  }

  // @@protoc_insertion_point(outer_class_scope)
}
